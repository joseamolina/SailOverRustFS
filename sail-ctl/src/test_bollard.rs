use bollard::network::ConnectNetworkOptions;
fn main() {
    let _ = ConnectNetworkOptions {
        container: "test".to_string(),
        ..Default::default()
    };
}
