import { network } from 'redspot'

const { api } = network

export const mochaHooks = {
  afterEach: () => {
    api.disconnect()
  }
}
