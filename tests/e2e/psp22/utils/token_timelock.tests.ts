import {expect, getSigners, oneDay} from './../../helpers'

import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP22 from '../../../../typechain-generated/constructors/my_psp22'
import ContractPSP22 from '../../../../typechain-generated/contracts/my_psp22'
import ConstructorsPSP22Timelock from '../../../../typechain-generated/constructors/my_psp22_token_timelock'
import ContractPSP22Timelock from '../../../../typechain-generated/contracts/my_psp22_token_timelock'
import BN from 'bn.js'

describe('TOKEN_TIMELOCK', () => {
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

  async function setupPSP22TokenTimelock(PSP22Address: string, beneficiaryAddress: string, releaseTime: number | string | BN) {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP22Timelock(api, defaultSigner)
    const contractAddress = (await contractFactory.new(PSP22Address, beneficiaryAddress, releaseTime)).address
    const contract = new ContractPSP22Timelock(contractAddress, defaultSigner, api)

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
    const beneficiary = psp22.defaultSigner


    const releaseTime = ((await psp22.api.query.timestamp.now()).toJSON() as number) + oneDay()

    const timelock = await setupPSP22TokenTimelock(psp22.contract.address, beneficiary.address, releaseTime)

    return { psp22, timelock, beneficiary, releaseTime }
  }

  it('New works', async () => {
    const { psp22: psp22Container, timelock: timelockContainer, beneficiary, releaseTime } = await setup()
    const { contract: psp22 } = psp22Container
    const { query: timelockQuery } = timelockContainer

    await expect(timelockQuery.token()).to.have.output(psp22.address)
    await expect(timelockQuery.beneficiary()).to.have.output(beneficiary.address)
    await expect(timelockQuery.releaseTime()).to.have.output(releaseTime)
  })

  // // this test does not work (we can not set timestamp)
  // it('Release works', async () => {
  //   const { psp22: psp22Container, timelock: timelockContainer, beneficiary, releaseTime } = await setup()
  //   const { contract: psp22, query: psp22Query } = psp22Container
  //   const { contract: timelock } = timelockContainer
  //
  //   const depositedTokens = 1000
  //   // deposit tokens to the contract
  //   await expect(psp22.withSigner(beneficiary).tx.transfer(timelock.address, depositedTokens, [])).to.eventually.be.fulfilled
  //   await expect(psp22Query.balanceOf(timelock.address)).to.have.bnToNumber(depositedTokens)
  //   await expect(psp22Query.balanceOf(beneficiary.address)).to.have.bnToNumber(0)
  //   // TODO this does not work, so the test will fail
  //   console.log((psp22Container.api.tx.timestamp))
  //   await psp22Container.api.tx.timestamp.set(releaseTime).signAndSend(beneficiary)
  //   // release the tokens
  //   await expect(timelock.withSigner(beneficiary).tx.release()).to.eventually.be.fulfilled
  //
  //   // // timelock should be empty
  //   await expect(psp22Query.balanceOf(timelock.address)).to.have.bnToNumber(0)
  //   await expect(psp22Query.balanceOf(beneficiary.address)).to.have.bnToNumber(depositedTokens)
  // })

  it('Release soon should not work', async () => {
    const { psp22: psp22Container, timelock: timelockContainer, beneficiary } = await setup()
    const { contract: psp22, query: psp22Query } = psp22Container
    const { contract: timelock } = timelockContainer

    const depositedTokens = 1000
    // deposit tokens to the contract
    await expect(psp22.withSigner(beneficiary).tx.transfer(timelock.address, depositedTokens, [])).to.eventually.be.fulfilled
    await expect(psp22Query.balanceOf(timelock.address)).to.have.bnToNumber(depositedTokens)
    await expect(psp22Query.balanceOf(beneficiary.address)).to.have.bnToNumber(0)

    // release the tokens early
    await expect(timelock.withSigner(beneficiary).tx.release()).to.eventually.be.rejected

    await expect(psp22Query.balanceOf(timelock.address)).to.have.bnToNumber(depositedTokens)
    await expect(psp22Query.balanceOf(beneficiary.address)).to.have.bnToNumber(0)
  })

  it('Release without deposit should not work', async () => {
    const { psp22: psp22Container, timelock: timelockContainer, beneficiary } = await setup()
    const { query: psp22Query } = psp22Container
    const { contract: timelock } = timelockContainer

    const tokens = 1000
    await expect(psp22Query.balanceOf(timelock.address)).to.have.bnToNumber(0)
    await expect(psp22Query.balanceOf(beneficiary.address)).to.have.bnToNumber(tokens)

    // release the tokens
    await expect(timelock.withSigner(beneficiary).tx.release()).to.eventually.be.rejected

    // // timelock should be empty
    await expect(psp22Query.balanceOf(timelock.address)).to.have.bnToNumber(0)
    await expect(psp22Query.balanceOf(beneficiary.address)).to.have.bnToNumber(tokens)
  })

})
