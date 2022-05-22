import { network } from 'redspot'

const { api } = network

export const mochaHooks = {
  afterAll: () => {
    api.disconnect()
  },
  beforeEach: function (done) {
    setTimeout(done, 1000);
  }
}
