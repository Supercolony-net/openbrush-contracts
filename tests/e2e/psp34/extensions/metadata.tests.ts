import {expect, getSigners} from '../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP34 from '../../../../typechain-generated/constructors/my_psp34_metadata'
import ContractPSP34 from '../../../../typechain-generated/contracts/my_psp34_metadata'
import {IdBuilder} from '../../../../typechain-generated/types-arguments/my_psp34_metadata'

describe('MY_PSP34_METADATA', () => {
  function bytesToString(bytes: string): string {
    const outputNumber = bytes.substring(2).split('').map(x => parseInt(x as unknown as string, 16))

    const length = outputNumber.length
    let result = ''
    for (let i = 0; i < length; i += 2) {
      result += String.fromCharCode(outputNumber[i] * 16 + outputNumber[i + 1])
    }

    return result
  }

  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP34(api, defaultSigner)
    const contractAddress = (await contractFactory.new(IdBuilder.U8(1), 'Non Fungible Token', 'NFT')).address
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

    let result = bytesToString((await query.getAttribute(IdBuilder.U8(1), 'name' as unknown as string[])).value! as unknown as string)
    expect(result).to.be.eq('Non Fungible Token')
    result = bytesToString((await query.getAttribute(IdBuilder.U8(1), 'symbol' as unknown as string[])).value! as unknown as string)
    expect(result).to.be.eq('NFT')

    await close()
  })
})
