use std::fs;
use std::path::PathBuf;
use leptos::prelude::{Get, Set};
use serde_json::json;
use crate::stores::signal::Signal;

struct ConfigStore {
    base_uri: Signal<Option<String>>,
    user: Signal<Option<String>>,
    api_key: Signal<Option<String>>,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("failed to load config, {}", .0)]
    LoadingFailed(String),

    #[error("failed to save config, {}", .0)]
    SavingFailed(String),

    #[error("invalid setting value in the following keys: {}", .0.join(", "))]
    InvalidSettingValue(Vec<&'static str>),
}

impl ConfigStore {
    fn default() -> Self {
        ConfigStore {
            base_uri: Signal::new(None),
            user: Signal::new(None),
            api_key: Signal::new(None),
        }
    }

    async fn load(&mut self, settings_file: &PathBuf) -> Result<(), Error> {
        if !settings_file.exists() {
            self.base_uri.write.set(None);
            self.user.write.set(None);
            self.api_key.write.set(None);
            return Ok(());
        }

        let content = fs::read_to_string(&settings_file)
            .map_err(|e| Error::LoadingFailed(
                format!("could not read settings file {}: {}", &settings_file.display(), e.to_string())
            ))?;

        let value = serde_json::from_str::<serde_json::Value>(content.as_str())
            .map_err(|e| Error::LoadingFailed(
                format!("malformed json in {}: {}", &settings_file.display(), e.to_string())
            ))?;

        if !value.is_object() {
            self.base_uri.write.set(None);
            self.user.write.set(None);
            self.api_key.write.set(None);
            return Err(Error::LoadingFailed(format!("malformed file {}", &settings_file.display())));
        }

        let mut malformed_properties: Vec<&'static str> = vec![];

        let base_uri = value.get("base_uri").and_then(|v| v.as_str());
        let user = value.get("user").and_then(|v| v.as_str());
        let api_key = value.get("api_key").and_then(|v| v.as_str());

        if let Some(s) = base_uri {
            self.base_uri.write.set(Some(s.to_string()));
        } else {
            malformed_properties.push("base_uri");
        }

        if let Some(s) = user {
            self.user.write.set(Some(s.to_string()));
        } else {
            malformed_properties.push("user");
        }

        if let Some(s) = api_key {
            self.api_key.write.set(Some(s.to_string()));
        } else {
            malformed_properties.push("api_key");
        }

        if !malformed_properties.is_empty() {
            return Err(Error::InvalidSettingValue(malformed_properties));
        }

        Ok(())
    }

    async fn save(&self, settings_file: &PathBuf) -> Result<(), Error> {
        let settings = json!({
            "base_uri": self.base_uri.read.get(),
            "user": self.user.read.get(),
            "api_key": self.api_key.read.get(),
        });

        let settings = serde_json::to_string_pretty(&settings)
            .map_err(|e| Error::SavingFailed(
                format!("could not encode json: {}", e.to_string())
            ))?;

        fs::write(&settings_file, settings)
            .map_err(|e| Error::SavingFailed(
                format!("could not write to {}: {}", &settings_file.display(), e.to_string())
            ))
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn test_resource_dir() -> PathBuf {
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .parent()
            .unwrap()
            .join(file!())
            .parent()
            .unwrap()
            .join("../../../test/resource")
    }

    #[tokio::test]
    async fn test_load() -> Result<(), Error> {
        let mut config = ConfigStore::default();

        let test_file = test_resource_dir().join("testconfig.full.json");

        let _ = config.load(&test_file).await?;

        let base_uri = config.base_uri.read.get();
        assert_eq!(base_uri, Some("a base uri".to_string()));

        let user = config.user.read.get();
        assert_eq!(user, Some("a user".to_string()));

        let api_key = config.api_key.read.get();
        assert_eq!(api_key, Some("an api key".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_load_when_file_does_not_exist() -> Result<(), Error> {
        let mut config = ConfigStore::default();

        let test_file = test_resource_dir().join("nonexistentfile.json");

        let _ = config.load(&test_file).await?;

        assert!(config.base_uri.read.get().is_none());
        assert!(config.user.read.get().is_none());
        assert!(config.api_key.read.get().is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_load_when_malformed_json() -> Result<(), Error> {
        let mut config = ConfigStore::default();

        let test_file = test_resource_dir().join("testconfig.malformed.json");

        let r = config.load(&test_file).await;
        assert!(matches!(r, Err(Error::LoadingFailed(_))));

        assert!(config.base_uri.read.get().is_none());
        assert!(config.user.read.get().is_none());
        assert!(config.api_key.read.get().is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_load_when_contents_not_an_object() -> Result<(), Error> {
        let mut config = ConfigStore::default();

        let test_file = test_resource_dir().join("testconfig.array.json");

        let r = config.load(&test_file).await;
        assert!(matches!(r, Err(Error::LoadingFailed(_))));

        assert!(config.base_uri.read.get().is_none());
        assert!(config.user.read.get().is_none());
        assert!(config.api_key.read.get().is_none());

        Ok(())
    }


    #[tokio::test]
    async fn test_load_when_properties_malformed() -> Result<(), Error> {
        let mut config = ConfigStore::default();

        let test_file = test_resource_dir().join("testconfig.malformed_property.json");

        let r = config.load(&test_file).await;
        assert!(
            matches!(
                r,
                Err(Error::InvalidSettingValue(p)) if p.contains(&"user")
                    && p.contains(&"api_key")
                    && !p.contains(&"base_uri")
            )
        );

        assert_eq!(config.base_uri.read.get(), Some("a base uri".to_string()));
        assert!(config.user.read.get().is_none());
        assert!(config.api_key.read.get().is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_save() -> Result<(), Error> {
        let config = ConfigStore::default();

        let output_file = test_resource_dir().parent().unwrap().join("gen/testconfig.save.json");

        config.api_key.write.set(Some("test api key".to_string()));
        config.base_uri.write.set(Some("test base uri".to_string()));
        config.user.write.set(Some("test user".to_string()));

        config.save(&output_file).await?;

        assert!(output_file.exists());

        let expected_contents = "{
  \"api_key\": \"test api key\",
  \"base_uri\": \"test base uri\",
  \"user\": \"test user\"
}";
        let actual_contents = fs::read_to_string(&output_file).unwrap();

        assert_eq!(expected_contents, actual_contents);

        Ok(())
    }
}
