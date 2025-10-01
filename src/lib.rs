#![deny(clippy::all)]

use anyhow::{Context, Result};
use atlas_local::Client as AtlasLocalClient;
use bollard::Docker;
use napi_derive::napi;

use crate::models::list_deployments::Deployment;

pub mod models;

#[napi]
pub struct Client {
  client: AtlasLocalClient,
}

#[napi]
impl Client {
  #[napi(factory)]
  pub fn connect() -> Result<Client> {
    let docker = Docker::connect_with_defaults().context("connect to docker")?;

    let atlas_local_client = AtlasLocalClient::new(docker);

    Ok(Client {
      client: atlas_local_client,
    })
  }

  #[napi]
  pub async fn create_deployment(
    &self,
    create_deploment_options: crate::models::create_deployment::CreateDeploymentOptions,
  ) -> Result<Deployment> {
    let options: atlas_local::models::CreateDeploymentOptions = create_deploment_options.into();
    self
      .client
      .create_deployment(&options)
      .await
      .map(|d| d.into())
      .context("create deployment")
  }

  #[napi]
  pub async fn list_deployments(&self) -> Result<Vec<Deployment>> {
    self
      .client
      .list_deployments()
      .await
      .context("list deployments")
      .map(|deployments| deployments.into_iter().map(|d| d.into()).collect())
  }

  #[napi]
  pub async fn delete_deployment(&self, deployment_name: String) -> Result<()> {
    self
      .client
      .delete_deployment(&deployment_name)
      .await
      .context("delete deployments")
  }

  #[napi]
  pub async fn get_deployment(&self, deployment_name: String) -> Result<Deployment> {
    self
      .client
      .get_deployment(&deployment_name)
      .await
      .context("get deployment")
      .map(|d| d.into())
  }

  #[napi]
  pub async fn get_connection_string(&self, deployment_name: String) -> Result<String> {
    self
      .client
      .get_connection_string(deployment_name)
      .await
      .context("get connection string")
  }
}
