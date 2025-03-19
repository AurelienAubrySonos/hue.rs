use crate::{HueError, HueError::DiscoveryError};
use mdns::discover_mdns_sd;
use serde_json::{Map, Value};
use std::net::IpAddr;

mod mdns;

// Define the service name for hue bridge
const DNS_SD_HUE_SERVICE_NAME: &str = "_hue._tcp.local";

// As Per instrucitons at
// https://developers.meethue.com/develop/application-design-guidance/hue-bridge-discovery/
pub async fn discover_hue_bridge() -> Result<IpAddr, HueError> {
    let bridge = discover_mdns_sd(DNS_SD_HUE_SERVICE_NAME).await;
    match bridge {
        Ok(bridge_ip) => {
            log::info!("discovered bridge at {bridge_ip} using mDNS");
            Ok(bridge_ip)
        }
        Err(mdns_error) => {
            log::debug!(
                "Error in mDNS discovery: {}, falling back to n-upnp",
                mdns_error
            );
            let n_upnp_result = discover_hue_bridge_n_upnp().await;
            match n_upnp_result {
                Ok(bridge_ip) => {
                    log::info!("discovered bridge at {bridge_ip} using n-upnp");
                    Ok(bridge_ip)
                }
                Err(nupnp_error) => {
                    log::debug!("Failed to discover bridge using or n-upnp: {nupnp_error}");
                    Err(DiscoveryError {
                        msg: "Could not discover bridge".into(),
                    })?
                }
            }
        }
    }
}

pub async fn discover_hue_bridge_n_upnp() -> Result<IpAddr, HueError> {
    let objects: Vec<Map<String, Value>> = reqwest::get("https://discovery.meethue.com/")
        .await?
        .json()
        .await?;

    if objects.is_empty() {
        Err(DiscoveryError {
            msg: "expected non-empty array".into(),
        })?
    }
    let object = &objects[0];

    let ip = object.get("internalipaddress").ok_or(DiscoveryError {
        msg: "Expected internalipaddress".into(),
    })?;
    Ok(ip
        .as_str()
        .ok_or(DiscoveryError {
            msg: "expect a string in internalipaddress".into(),
        })?
        .parse()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_discover_hue_bridge() {
        let ip = discover_hue_bridge().await;
        assert!(ip.is_ok());
        let ip = ip.unwrap();
        assert_eq!(ip.to_string(), "192.168.1.149");
    }
}
