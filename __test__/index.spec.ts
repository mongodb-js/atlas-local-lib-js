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

  // Count initial deployments
  let start_deployments_count = (await client.listDeployments()).length

  // Create deployment
  let createDeploymentOptions = {
    name: "test_deployment",
  }
  await client.createDeployment(createDeploymentOptions)

  // Count deployments after creation
  let after_create_deployment_count = (await client.listDeployments()).length
  t.assert(after_create_deployment_count - start_deployments_count === 1)

  // Delete deployment
  await client.deleteDeployment(createDeploymentOptions.name)
  
  // Count deployments after deletion
  let after_delete_deployment_count = (await client.listDeployments()).length
  t.assert(start_deployments_count === after_delete_deployment_count)
})
