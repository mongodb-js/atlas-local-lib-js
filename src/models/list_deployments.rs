use napi_derive::napi;

#[napi(object)]
pub struct Deployment {
  // Identifiers
  pub container_id: String,
  pub name: Option<String>,

  // Docker specific
  pub state: State,
  pub port_bindings: Option<MongoDBPortBinding>,

  // MongoDB details (MongoD)
  pub mongodb_type: MongodbType,
  pub mongodb_version: String,

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
  pub do_not_track: Option<String>,
  pub telemetry_base_url: Option<String>,
}

#[napi(string_enum)]
pub enum State {
  Created,
  Dead,
  Exited,
  Paused,
  Removing,
  Restarting,
  Running,
}

#[napi(object)]
pub struct MongoDBPortBinding {
  #[napi(js_name = "type")]
  pub binding_type: BindingType,
  pub ip: String,
  pub port: u16,
}

#[napi(string_enum)]
pub enum BindingType {
  Loopback,     // 127.0.0.1
  AnyInterface, // 0.0.0.0
  Specific,     // Specific IP address
}

#[napi(string_enum)]
pub enum MongodbType {
  Community,
  Enterprise,
}

#[napi(object)]
pub struct CreationSource {
  #[napi(js_name = "type")]
  pub source_type: CreationSourceType,
  pub source: String,
}

#[napi(string_enum)]
pub enum CreationSourceType {
  AtlasCLI,
  Container,
  Other,
}

impl From<atlas_local::models::Deployment> for Deployment {
  fn from(source: atlas_local::models::Deployment) -> Self {
    Self {
      container_id: source.container_id,
      name: source.name,
      state: source.state.into(),
      port_bindings: source.port_bindings.map(MongoDBPortBinding::from),
      mongodb_type: source.mongodb_type.into(),
      mongodb_version: source.mongodb_version.to_string(),
      creation_source: source.creation_source.map(CreationSource::from),
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
    }
  }
}

impl From<atlas_local::models::State> for State {
  fn from(source: atlas_local::models::State) -> Self {
    match source {
      atlas_local::models::State::Created => State::Created,
      atlas_local::models::State::Dead => State::Dead,
      atlas_local::models::State::Exited => State::Exited,
      atlas_local::models::State::Paused => State::Paused,
      atlas_local::models::State::Removing => State::Removing,
      atlas_local::models::State::Restarting => State::Restarting,
      atlas_local::models::State::Running => State::Running,
    }
  }
}

impl From<atlas_local::models::MongoDBPortBinding> for MongoDBPortBinding {
  fn from(source: atlas_local::models::MongoDBPortBinding) -> Self {
    use atlas_local::models::BindingType as SourceType;

    match source.binding_type {
      SourceType::Loopback => MongoDBPortBinding {
        binding_type: BindingType::Loopback,
        ip: "127.0.0.1".to_string(),
        port: source.port,
      },
      SourceType::AnyInterface => MongoDBPortBinding {
        binding_type: BindingType::AnyInterface,
        ip: "0.0.0.0".to_string(),
        port: source.port,
      },
      SourceType::Specific(ip) => MongoDBPortBinding {
        binding_type: BindingType::Specific,
        ip: ip.to_string(),
        port: source.port,
      },
    }
  }
}

impl From<atlas_local::models::MongodbType> for MongodbType {
  fn from(source: atlas_local::models::MongodbType) -> Self {
    match source {
      atlas_local::models::MongodbType::Community => MongodbType::Community,
      atlas_local::models::MongodbType::Enterprise => MongodbType::Enterprise,
    }
  }
}

impl From<atlas_local::models::CreationSource> for CreationSource {
  fn from(source: atlas_local::models::CreationSource) -> Self {
    use atlas_local::models::CreationSource as CreationSourceSource;

    match source {
      CreationSourceSource::AtlasCLI => CreationSource {
        source_type: CreationSourceType::AtlasCLI,
        source: "ATLASCLI".to_string(),
      },
      CreationSourceSource::Container => CreationSource {
        source_type: CreationSourceType::Container,
        source: "CONTAINER".to_string(),
      },
      CreationSourceSource::Unknown(source) => CreationSource {
        source_type: CreationSourceType::Other,
        source,
      },
    }
  }
}
