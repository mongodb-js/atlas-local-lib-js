import test from 'ava'

import { Client } from '../index'

test('smoke test: list deployments', async (t) => {
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

  // List deployments
  // We don't care about the number, we're just testing that the method doesn't fail
  await client.listDeployments()
  t.pass()
})
