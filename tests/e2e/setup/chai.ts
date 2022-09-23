import chai from 'chai'
import chaiAsPromised from 'chai-as-promised'
import type { AccountId } from '@polkadot/types/interfaces'

declare global {
  export namespace Chai {
    interface Assertion {
      output(value: AccountId | string | number | boolean | string[] | number[] | unknown, msg?: string): void
    }
  }
}

chai.use(chaiAsPromised)
chai.use((chai) => {
  chai.Assertion.addMethod('output', async function (param, message) {
    await new chai.Assertion(this._obj).to.eventually.have.property('output').to.equal(param, message)
  })
})
export const { expect } = chai
