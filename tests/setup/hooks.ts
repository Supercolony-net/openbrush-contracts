import { network } from '@supercolony/redspot'

const { api } = network

export const mochaHooks = {
  afterAll: () => {
    api.disconnect()
  }
}
