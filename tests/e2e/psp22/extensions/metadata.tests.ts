import {expect, getSigners} from '../../helpers'
import {ApiPromise} from '@polkadot/api'
import Constructors from '../../../../typechain-generated/constructors/my_psp22_metadata'
import Contract from '../../../../typechain-generated/contracts/my_psp22_metadata'

describe('MY_PSP22_METADATA', () => {
  async function setup() {
    // return setupContract('my_psp22_metadata', 'new', '1000', 'TOKEN', 'TKN', 18)
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new Constructors(api, defaultSigner)
    const contractAddress = (await contractFactory.new(1000, 'TOKEN', 'TKN', 18)).address
    const contract = new Contract(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx
    }
  }

  it('Metadata works', async () => {
    const { api, query, defaultSigner: sender } = await setup()

    await expect(query.tokenName()).to.have.output('TOKEN')
    await expect(query.tokenSymbol()).to.have.output('TKN')
    await expect(query.tokenDecimals()).to.have.output(18)

    await api.disconnect()
  })

})
