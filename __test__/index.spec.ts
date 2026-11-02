import { test, expect } from 'vitest'

import { Client } from '../index'

test('smoke test', async () => {
  let client: Client | null = null

  try {
    // Create client
    client = await Client.connect()
  } catch (e: any) {
    // If docker is not running we get this error
    // any other error means failure
    expect(e?.message).toBe('connect to docker')
    return
  }

  if (client == null) {
    throw new Error('Client not created')
  }

  await expect(
    client.createDeployment({
      embeddingProviderEndpoint: 'https://api.voyageai.com/v1/embeddings',
    }),
  ).rejects.toThrow('voyageApiKey is required')
  await expect(
    client.createDeployment({
      voyageApiKey: 'test-voyage-api-key',
      embeddingProviderEndpoint: 'http://api.voyageai.com/v1/embeddings',
    }),
  ).rejects.toThrow('absolute HTTPS URL')

  // Skip test after client creation on Windows
  // Note all Windows return win32 including 64 bit
  if (process.platform === 'win32') {
    return
  }

  // Count initial deployments
  let start_deployments_count = (await client.listDeployments()).length

  // Create deployment
  let createDeploymentOptions = {
    name: 'test_deployment',
    doNotTrack: true,
    voyageApiKey: 'test-voyage-api-key',
    embeddingProviderEndpoint: 'https://api.voyageai.com/v1/embeddings',
  }
  let deployment = await client.createDeployment(createDeploymentOptions)
  expect(deployment.name).toBe(createDeploymentOptions.name)
  expect(deployment.embeddingProviderEndpoint).toBe(
    createDeploymentOptions.embeddingProviderEndpoint,
  )

  // Get the deployment id twice, make sure it's the same
  let deploymentId = await client.getDeploymentId(createDeploymentOptions.name)
  let deploymentId2 = await client.getDeploymentId(createDeploymentOptions.name)
  expect(deploymentId).toBe(deploymentId2)

  // Get deployment
  let getDeployment = await client.getDeployment(createDeploymentOptions.name)
  expect(getDeployment.name).toBe(createDeploymentOptions.name)

  let connString = await client.getConnectionString(createDeploymentOptions.name)
  expect(connString).toBe(`mongodb://127.0.0.1:${getDeployment.portBindings.port}/?directConnection=true`)

  // Count deployments after creation
  let after_create_deployment_count = (await client.listDeployments()).length
  expect(after_create_deployment_count - start_deployments_count).toBe(1)

  // Delete deployment
  await client.deleteDeployment(createDeploymentOptions.name)

  // Count deployments after deletion
  let after_delete_deployment_count = (await client.listDeployments()).length
  expect(start_deployments_count).toBe(after_delete_deployment_count)
})
