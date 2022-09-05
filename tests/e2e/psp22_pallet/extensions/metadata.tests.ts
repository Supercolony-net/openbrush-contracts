import {expect, getSigners} from '../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP22 from '../../../../typechain-generated/constructors/my_psp22_pallet_metadata'
import ContractPSP22 from '../../../../typechain-generated/contracts/my_psp22_pallet_burnable'

describe('MY_PSP22_METADATA', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]


    const contractFactory = new ConstructorsPSP22(api, defaultSigner)
    const contractAddress = (await contractFactory.new(Math.floor(Math.random() * 10000) + 1, 1, 1000, 'TOKEN' as unknown as [], 'TKN' as unknown as [], 18, {value: '10000000000000000'})).address
    const contract = new ContractPSP22(contractAddress, defaultSigner, api)

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
  // TODO: not works from contract side
  // it('Metadata works', async () => {
  //   const { query, defaultSigner: sender } = await setup()
  //
  //   await expect(query.tokenName()).to.have.output('TOKEN')
  //   await expect(query.tokenSymbol()).to.have.output('TKN')
  //   await expect(query.tokenDecimals()).to.have.output(18)
  // })

})
