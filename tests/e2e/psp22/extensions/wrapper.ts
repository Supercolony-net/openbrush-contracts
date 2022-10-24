import {expect, getSigners} from './../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP22 from '../../../../typechain-generated/constructors/my_psp22'
import ContractPSP22 from '../../../../typechain-generated/contracts/my_psp22'
import ConstructorsPSP22Wrapper from '../../../../typechain-generated/constructors/my_psp22_wrapper'
import ContractPSP22Wrapper from '../../../../typechain-generated/contracts/my_psp22_wrapper'

describe('MY_PSP22_WRAPPER', () => {
  async function setupPSP22() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP22(api, defaultSigner)
    const contractAddress = (await contractFactory.new(1000)).address
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
  
  async function setupWrapper(address: string) {
    const api = await ApiPromise.create()
    
    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]
    
    const contractFactory = new ConstructorsPSP22Wrapper(api, defaultSigner)
    const contractAddress = (await contractFactory.new(address)).address
    const contract = new ContractPSP22Wrapper(contractAddress, defaultSigner, api)
    
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
  
  async function setup() {
    const psp22 = await setupPSP22()
    const wrapper = await setupWrapper(psp22.contract.address)

    return { psp22, wrapper }
  }

  it('Deposit for works', async () => {
    const { psp22, wrapper } = await setup()
    const { contract: wrapperContract, query: wrapperQuery } = wrapper
    const { contract: psp22Contract, query: psp22Query, defaultSigner: sender } = psp22

    // sender has 1000 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.bnToNumber(1000)
    // sender has 0 wrapper tokens
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.bnToNumber(0)
    // contract has 0 tokens
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.bnToNumber(0)

    // sender allows the contract to spend their tokens
    await expect(psp22Contract.tx.approve(wrapperContract.address, 100)).to.eventually.be.fulfilled
    // sender wraps 100 tokens
    await expect(wrapperContract.withSigner(sender).tx.depositFor(sender.address, 100)).to.eventually.be.fulfilled

    // sender has 900 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.bnToNumber(900)
    // sender has 100 wrapper tokens
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.bnToNumber(100)
    // contract has 100 tokens
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.bnToNumber(100)

    await psp22.api.disconnect()
    await wrapper.api.disconnect()
  })

  it('Deposit without allowance should fail', async () => {
    const { psp22, wrapper } = await setup()
    const { contract: wrapperContract, query: wrapperQuery } = wrapper
    const { query: psp22Query, defaultSigner: sender } = psp22

    // sender has 1000 tokens and 0 wrapper tokens, contract has 0 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.bnToNumber(1000)
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.bnToNumber(0)
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.bnToNumber(0)

    // sender deposits 100 tokens
    await expect(wrapperContract.withSigner(sender).tx.depositFor(sender.address, 100)).to.eventually.be.rejected

    // sender has 1000 tokens and 0 wrapper tokens, contract has 0 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.bnToNumber(1000)
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.bnToNumber(0)
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.bnToNumber(0)

    await psp22.api.disconnect()
    await wrapper.api.disconnect()
  })

  it('Deposit without underlying tokens should fail', async () => {
    const { psp22, wrapper } = await setup()
    const { contract: wrapperContract, query: wrapperQuery } = wrapper
    const { contract: psp22Contract, query: psp22Query, defaultSigner: sender } = psp22

    // sender has 1000 tokens and 0 wrapper tokens, contract has 0 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.bnToNumber(1000)
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.bnToNumber(0)
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.bnToNumber(0)

    // sender approves contract to spend their tokens, sender deposits 1001 tokens
    await expect(psp22Contract.tx.approve(wrapperContract.address, 100)).to.eventually.be.fulfilled
    await expect(wrapperContract.withSigner(sender).tx.depositFor(sender.address, 1001)).to.eventually.be.rejected

    // sender has 1000 tokens and 0 wrapper tokens, contract has 0 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.bnToNumber(1000)
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.bnToNumber(0)
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.bnToNumber(0)

    await psp22.api.disconnect()
    await wrapper.api.disconnect()
  })

  it('Withdraw to works', async () => {
    const { psp22, wrapper } = await setup()
    const { contract: wrapperContract, query: wrapperQuery } = wrapper
    const { contract: psp22Contract, query: psp22Query, defaultSigner: sender } = psp22

    // sender allows the contract to spend their tokens
    await expect(psp22Contract.tx.approve(wrapperContract.address, 100)).to.eventually.be.fulfilled
    // sender wraps 100 tokens
    await expect(wrapperContract.withSigner(sender).tx.depositFor(sender.address, 100)).to.eventually.be.fulfilled
    // sender has 900 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.bnToNumber(900)
    // sender has 100 wrapper tokens
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.bnToNumber(100)
    // contract has 100 tokens
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.bnToNumber(100)

    // sender withdraws 100 tokens
    await expect(wrapperContract.withSigner(sender).tx.withdrawTo(sender.address, 100)).to.eventually.be.fulfilled

    // sender has 1000 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.bnToNumber(1000)
    // sender has 0 wrapped tokens
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.bnToNumber(0)
    // contract has 0 tokens
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.bnToNumber(0)

    await psp22.api.disconnect()
    await wrapper.api.disconnect()
  })

  it('Withdraw without deposit should fail', async () => {
    const { psp22, wrapper } = await setup()
    const { contract: wrapperContract, query: wrapperQuery } = wrapper
    const { query: psp22Query, defaultSigner: sender } = psp22

    // sender has 1000 tokens and 0 wrapper tokens, contract has 0 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.bnToNumber(1000)
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.bnToNumber(0)
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.bnToNumber(0)

    // sender withdraws 100 tokens
    await expect(wrapperContract.withSigner(sender).tx.withdrawTo(sender.address, 100)).to.eventually.be.rejected

    // sender has 1000 tokens and 0 wrapper tokens, contract has 0 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.bnToNumber(1000)
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.bnToNumber(0)
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.bnToNumber(0)

    await psp22.api.disconnect()
    await wrapper.api.disconnect()
  })

  it('Recover works', async () => {
    const { psp22, wrapper } = await setup()
    const { contract: wrapperContract, query: wrapperQuery } = wrapper
    const { contract: psp22Contract, query: psp22Query, defaultSigner: sender } = psp22

    // sender send 100 tokens to the wrapper contract
    await expect(psp22Contract.tx.transfer(wrapperContract.address, 100, [])).to.eventually.be.fulfilled
    // sender has 900 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.bnToNumber(900)
    // sender has 0 wrapper tokens
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.bnToNumber(0)
    // contract has 100 tokens
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.bnToNumber(100)

    // sender calls recover function
    await expect(wrapperContract.withSigner(sender).tx.recover()).to.eventually.be.fulfilled

    // sender has 900 tokens
    await expect(psp22Query.balanceOf(sender.address)).to.have.bnToNumber(900)
    // sender has 100 wrapped tokens
    await expect(wrapperQuery.balanceOf(sender.address)).to.have.bnToNumber(100)
    // contract has 100 tokens
    await expect(psp22Query.balanceOf(wrapperContract.address)).to.have.bnToNumber(100)

    await psp22.api.disconnect()
    await wrapper.api.disconnect()
  })

})
