use crate::{HueError, HueError::DiscoveryError};
use mdns::discover_mdns_sd;
use serde_json::{Map, Value};
use std::net::IpAddr;

mod mdns;

// Define the service name for hue bridge
const DNS_SD_HUE_SERVICE_NAME: &str = "_hue._tcp.local";

pub(crate) struct BridgeInfo {
    pub(crate) ip: IpAddr,
    pub(crate) id: String,
}

// As Per instructions at
// https://developers.meethue.com/develop/application-design-guidance/hue-bridge-discovery/
pub async fn discover_hue_bridge() -> Result<BridgeInfo, HueError> {
    let bridge = discover_mdns_sd(DNS_SD_HUE_SERVICE_NAME).await;
    match bridge {
        Ok(bridge_info) => {
            log::info!(
                "Discovered bridge using mDNS. IP: {}, ID: {}",
                bridge_info.ip,
                bridge_info.id
            );
            Ok(bridge_info)
        }
        Err(mdns_error) => {
            log::debug!(
                "Error in mDNS discovery: {}, falling back to n-upnp",
                mdns_error
            );
            let n_upnp_result = discover_hue_bridge_n_upnp().await;
            match n_upnp_result {
                Ok(bridge_info) => {
                    log::info!(
                        "Discovered bridge using n-upnp. IP: {}, ID: {}",
                        bridge_info.ip,
                        bridge_info.id
                    );
                    Ok(bridge_info)
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

pub async fn discover_hue_bridge_n_upnp() -> Result<BridgeInfo, HueError> {
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

    let ip = ip
        .as_str()
        .ok_or(DiscoveryError {
            msg: "expect a string in internalipaddress".into(),
        })?
        .parse()?;

    let id = object
        .get("id")
        .ok_or(DiscoveryError {
            msg: "Expected id".into(),
        })?
        .as_str()
        .ok_or(DiscoveryError {
            msg: "expect a string in id".into(),
        })?
        .to_string();

    Ok(BridgeInfo { ip, id })
}

pub async fn discover_hue_bridge_mdns() -> Result<BridgeInfo, HueError> {
    discover_mdns_sd(DNS_SD_HUE_SERVICE_NAME).await
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
