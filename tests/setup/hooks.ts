import { network } from 'redspot'

const { api } = network

export const mochaHooks = {
  beforeEach: async () => {
    await api.connect()
  },
  afterEach: async () => {
    await api.disconnect()
  }
}
