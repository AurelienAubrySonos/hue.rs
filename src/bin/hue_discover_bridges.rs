extern crate hueclient;
use hueclient::Bridge;

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    #[cfg(feature = "pretty_env_logger")]
    pretty_env_logger::init_custom_env("HUE_LOG");

    let bridges = Bridge::discover().await;
    println!("Hue bridges found: {:?}", bridges);
}
