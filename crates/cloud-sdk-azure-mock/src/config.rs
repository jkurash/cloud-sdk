use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Root configuration for the Azure mock server.
/// Loaded from an `azure-mock.toml` file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureMockConfig {
    pub server: ServerConfig,
    #[serde(default)]
    pub subscriptions: HashMap<String, SubscriptionConfig>,
}

/// Server bind address, port, and behavior settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_bind")]
    pub bind: String,
    #[serde(default = "default_port")]
    pub port: u16,
    /// Response delay in milliseconds applied to every endpoint.
    /// Simulates network latency for testing async client behavior.
    /// Default: 0 (no delay).
    #[serde(default)]
    pub delay_ms: u64,
}

/// Configuration for a single subscription and its seed data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionConfig {
    pub id: String,
    pub display_name: String,
    pub tenant_id: String,
    #[serde(default = "default_subscription_state")]
    pub state: String,
    #[serde(default)]
    pub resource_groups: Vec<ResourceGroupSeed>,
}

/// Seed data for a pre-populated resource group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceGroupSeed {
    pub name: String,
    pub location: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

fn default_bind() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_subscription_state() -> String {
    "Enabled".to_string()
}

impl AzureMockConfig {
    /// Load configuration from a TOML file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let contents = std::fs::read_to_string(path.as_ref()).map_err(|e| ConfigError::Io {
            path: path.as_ref().display().to_string(),
            source: e,
        })?;
        Self::from_toml(&contents)
    }

    /// Parse configuration from a TOML string.
    pub fn from_toml(s: &str) -> Result<Self, ConfigError> {
        let config: Self = toml::from_str(s).map_err(ConfigError::Parse)?;
        config.validate()?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), ConfigError> {
        if self.subscriptions.is_empty() {
            return Err(ConfigError::Validation(
                "at least one subscription must be defined".to_string(),
            ));
        }
        for (key, sub) in &self.subscriptions {
            if sub.id.is_empty() {
                return Err(ConfigError::Validation(format!(
                    "subscription '{key}' has an empty id"
                )));
            }
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("failed to read config file '{path}': {source}")]
    Io {
        path: String,
        source: std::io::Error,
    },

    #[error("failed to parse config: {0}")]
    Parse(#[from] toml::de::Error),

    #[error("invalid config: {0}")]
    Validation(String),
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind: default_bind(),
            port: default_port(),
            delay_ms: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_full_config() {
        let toml = r#"
[server]
bind = "0.0.0.0"
port = 9090

[subscriptions.default]
id = "aaaa-bbbb-cccc"
display_name = "Test Sub"
tenant_id = "tttt-uuuu"
state = "Enabled"

[[subscriptions.default.resource_groups]]
name = "dev-rg"
location = "eastus"

[[subscriptions.default.resource_groups]]
name = "staging-rg"
location = "westus2"
tags = { env = "staging" }
"#;
        let config = AzureMockConfig::from_toml(toml).unwrap();
        assert_eq!(config.server.bind, "0.0.0.0");
        assert_eq!(config.server.port, 9090);

        let sub = &config.subscriptions["default"];
        assert_eq!(sub.id, "aaaa-bbbb-cccc");
        assert_eq!(sub.display_name, "Test Sub");
        assert_eq!(sub.resource_groups.len(), 2);
        assert_eq!(sub.resource_groups[0].name, "dev-rg");
        assert_eq!(sub.resource_groups[1].tags["env"], "staging");
    }

    #[test]
    fn parse_minimal_config() {
        let toml = r#"
[server]

[subscriptions.main]
id = "sub-1"
display_name = "My Sub"
tenant_id = "tenant-1"
"#;
        let config = AzureMockConfig::from_toml(toml).unwrap();
        assert_eq!(config.server.bind, "127.0.0.1");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.subscriptions["main"].state, "Enabled");
        assert!(config.subscriptions["main"].resource_groups.is_empty());
    }

    #[test]
    fn empty_subscriptions_fails_validation() {
        let toml = r#"
[server]
"#;
        let result = AzureMockConfig::from_toml(toml);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("at least one subscription")
        );
    }

    #[test]
    fn empty_subscription_id_fails_validation() {
        let toml = r#"
[server]

[subscriptions.bad]
id = ""
display_name = "Bad"
tenant_id = "t"
"#;
        let result = AzureMockConfig::from_toml(toml);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty id"));
    }
}
