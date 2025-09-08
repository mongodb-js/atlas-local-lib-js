use crate::models::list_deployments::{CreationSource, MongoDBPortBinding};
use napi_derive::napi;
use semver::Version;

#[napi(object)]
pub struct CreateDeploymentOptions {
  // Identifiers
  pub name: Option<String>,

  // Image details
  pub image: Option<String>,
  pub mongodb_version: Option<String>,

  // Creation source
  pub creation_source: Option<CreationSource>,

  // Initial database configuration
  pub local_seed_location: Option<String>,
  pub mongodb_initdb_database: Option<String>,
  pub mongodb_initdb_root_password_file: Option<String>,
  pub mongodb_initdb_root_password: Option<String>,
  pub mongodb_initdb_root_username_file: Option<String>,
  pub mongodb_initdb_root_username: Option<String>,

  // Logging
  pub mongot_log_file: Option<String>,
  pub runner_log_file: Option<String>,

  // Telemetry
  pub do_not_track: Option<bool>,
  pub telemetry_base_url: Option<String>,

  // Port configuration
  pub mongodb_port_binding: Option<MongoDBPortBinding>,
}

impl From<CreateDeploymentOptions> for atlas_local::models::CreateDeploymentOptions {
  fn from(source: CreateDeploymentOptions) -> Self {
    let version: Option<Version> = match source.mongodb_version.as_deref() {
      Some("latest") => None,
      None => None,
      Some(ver_string) => {
        // If malformed Version if given, it will panic here
        Some(Version::parse(ver_string).expect("Parse version string"))
      }
    };

    Self {
      name: source.name,
      image: source.image,
      mongodb_version: version,
      creation_source: source
        .creation_source
        .map(atlas_local::models::CreationSource::from),
      local_seed_location: source.local_seed_location,
      mongodb_initdb_database: source.mongodb_initdb_database,
      mongodb_initdb_root_password_file: source.mongodb_initdb_root_password_file,
      mongodb_initdb_root_password: source.mongodb_initdb_root_password,
      mongodb_initdb_root_username_file: source.mongodb_initdb_root_username_file,
      mongodb_initdb_root_username: source.mongodb_initdb_root_username,
      mongot_log_file: source.mongot_log_file,
      runner_log_file: source.runner_log_file,
      do_not_track: source.do_not_track,
      telemetry_base_url: source.telemetry_base_url,
      mongodb_port_binding: source
        .mongodb_port_binding
        .map(atlas_local::models::MongoDBPortBinding::from),
    }
  }
}
