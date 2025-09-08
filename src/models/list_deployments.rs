use std::net::IpAddr;

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
#[derive(PartialEq, Debug)]
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
#[derive(PartialEq, Debug)]
pub struct MongoDBPortBinding {
  #[napi(js_name = "type")]
  pub binding_type: BindingType,
  pub ip: String,
  pub port: u16,
}

#[napi(string_enum)]
#[derive(PartialEq, Debug)]
pub enum BindingType {
  Loopback,     // 127.0.0.1
  AnyInterface, // 0.0.0.0
  Specific,     // Specific IP address
}

#[napi(string_enum)]
#[derive(PartialEq, Debug)]
pub enum MongodbType {
  Community,
  Enterprise,
}

#[napi(object)]
#[derive(PartialEq, Debug)]
pub struct CreationSource {
  #[napi(js_name = "type")]
  pub source_type: CreationSourceType,
  pub source: String,
}

#[napi(string_enum)]
#[derive(PartialEq, Debug)]
pub enum CreationSourceType {
  AtlasCLI,
  Container,
  MCPServer,
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

impl From<MongoDBPortBinding> for atlas_local::models::MongoDBPortBinding {
  fn from(source: MongoDBPortBinding) -> Self {
    match source.binding_type {
      BindingType::Loopback => atlas_local::models::MongoDBPortBinding {
        binding_type: atlas_local::models::BindingType::Loopback,
        port: source.port,
      },
      BindingType::AnyInterface => atlas_local::models::MongoDBPortBinding {
        binding_type: atlas_local::models::BindingType::AnyInterface,
        port: source.port,
      },
      BindingType::Specific => atlas_local::models::MongoDBPortBinding {
        binding_type: atlas_local::models::BindingType::Specific(
          source.ip.parse::<IpAddr>().expect("Parse IP address"),
        ),
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
      CreationSourceSource::MCPServer => CreationSource {
        source_type: CreationSourceType::MCPServer,
        source: "MCPSERVER".to_string(),
      },
      CreationSourceSource::Unknown(source) => CreationSource {
        source_type: CreationSourceType::Other,
        source,
      },
    }
  }
}

impl From<CreationSource> for atlas_local::models::CreationSource {
  fn from(source: CreationSource) -> Self {
    match source.source_type {
      CreationSourceType::AtlasCLI => atlas_local::models::CreationSource::AtlasCLI,
      CreationSourceType::Container => atlas_local::models::CreationSource::Container,
      CreationSourceType::MCPServer => atlas_local::models::CreationSource::MCPServer,
      CreationSourceType::Other => atlas_local::models::CreationSource::Unknown(source.source),
    }
  }
}

#[cfg(test)]
mod tests {
  use semver::Version;

  use super::*;

  #[test]
  fn test_deployment_from_lib_deployment() {
    let lib_deployment = atlas_local::models::Deployment {
      container_id: "container_id".to_string(),
      name: Some("test_deployment".to_string()),
      state: atlas_local::models::State::Running,
      port_bindings: Some(atlas_local::models::MongoDBPortBinding {
        binding_type: atlas_local::models::BindingType::Loopback,
        port: 27017,
      }),
      mongodb_type: atlas_local::models::MongodbType::Community,
      mongodb_version: Version::new(8, 0, 0),
      creation_source: Some(atlas_local::models::CreationSource::AtlasCLI),
      local_seed_location: Some("/host/seed-data".to_string()),
      mongodb_initdb_database: Some("testdb".to_string()),
      mongodb_initdb_root_password_file: Some("/run/secrets/password".to_string()),
      mongodb_initdb_root_password: Some("password123".to_string()),
      mongodb_initdb_root_username_file: Some("/run/secrets/username".to_string()),
      mongodb_initdb_root_username: Some("admin".to_string()),
      mongot_log_file: Some("/tmp/mongot.log".to_string()),
      runner_log_file: Some("/tmp/runner.log".to_string()),
      do_not_track: Some("false".to_string()),
      telemetry_base_url: Some("https://telemetry.example.com".to_string()),
    };

    let deployment: Deployment = lib_deployment.into();

    assert_eq!(deployment.container_id, "container_id");
    assert_eq!(deployment.name, Some("test_deployment".to_string()));
    assert_eq!(deployment.state, State::Running);
    assert!(deployment.port_bindings.is_some());
    let port_binding = deployment.port_bindings.unwrap();
    assert_eq!(port_binding.binding_type, BindingType::Loopback);
    assert_eq!(port_binding.ip, "127.0.0.1");
    assert_eq!(port_binding.port, 27017);
    assert_eq!(deployment.mongodb_type, MongodbType::Community);
    assert_eq!(deployment.mongodb_version, "8.0.0");
    assert_eq!(
      deployment.creation_source,
      Some(CreationSource {
        source_type: CreationSourceType::AtlasCLI,
        source: "ATLASCLI".to_string(),
      })
    );
    assert_eq!(
      deployment.local_seed_location,
      Some("/host/seed-data".to_string())
    );
    assert_eq!(
      deployment.mongodb_initdb_database,
      Some("testdb".to_string())
    );
    assert_eq!(
      deployment.mongodb_initdb_root_password_file,
      Some("/run/secrets/password".to_string())
    );
    assert_eq!(
      deployment.mongodb_initdb_root_password,
      Some("password123".to_string())
    );
    assert_eq!(
      deployment.mongodb_initdb_root_username_file,
      Some("/run/secrets/username".to_string())
    );
    assert_eq!(
      deployment.mongodb_initdb_root_username,
      Some("admin".to_string())
    );
    assert_eq!(
      deployment.mongot_log_file,
      Some("/tmp/mongot.log".to_string())
    );
    assert_eq!(
      deployment.runner_log_file,
      Some("/tmp/runner.log".to_string())
    );
    assert_eq!(deployment.do_not_track, Some("false".to_string()));
    assert_eq!(
      deployment.telemetry_base_url,
      Some("https://telemetry.example.com".to_string())
    );
  }

  #[test]
  fn test_mongodb_port_binding_from_lib_mongodb_port_binding_loopback() {
    let lib_mongodb_port_binding = atlas_local::models::MongoDBPortBinding {
      binding_type: atlas_local::models::BindingType::Loopback,
      port: 27017,
    };
    let mongodb_port_binding: MongoDBPortBinding = lib_mongodb_port_binding.into();
    assert_eq!(mongodb_port_binding.binding_type, BindingType::Loopback);
    assert_eq!(mongodb_port_binding.ip, "127.0.0.1");
    assert_eq!(mongodb_port_binding.port, 27017);
  }

  #[test]
  fn test_mongodb_port_binding_from_lib_mongodb_port_binding_any_interface() {
    let lib_mongodb_port_binding = atlas_local::models::MongoDBPortBinding {
      binding_type: atlas_local::models::BindingType::AnyInterface,
      port: 27017,
    };
    let mongodb_port_binding: MongoDBPortBinding = lib_mongodb_port_binding.into();
    assert_eq!(mongodb_port_binding.binding_type, BindingType::AnyInterface);
    assert_eq!(mongodb_port_binding.ip, "0.0.0.0");
    assert_eq!(mongodb_port_binding.port, 27017);
  }

  #[test]
  fn test_mongodb_port_binding_from_lib_mongodb_port_binding_specific() {
    let lib_mongodb_port_binding = atlas_local::models::MongoDBPortBinding {
      binding_type: atlas_local::models::BindingType::Specific("192.0.2.0".parse().unwrap()),
      port: 27017,
    };
    let mongodb_port_binding: MongoDBPortBinding = lib_mongodb_port_binding.into();
    assert_eq!(mongodb_port_binding.binding_type, BindingType::Specific);
    assert_eq!(mongodb_port_binding.ip, "192.0.2.0");
    assert_eq!(mongodb_port_binding.port, 27017);
  }

  #[test]
  fn test_mongodb_port_binding_lib_into_mongodb_port_binding_loopback() {
    let mongodb_port_binding = MongoDBPortBinding {
      binding_type: BindingType::Loopback,
      ip: "127.0.0.1".to_string(),
      port: 27017,
    };
    let lib_mongodb_port_binding: atlas_local::models::MongoDBPortBinding =
      mongodb_port_binding.into();
    assert_eq!(
      lib_mongodb_port_binding.binding_type,
      atlas_local::models::BindingType::Loopback
    );
    assert_eq!(lib_mongodb_port_binding.port, 27017);
  }

  #[test]
  fn test_mongodb_port_binding_lib_into_mongodb_port_binding_any_interface() {
    let mongodb_port_binding = MongoDBPortBinding {
      binding_type: BindingType::AnyInterface,
      ip: "0.0.0.0".to_string(),
      port: 27017,
    };
    let lib_mongodb_port_binding: atlas_local::models::MongoDBPortBinding =
      mongodb_port_binding.into();
    assert_eq!(
      lib_mongodb_port_binding.binding_type,
      atlas_local::models::BindingType::AnyInterface
    );
    assert_eq!(lib_mongodb_port_binding.port, 27017);
  }
  #[test]
  fn test_mongodb_port_binding_lib_into_mongodb_port_binding_specific() {
    let mongodb_port_binding = MongoDBPortBinding {
      binding_type: BindingType::Specific,
      ip: "192.0.2.0".to_string(),
      port: 27017,
    };
    let lib_mongodb_port_binding: atlas_local::models::MongoDBPortBinding =
      mongodb_port_binding.into();
    assert_eq!(
      lib_mongodb_port_binding.binding_type,
      atlas_local::models::BindingType::Specific("192.0.2.0".parse().unwrap())
    );
    assert_eq!(lib_mongodb_port_binding.port, 27017);
  }

  #[test]
  fn test_creation_source_from_lib_creation_source_atlas_cli() {
    let lib_creation_source = atlas_local::models::CreationSource::AtlasCLI;
    let creation_source: CreationSource = lib_creation_source.into();
    assert_eq!(creation_source.source_type, CreationSourceType::AtlasCLI);
    assert_eq!(creation_source.source, "ATLASCLI");
  }

  #[test]
  fn test_creation_source_from_lib_creation_source_container() {
    let lib_creation_source = atlas_local::models::CreationSource::Container;
    let creation_source: CreationSource = lib_creation_source.into();
    assert_eq!(creation_source.source_type, CreationSourceType::Container);
    assert_eq!(creation_source.source, "CONTAINER");
  }

  #[test]
  fn test_creation_source_from_lib_creation_source_mcp_server() {
    let lib_creation_source = atlas_local::models::CreationSource::MCPServer;
    let creation_source: CreationSource = lib_creation_source.into();
    assert_eq!(creation_source.source_type, CreationSourceType::MCPServer);
    assert_eq!(creation_source.source, "MCPSERVER");
  }

  #[test]
  fn test_creation_source_from_lib_creation_source_unknown() {
    let lib_creation_source = atlas_local::models::CreationSource::Unknown("test".to_string());
    let creation_source: CreationSource = lib_creation_source.into();
    assert_eq!(creation_source.source_type, CreationSourceType::Other);
    assert_eq!(creation_source.source, "test");
  }
}
