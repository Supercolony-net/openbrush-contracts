import {expect, getSigners} from './../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP22 from '../../../../typechain-generated/constructors/my_psp22_capped'
import ContractPSP22 from '../../../../typechain-generated/contracts/my_psp22_capped'

describe('MY_PSP22_CAPPED', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP22(api, defaultSigner)
    const contractAddress = (await contractFactory.new(1000, 2000)).address
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

  it('New works', async () => {
    const { api, query, defaultSigner: sender } = await setup()

    await expect(query.balanceOf(sender.address)).to.have.bnToNumber(1000)
    await expect(query.totalSupply()).to.have.bnToNumber(1000)
    await expect(query.cap()).to.have.bnToNumber(2000)

    await api.disconnect()
  })

  it('Can mint when total supply is lower than cap', async () => {
    const { api, contract, query, defaultSigner: sender } = await setup()

    const mintAmount = 1000

    await expect(query.balanceOf(sender.address)).to.have.bnToNumber(mintAmount)
    await expect(query.totalSupply()).to.have.bnToNumber(mintAmount)

    // mint tokens to sender
    await expect(contract.tx.mint(sender.address, mintAmount)).to.eventually.be.fulfilled

    // sender's balance changed
    await expect(query.balanceOf(sender.address)).to.have.bnToNumber(mintAmount + mintAmount)
    // total supply changed
    await expect(query.totalSupply()).to.have.bnToNumber(mintAmount + mintAmount)

    await api.disconnect()
  })

  it('Can not mint if total supply will exceed the cap', async () => {
    const { api, contract, query, defaultSigner: sender } = await setup()
        
    const mintAmount = 1000
    const newMintAmount = 1001
        
    await expect(query.balanceOf(sender.address)).to.have.bnToNumber(mintAmount)
    await expect(query.totalSupply()).to.have.bnToNumber(mintAmount)
        
    // mint tokens to sender
    await expect(contract.tx.mint(sender.address, newMintAmount)).to.eventually.be.rejected
        
    // sender's balance did not change
    await expect(query.balanceOf(sender.address)).to.have.bnToNumber(mintAmount)
    // total supply did not change
    await expect(query.totalSupply()).to.have.bnToNumber(mintAmount)

    await api.disconnect()
  })

})
