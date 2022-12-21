import {bytesToString, expect, getSigners} from '../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP34 from '../../../../typechain-generated/constructors/my_psp34_metadata'
import ContractPSP34 from '../../../../typechain-generated/contracts/my_psp34_metadata'
import {IdBuilder} from '../../../../typechain-generated/types-arguments/my_psp34_metadata'

describe('MY_PSP34_METADATA', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP34(api, defaultSigner)
    const contractAddress = (await contractFactory.new(IdBuilder.U8(1), 'Non Fungible Token' as unknown as string[], 'NFT' as unknown as string[])).address
    const contract = new ContractPSP34(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx,
      close: async () => {
        await api.disconnect()
      }
    }
  }

  it('Metadata works', async () => {
    const { query, close } = await setup()

    let result = bytesToString((await query.getAttribute(IdBuilder.U8(1), 'name' as unknown as string[])).value.unwrapRecursively()! as unknown as string)
    expect(result).to.be.eq('Non Fungible Token')
    result = bytesToString((await query.getAttribute(IdBuilder.U8(1), 'symbol' as unknown as string[])).value.unwrapRecursively()! as unknown as string)
    expect(result).to.be.eq('NFT')

    await close()
  })
})
