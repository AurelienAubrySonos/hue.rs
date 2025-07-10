use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ResourceIdentifier {
    pub rid: String,
    pub rtype: String,
}

type Owner = ResourceIdentifier;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct LightMetadata {
    pub name: Option<String>,
    pub archetype: Option<String>,
    pub fixed_mired: Option<u16>,
    pub function: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct On {
    pub on: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dimming {
    pub brightness: f32,
    pub min_dim_level: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ColorTemperature {
    pub mirek: Option<u16>,
    pub mirek_valid: Option<bool>,
    pub mirek_schema: Option<MirekSchema>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct MirekSchema {
    pub mirek_minimum: u16,
    pub mirek_maximum: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Color {
    pub xy: Option<XY>,
    pub gamut: Option<Gamut>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct XY {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Gamut {
    pub red: XY,
    pub green: XY,
    pub blue: XY,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Light {
    pub id: String,
    pub id_v1: Option<String>,
    pub owner: Option<Owner>,
    pub metadata: Option<LightMetadata>,
    pub product_data: Option<LightMetadata>,
    pub service_id: Option<u32>,
    pub on: Option<On>,
    pub dimming: Option<Dimming>,
    pub color_temperature: Option<ColorTemperature>,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct SceneMetadata {
    pub name: Option<String>,
}

pub type Group = ResourceIdentifier;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct SceneStatus {
    pub active: Option<SceneStatusActive>,
    pub last_recall: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SceneStatusActive {
    Inactive,
    Static,
    DynamicPalette,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Scene {
    pub id: String,
    pub id_v1: Option<String>,
    pub metadata: Option<SceneMetadata>,
    pub group: Option<Group>,
    pub status: Option<SceneStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SmartSceneActive {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct SmartScene {
    pub id: String,
    pub id_v1: Option<String>,
    pub metadata: Option<SceneMetadata>,
    pub group: Option<Group>,
    pub state: Option<SmartSceneActive>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Metadata {
    pub name: Option<String>,
    pub archetype: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Room {
    pub id: String,
    pub id_v1: Option<String>,
    pub children: Option<Vec<ResourceIdentifier>>,
    pub services: Option<Vec<ResourceIdentifier>>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Zone {
    pub id: String,
    pub id_v1: Option<String>,
    pub children: Option<Vec<ResourceIdentifier>>,
    pub services: Option<Vec<ResourceIdentifier>>,
    pub metadata: Option<Metadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct BridgeHome {
    pub id: String,
    pub id_v1: Option<String>,
    pub children: Option<Vec<ResourceIdentifier>>,
    pub services: Option<Vec<ResourceIdentifier>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupedLight {
    pub id: String,
    pub id_v1: Option<String>,
    pub owner: Option<Owner>,
    pub on: Option<On>,
    pub dimming: Option<Dimming>,
    pub color_temperature: Option<ColorTemperature>,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ProductData {
    pub model_id: Option<String>,
    pub manufacturer_name: Option<String>,
    pub product_name: Option<String>,
    pub product_archetype: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct DeviceMetadata {
    pub name: Option<String>,
    pub archetype: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Device {
    pub id: String,
    pub id_v1: Option<String>,
    pub product_data: Option<ProductData>,
    pub metadata: Option<DeviceMetadata>,
    pub services: Option<Vec<ResourceIdentifier>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Bridge {
    pub id: String,
    pub id_v1: Option<String>,
    pub owner: Option<Owner>,
    pub bridge_id: Option<String>,
}
