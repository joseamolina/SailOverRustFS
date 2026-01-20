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


async fn sync_manifests() -> Result<(), Box<dyn Error>> {
    let docker = Docker::connect_with_local_defaults()?;
    let network_name = "kind";
    let container_name = "rustsf";

    let container = docker.inspect_container(container_name, None).await?;
    let ip = container.network_settings
        .and_then(|ns| ns.networks)
        .and_then(|nets| nets.get(network_name).cloned())
        .and_then(|endpoint| endpoint.ip_address);

    if let Some(ip_addr) = ip {
        println!("Detected RustFS IP: {}", ip_addr);
        println!("Updating k8s/sail.yml...");

        let manifest_path = "../k8s/sail.yml";
        let content = std::fs::read_to_string(manifest_path)?;

        // Simple string replacement for now, as YAML parsing can be complex for multi-document files
        // and we want to preserve comments/structure.
        // We look for patterns like: value: "http://172.23.0.3:9000"
        let re_endpoint = Regex::new(r#"http://\d+\.\d+\.\d+\.\d+:9000"#)?;
        let new_endpoint = format!("http://{}:9000", ip_addr);
        let updated_content = re_endpoint.replace_all(&content, new_endpoint.as_str());

        std::fs::write(manifest_path, updated_content.into_owned())?;
        println!("Manifest updated successfully.");
    } else {
        println!("Error: Could not find IP for '{}' in network '{}'. Run 'connect' first.", container_name, network_name);
    }

    Ok(())
}

async fn check_status() -> Result<(), Box<dyn Error>> {
    let docker = Docker::connect_with_local_defaults()?;
    let k8s = Client::try_default().await?;

    println!("--- Docker Status ---");
    let version = docker.version().await?;
    println!("Docker Version: {}", version.version.unwrap_or_else(|| "Unknown".to_string()));

    let containers = docker.list_containers::<String>(None).await?;
    println!("Running Containers: {}", containers.len());
    for c in containers {
        println!("  - {} ({})", c.names.unwrap_or_default().join(", "), c.image.unwrap_or_default());
    }

    println!("\n--- Kubernetes Status ---");
    let namespaces: kube::api::Api<k8s_openapi::api::core::v1::Namespace> = kube::Api::all(k8s);
    let lp = kube::api::ListParams::default();
    let ns_list = namespaces.list(&lp).await?;
    println!("Connected to K8s. Found {} namespaces.", ns_list.items.len());

    Ok(())
}

