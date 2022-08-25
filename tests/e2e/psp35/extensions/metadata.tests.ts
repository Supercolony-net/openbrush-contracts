import {bnArg, expect, getSigners} from '../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP35 from '../../../../typechain-generated/constructors/my_psp35_metadata'
import {IdBuilder} from '../../../../typechain-generated/types-arguments/my_psp35_metadata'
import ContractPSP35 from '../../../../typechain-generated/contracts/my_psp35_metadata'
import {bytesToHex} from '@noble/hashes/utils'

describe('MY_PSP35_METADATA', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP35(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new ContractPSP35(contractAddress, defaultSigner, api)

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
    const { query, tx } = await setup()
    const encoder = new TextEncoder()
    const id = IdBuilder.U8(0)

    await expect(query.getAttribute(id, bnArg(1))).to.have.output(null)

    await expect(tx.setAttribute(id, bnArg(1), bnArg(encoder.encode('https://www.supercolony.net/')))).to.eventually.be.fulfilled

    await expect(query.getAttribute(id, bnArg(1))).to.have.output(
      '0x' + bytesToHex(
        Uint8Array.from(bnArg(encoder.encode('https://www.supercolony.net/')))
      )
    )
  })

})
