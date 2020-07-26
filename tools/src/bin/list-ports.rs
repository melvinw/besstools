extern crate libbess;

use libbess::bess_control_client::BessControlClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bessctl = BessControlClient::connect("http://127.0.0.1:10514").await?;
    let req = libbess::ListTcsRequest{};
    let resp = bessctl.list_tcs(req).await?;
    println!("{:?}", resp);
    Ok(())
}
