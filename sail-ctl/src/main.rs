use clap::{Parser, Subcommand};
use bollard::Docker;
use kube::Client;
use std::error::Error;
use regex::Regex;


#[derive(Parser)]
#[command(name = "sail-ctl")]
#[command(about = "CLI tool to manage SailOverRustFS stack", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect containers (rustsf, marquez) to the kind network
    Connect,
    /// Sync RustFS IP with Kubernetes manifests
    Sync,
    /// Show status of the stack (Docker & K8s)
    Status,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Connect => {
            println!("Connecting containers to kind network...");
            connect_containers().await?;
        }
        Commands::Sync => {
            println!("Syncing RustFS IP with K8s manifests...");
            sync_manifests().await?;
        }
        Commands::Status => {
            println!("Checking stack status...");
            check_status().await?;
        }
    }

    Ok(())
}

async fn connect_containers() -> Result<(), Box<dyn Error>> {
    let docker = Docker::connect_with_local_defaults()?;
    let network_name = "kind";
    let containers = vec!["rustsf", "marquez"];

    // 1. Check if network exists
    let networks = docker.list_networks::<String>(None).await?;
    let kind_net = networks.iter().find(|n| n.name.as_deref() == Some(network_name));

    if let Some(net) = kind_net {
        let net_id = net.id.as_ref().unwrap();
        println!("Found network: {} ({})", network_name, net_id);

        for container_name in containers {
            match docker.inspect_container(container_name, None).await {
                Ok(container) => {
                    let settings = container.network_settings.unwrap();
                    let networks = settings.networks.unwrap();
                    
                    if networks.contains_key(network_name) {
                        println!("Container '{}' is already connected to '{}'.", container_name, network_name);
                    } else {
                        println!("Connecting '{}' to '{}'...", container_name, network_name);
                        docker.connect_network(net_id, bollard::network::ConnectNetworkOptions {
                            container: container_name,
                            ..Default::default()
                        }).await?;
                        println!("Successfully connected '{}'.", container_name);
                    }

                    // Get and print IP
                    let updated_container = docker.inspect_container(container_name, None).await?;
                    if let Some(nets) = updated_container.network_settings.and_then(|ns| ns.networks) {
                        if let Some(endpoint) = nets.get(network_name) {
                            println!("  IP in {}: {}", network_name, endpoint.ip_address.as_deref().unwrap_or("Unknown"));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Warning: Could not find or inspect container '{}': {}", container_name, e);
                }
            }
        }
    } else {
        println!("Error: Network '{}' not found. Is Kind running?", network_name);
    }

    Ok(())
}
