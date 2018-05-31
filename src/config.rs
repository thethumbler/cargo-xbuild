use serde_json;
use cargo_metadata;

#[derive(Debug, Deserialize, Hash)]
pub struct Config {
    pub memcpy: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            memcpy: true,
        }
    }
}

impl Config {
    pub fn from_metadata(metadata: &cargo_metadata::Metadata) -> Result<Config, serde_json::Error> {
        let package_metadata = metadata.packages.first().map(|p| &p.metadata);
        let crate_metadata = package_metadata.as_ref().and_then(|m| m.get("cargo-xbuild"));
        match crate_metadata {
            Some(json) => serde_json::from_value(json.clone()),
            None => Ok(Config::default())
        }
    }
}
