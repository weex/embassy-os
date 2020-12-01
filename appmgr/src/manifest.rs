use std::path::PathBuf;

use emver::{Version, VersionRange};
use linear_map::LinearMap;

use crate::dependencies::Dependencies;
use crate::tor::{HiddenServiceMode, HiddenServiceVersion, PortMapping};

pub type ManifestLatest = ManifestV0;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Description {
    pub short: String,
    pub long: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum ImageConfig {
    Tar,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Asset {
    pub src: PathBuf,
    pub dst: PathBuf,
    pub overwrite: bool,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ManifestV0 {
    pub id: String,
    pub version: Version,
    pub title: String,
    pub description: Description,
    pub release_notes: String,
    #[serde(default)]
    pub has_instructions: bool,
    #[serde(default = "VersionRange::any")]
    pub os_version_required: VersionRange,
    #[serde(default = "VersionRange::any")]
    pub os_version_recommended: VersionRange,
    pub ports: Vec<PortMapping>,
    pub image: ImageConfig,
    #[serde(default)]
    pub shm_size_mb: Option<usize>,
    pub mount: PathBuf,
    #[serde(default)]
    pub public: Option<PathBuf>,
    #[serde(default)]
    pub shared: Option<PathBuf>,
    #[serde(default)]
    pub assets: Vec<Asset>,
    #[serde(default)]
    pub hidden_service_version: HiddenServiceVersion,
    #[serde(default)]
    pub dependencies: Dependencies,
    #[serde(flatten)]
    pub extra: LinearMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum BundleInfo {
    #[serde(rename_all = "kebab-case")]
    Docker {
        image_format: ImageConfig,
        #[serde(default)]
        shm_size_mb: Option<usize>,
        mount: PathBuf,
    },
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct HiddenServiceConfig {
    pub version: HiddenServiceVersion,
    pub mode: HiddenServiceMode,
    pub port_mapping: LinearMap<u16, u16>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NetworkInterfaces(pub LinearMap<String, NetworkInterface>);
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct NetworkInterface {
    pub name: String,
    pub ports: Vec<u16>,
    pub hidden_service: Option<HiddenServiceConfig>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ManifestV1 {
    pub id: String,
    pub version: Version,
    pub title: String,
    pub description: Description,
    pub release_notes: String,
    pub has_instructions: bool,
    #[serde(default = "VersionRange::any")]
    pub os_version_required: VersionRange,
    #[serde(default = "VersionRange::any")]
    pub os_version_recommended: VersionRange,
    pub network_interfaces: NetworkInterfaces,
    pub bundle_info: BundleInfo,
    pub public: Option<PathBuf>,
    pub shared: Option<PathBuf>,
    pub assets: Vec<Asset>,
    pub dependencies: Dependencies,
    #[serde(flatten)]
    pub extra: LinearMap<String, serde_yaml::Value>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "compat")]
#[serde(rename_all = "lowercase")]
pub enum Manifest {
    V0(ManifestV0),
}
impl Manifest {
    pub fn into_latest(self) -> ManifestLatest {
        match self {
            Manifest::V0(m) => m,
        }
    }
}
