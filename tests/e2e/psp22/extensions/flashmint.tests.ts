import {expect, getSigners} from './../../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP22Flashmint from '../../../../typechain-generated/constructors/my_psp22_flashmint'
import ContractPSP22Flashmint from '../../../../typechain-generated/contracts/my_psp22_flashmint'
import ConstructorsPSP22FlashBorrower from '../../../../typechain-generated/constructors/flash_borrower'
import ContractPSP22FlashBorrower from '../../../../typechain-generated/contracts/flash_borrower'

describe('MY_PSP22_FLASHMINT', () => {
  async function setupFlashmint() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP22Flashmint(api, defaultSigner)
    const contractAddress = (await contractFactory.new(1000)).address
    const contract = new ContractPSP22Flashmint(contractAddress, defaultSigner, api)

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
  
  async function setupFlashBorrower() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP22FlashBorrower(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new ContractPSP22FlashBorrower(contractAddress, defaultSigner, api)

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
  
  async function setup() {
    const flashmint = await setupFlashmint()
    const receiver = await setupFlashBorrower()

    return { flashmint, receiver, close: async () => {
      await flashmint.close()
      await receiver.close()
    } }
  }
  // TODO: implement tests (bug in typechain)
  it('New works', async () => {
    const { flashmint, close } = await setup()

    // flash fee should be 1%
    const flashFee = await flashmint.query.flashFee(flashmint.contract.address, 100)
    expect((await flashmint.query.flashFee(flashmint.contract.address, 100)).value.Ok).to.be.bnToNumber(1)

    await close()
  })

  it('Flashloan works', async () => {
    const { flashmint, receiver } = await setup()
    const { contract: flashmintContract, query: flashmintQuery } = flashmint
    const { contract: receiverContract } = receiver

    const borrowAmount = 100
    const fee = borrowAmount / 100
    const sendAmount = borrowAmount + fee
    const minted = 1000

    // sender has the initial supply of tokens, we send some to the receiver
    await expect(flashmintContract.tx.transfer(receiverContract.address, sendAmount, [])).to.eventually.be.fulfilled
    await expect(flashmintQuery.balanceOf(receiverContract.address)).to.have.bnToNumber(sendAmount)
    await expect(flashmintQuery.totalSupply()).to.have.bnToNumber(minted)

    // we will do the flashloan
    await expect(flashmintContract.tx.flashloan(receiverContract.address, flashmintContract.address, sendAmount, []))
      .to.eventually.be.fulfilled

    // receiver should have the fee deducted
    await expect(flashmintQuery.balanceOf(receiverContract.address))
      .to.have.bnToNumber(sendAmount - fee)
    // one token should be burned
    await expect(flashmintQuery.totalSupply()).to.have.bnToNumber(minted - fee)

    await flashmint.close()
    await receiver.close()
  })

})
