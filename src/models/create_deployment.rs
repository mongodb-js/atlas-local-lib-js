use crate::models::list_deployments::{CreationSource, MongoDBPortBinding};
use atlas_local::models::MongoDBVersion;
use napi_derive::napi;
use std::time::Duration;

#[napi(object)]
pub struct CreateDeploymentOptions {
  // Identifiers
  pub name: Option<String>,

  // Image details
  pub image: Option<String>,
  pub skip_pull_image: Option<bool>,
  pub mongodb_version: Option<String>,

  // Creation Options
  pub wait_until_healthy: Option<bool>,
  pub wait_until_healthy_timeout: Option<u32>,
  pub creation_source: Option<CreationSource>,

  // Initial database configuration
  pub local_seed_location: Option<String>,
  pub load_sample_data: Option<bool>,
  pub mongodb_initdb_database: Option<String>,
  pub mongodb_initdb_root_password_file: Option<String>,
  pub mongodb_initdb_root_password: Option<String>,
  pub mongodb_initdb_root_username_file: Option<String>,
  pub mongodb_initdb_root_username: Option<String>,
  pub voyage_api_key: Option<String>,

  // Logging
  pub mongot_log_file: Option<String>,
  pub runner_log_file: Option<String>,

  // Telemetry
  pub do_not_track: Option<bool>,
  pub telemetry_base_url: Option<String>,

  // Port configuration
  pub mongodb_port_binding: Option<MongoDBPortBinding>,
}

impl TryFrom<CreateDeploymentOptions> for atlas_local::models::CreateDeploymentOptions {
  type Error = anyhow::Error;

  fn try_from(source: CreateDeploymentOptions) -> Result<Self, Self::Error> {
    Ok(Self {
      name: source.name,
      image: source.image,
      skip_pull_image: source.skip_pull_image,
      mongodb_version: source
        .mongodb_version
        .as_deref()
        .map(MongoDBVersion::try_from)
        .transpose()
        .map_err(anyhow::Error::msg)?,
      wait_until_healthy: source.wait_until_healthy,
      wait_until_healthy_timeout: source
        .wait_until_healthy_timeout
        .map(|timeout| Duration::from_secs(timeout as u64)),
      creation_source: source
        .creation_source
        .map(atlas_local::models::CreationSource::from),
      local_seed_location: source.local_seed_location,
      load_sample_data: source.load_sample_data,
      mongodb_initdb_database: source.mongodb_initdb_database,
      mongodb_initdb_root_password_file: source.mongodb_initdb_root_password_file,
      mongodb_initdb_root_password: source.mongodb_initdb_root_password,
      mongodb_initdb_root_username_file: source.mongodb_initdb_root_username_file,
      mongodb_initdb_root_username: source.mongodb_initdb_root_username,
      voyage_api_key: source.voyage_api_key,
      mongot_log_file: source.mongot_log_file,
      runner_log_file: source.runner_log_file,
      do_not_track: source.do_not_track,
      telemetry_base_url: source.telemetry_base_url,
      mongodb_port_binding: source
        .mongodb_port_binding
        .map(atlas_local::models::MongoDBPortBinding::from),
    })
  }
}

#[cfg(test)]
mod tests {
  use atlas_local::models::MongoDBVersionMajorMinorPatch;

  use crate::models::list_deployments::{BindingType, CreationSourceType};

  use super::*;

  #[test]
  fn test_lib_create_deployment_options_from_create_deployment_options() {
    let create_deployment_options = CreateDeploymentOptions {
      name: Some("test_deployment".to_string()),
      image: Some("mongodb/mongodb-atlas-local".to_string()),
      skip_pull_image: Some(false),
      mongodb_version: Some("8.0.0".to_string()),
      wait_until_healthy: Some(true),
      wait_until_healthy_timeout: Some(30),
      creation_source: Some(CreationSource {
        source_type: CreationSourceType::MCPServer,
        source: "MCPSERVER".to_string(),
      }),
      local_seed_location: Some("/host/seed-data".to_string()),
      load_sample_data: Some(true),
      mongodb_initdb_database: Some("testdb".to_string()),
      mongodb_initdb_root_password_file: Some("/run/secrets/password".to_string()),
      mongodb_initdb_root_password: Some("password123".to_string()),
      mongodb_initdb_root_username_file: Some("/run/secrets/username".to_string()),
      mongodb_initdb_root_username: Some("admin".to_string()),
      voyage_api_key: Some("voyage_api_key".to_string()),
      mongot_log_file: Some("/tmp/mongot.log".to_string()),
      runner_log_file: Some("/tmp/runner.log".to_string()),
      do_not_track: Some(false),
      telemetry_base_url: Some("https://telemetry.example.com".to_string()),
      mongodb_port_binding: Some(MongoDBPortBinding {
        binding_type: BindingType::Loopback,
        ip: "127.0.0.1".to_string(),
        port: Some(27017),
      }),
    };
    let lib_create_deployment_options: atlas_local::models::CreateDeploymentOptions =
      create_deployment_options.try_into().unwrap();
    assert_eq!(
      lib_create_deployment_options.name,
      Some("test_deployment".to_string())
    );
    assert_eq!(
      lib_create_deployment_options.image,
      Some("mongodb/mongodb-atlas-local".to_string())
    );
    assert_eq!(
      lib_create_deployment_options.mongodb_version,
      Some(MongoDBVersion::MajorMinorPatch(
        MongoDBVersionMajorMinorPatch {
          major: 8,
          minor: 0,
          patch: 0,
        }
      ))
    );
    assert_eq!(lib_create_deployment_options.wait_until_healthy, Some(true));
    assert_eq!(
      lib_create_deployment_options.wait_until_healthy_timeout,
      Some(Duration::from_secs(30))
    );
    assert_eq!(
      lib_create_deployment_options.creation_source,
      Some(atlas_local::models::CreationSource::MCPServer)
    );
    assert_eq!(
      lib_create_deployment_options.local_seed_location,
      Some("/host/seed-data".to_string())
    );
    assert_eq!(lib_create_deployment_options.load_sample_data, Some(true));
    assert_eq!(
      lib_create_deployment_options.mongodb_initdb_database,
      Some("testdb".to_string())
    );
    assert_eq!(
      lib_create_deployment_options.mongodb_initdb_root_password_file,
      Some("/run/secrets/password".to_string())
    );
    assert_eq!(
      lib_create_deployment_options.mongodb_initdb_root_password,
      Some("password123".to_string())
    );
    assert_eq!(
      lib_create_deployment_options.mongodb_initdb_root_username_file,
      Some("/run/secrets/username".to_string())
    );
    assert_eq!(
      lib_create_deployment_options.mongodb_initdb_root_username,
      Some("admin".to_string())
    );
    assert_eq!(
      lib_create_deployment_options.voyage_api_key,
      Some("voyage_api_key".to_string())
    );
    assert_eq!(
      lib_create_deployment_options.mongot_log_file,
      Some("/tmp/mongot.log".to_string())
    );
    assert_eq!(
      lib_create_deployment_options.runner_log_file,
      Some("/tmp/runner.log".to_string())
    );
    assert_eq!(lib_create_deployment_options.do_not_track, Some(false));
    assert_eq!(
      lib_create_deployment_options.telemetry_base_url,
      Some("https://telemetry.example.com".to_string())
    );
  }
}
