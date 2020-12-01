use std::path::PathBuf;

use emver::{Version, VersionRange};
use linear_map::LinearMap;

use crate::dependencies::Dependencies;
use crate::tor::{HiddenServiceConfig, HiddenServiceMode, HiddenServiceVersion, PortMapping};
use crate::util::{ByteSize, ByteUnit};

pub type ManifestLatest = ManifestV1;

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
        mount: PathBuf,
        #[serde(default)]
        shm_size: Option<ByteSize>,
    },
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
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
    pub instructions: bool,
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
    V1(ManifestV1),
}
impl Manifest {
    pub fn into_latest(self) -> ManifestLatest {
        match self {
            Manifest::V0(m) => ManifestV1 {
                id: m.id,
                version: m.version,
                title: m.title,
                description: m.description,
                release_notes: m.release_notes,
                instructions: m.has_instructions,
                os_version_required: m.os_version_required,
                os_version_recommended: m.os_version_recommended,
                network_interfaces: NetworkInterfaces(linear_map::linear_map! {
                    "default" => NetworkInterface {
                        name: "Default".to_owned(),
                        ports: m.ports.iter().map(|p| p.internal).collect(),
                        hidden_service: if m.ports.is_empty() { None } else { Some(HiddenServiceConfig {
                            version: m.hidden_service_version,
                            mode: HiddenServiceMode::Anonymous,
                            port_mapping: m.ports.iter().filter_map(|p| if p.internal == p.tor {
                                None
                            } else {
                                Some((p.internal, p.tor))
                            }).collect()
                        })},
                    }
                }),
                bundle_info: BundleInfo::Docker {
                    image_format: m.image,
                    mount: m.mount,
                    shm_size: m.shm_size_mb.map(|shm_size_mb| ByteSize {
                        size: shm_size_mb,
                        units: ByteUnit::M,
                    }),
                },
                public: m.public,
                shared: m.shared,
                assets: m.assets,
                dependencies: m.dependencies,
                extra: LinearMap::new(),
            },
            Manifest::V1(m) => m,
        }
    }
}
