import {expect, getSigners} from '../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP34 from '../../../../typechain-generated/constructors/my_psp34_mintable'
import ContractPSP34 from '../../../../typechain-generated/contracts/my_psp34_mintable'
import {IdBuilder} from '../../../../typechain-generated/types-arguments/my_psp34_mintable'

describe('MY_PSP34_MINTABLE', () => {
  const id0 = IdBuilder.U8(0)
  const id1 = IdBuilder.U8(1)
  
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

  it('Mint works', async () => {
    const {
      contract,
      defaultSigner: sender,
      alice,
      query,
      close
    } = await setup()

    await expect(query.balanceOf(sender.address)).to.have.output(0)
    await expect(query.balanceOf(alice.address)).to.have.output(0)

    await expect(contract.tx.mint(sender.address, id0)).to.eventually.be.fulfilled
    await expect(contract.tx.mint(alice.address, id1)).to.eventually.be.fulfilled

    await expect(query.balanceOf(sender.address)).to.have.output(1)
    await expect(query.balanceOf(alice.address)).to.have.output(1)

    await close()
  })

  it('Mint existing should fail', async () => {
    const {
      contract,
      alice,
      defaultSigner: sender,
      query,
      close
    } = await setup()

    await expect(contract.tx.mint(sender.address, id0)).to.eventually.be.fulfilled
    await expect(query.balanceOf(sender.address)).to.have.output(1)
    await expect(query.balanceOf(alice.address)).to.have.output(0)

    await expect(contract.tx.mint(sender.address, id0)).to.eventually.be.rejected
    await expect(contract.tx.mint(alice.address, id0)).to.eventually.be.rejected

    await expect(query.balanceOf(sender.address)).to.have.output(1)
    await expect(query.balanceOf(alice.address)).to.have.output(0)

    await close()
  })

})
