import test from 'ava'

import { Client } from '../index'

test('smoke test', async (t) => {
  let client: Client | null = null

  try {
    // Create client
    client = await Client.connect()
  } catch (e: any) {
    // If docker is not running we get this error
    // any other error means failure
    t.is(e?.message, 'connect to docker')
    return
  }

  if (client == null) {
    t.fail('Client not created')
    return
  }

  // Skip test after client creation on Windows
  // Note all Windows return win32 including 64 bit
  if (process.platform === 'win32') {
    t.pass('Skipping end-to-end test on Windows')
    return
  }
  
  // Count initial deployments
  let start_deployments_count = (await client.listDeployments()).length

  // Create deployment
  let createDeploymentOptions = {
    name: "test_deployment",
  }
  let deployment = await client.createDeployment(createDeploymentOptions)
  t.is(deployment.name, createDeploymentOptions.name)

  // Get the deployment id twice, make sure it's the same
  let deploymentId = await client.getDeploymentId(createDeploymentOptions.name)
  let deploymentId2 = await client.getDeploymentId(createDeploymentOptions.name)
  t.is(deploymentId, deploymentId2)

  // Get deployment
  let getDeployment = await client.getDeployment(createDeploymentOptions.name)
  t.is(getDeployment.name,createDeploymentOptions.name)

  let connString = await client.getConnectionString(createDeploymentOptions.name)
  t.assert(connString === `mongodb://user:password@127.0.0.1:${getDeployment.portBindings.port}/?directConnection=true`)

  // Count deployments after creation
  let after_create_deployment_count = (await client.listDeployments()).length
  t.assert(after_create_deployment_count - start_deployments_count === 1)

  // Delete deployment
  await client.deleteDeployment(createDeploymentOptions.name)
  
  // Count deployments after deletion
  let after_delete_deployment_count = (await client.listDeployments()).length
  t.assert(start_deployments_count === after_delete_deployment_count)
})
