import { network } from 'redspot'

const { api } = network

export const mochaHooks = {
  afterAll: () => {
    api.disconnect()
  }
}
