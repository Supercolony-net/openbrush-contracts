import {expect, getSigners} from '../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP34 from '../../../../typechain-generated/constructors/my_psp34_burnable'
import ContractPSP34 from '../../../../typechain-generated/contracts/my_psp34_burnable'
import {IdBuilder} from '../../../../typechain-generated/types-arguments/my_psp34';

describe('MY_PSP34_BURNABLE', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP34(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
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

  it('Burn works', async () => {
    const {
      contract,
      defaultSigner: sender,
      query,
      close
    } = await setup()

    await expect(query.balanceOf(sender.address)).to.have.output(3)

    await expect(contract.tx.burn(sender.address, IdBuilder.U8(0))).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address)).to.have.output(2)

    await close()
  })

  it('Burn from works', async () => {
    const {
      contract,
      defaultSigner: sender,
      alice,
      query,
      close
    } = await setup()

    await expect(query.balanceOf(sender.address)).to.have.output(3)

    await expect(contract.withSigner(alice).tx.burn(sender.address, IdBuilder.U8(0))).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address)).to.have.output(2)

    await close()
  })
})
