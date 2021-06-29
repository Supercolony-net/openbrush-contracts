import chai from 'chai'
import chaiAsPromised from 'chai-as-promised'

declare global {
  export namespace Chai {
    interface Assertion {
      output(value: string | number | boolean | string[] | number[], msg?: string): void
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
