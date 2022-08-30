import {expect, getSigners} from '../helpers'
import BN from 'bn.js'
import {ApiPromise} from '@polkadot/api'
import ConstructorsCoin from '../../../typechain-generated/constructors/stable_coin_contract'
import ContractCoin from '../../../typechain-generated/contracts/stable_coin_contract'
import ConstructorsLoan from '../../../typechain-generated/constructors/loan_contract'
import ContractLoan from '../../../typechain-generated/contracts/loan_contract'
import ConstructorsShares from '../../../typechain-generated/constructors/shares_contract'
import ContractShares from '../../../typechain-generated/contracts/shares_contract'
import ConstructorsLending from '../../../typechain-generated/constructors/lending_contract'
import ContractLending from '../../../typechain-generated/contracts/lending_contract'
import {assert} from 'chai'

describe('LENDING_CONTRACT', () => {

  async function setup() {
    const stable_coin = await setupCoin()
    const loan = await setuptLoan()
    const shares = await setupShares()

    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const lending = await setupLending(shares.contract.abi.json.source.hash, loan.contract.abi.json.source.hash)

    return { lending, stable_coin, alice: stable_coin.defaultSigner, close: async () => {
      await stable_coin.close()
      await loan.close()
      await shares.close()
      await lending.close()
    } }
  }

  async function setupCoin() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsCoin(api, defaultSigner)
    const contractAddress = (await contractFactory.new('Green Coin', 'GC')).address
    const contract = new ContractCoin(contractAddress, defaultSigner, api)

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
  
  async function setuptLoan() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsLoan(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new ContractLoan(contractAddress, defaultSigner, api)

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
  
  async function setupShares() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsShares(api, defaultSigner)
    const contractAddress = (await contractFactory.new('', '')).address
    const contract = new ContractShares(contractAddress, defaultSigner, api)

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
  
  async function setupLending(sharesHash, loanHash) {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsLending(api, defaultSigner)
    const contractAddress = (await contractFactory.new(sharesHash, loanHash)).address
    const contract = new ContractLending(contractAddress, defaultSigner, api)

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

  it('LENDING CONTRACT - lend asset', async () => {
    const { lending, stable_coin, alice, close } = await setup()

    const amount = 100

    // Alice balance should be >= than lending `amount`
    const alice_balance = (await stable_coin.query.balanceOf(alice.address)).value
    assert(alice_balance.rawNumber.gte(new BN(amount)), 'Alice balance should be >= than lending `amount`')

    // Alice approves `amount` for lending contact
    await expect(stable_coin.tx.approve(lending.contract.address, amount)).to.eventually.be.fulfilled

    // Allow new asset(stable coin) in the lending
    await expect(lending.tx.allowAsset(stable_coin.contract.address)).to.eventually.be.fulfilled

    // Alice lends `amount` tokens into lending
    await expect(stable_coin.query.balanceOf(lending.contract.address)).to.have.bnToNumber(0)
    expect((await stable_coin.query.balanceOf(alice.address)).value).to.be.deep.equal(alice_balance)
    await expect(lending.contract.withSigner(alice).tx.lendAssets(stable_coin.contract.address, amount)).to.eventually.be.fulfilled
    await expect(stable_coin.query.balanceOf(lending.contract.address)).to.have.bnToNumber(amount)
    expect((await stable_coin.query.balanceOf(alice.address)).value.rawNumber.toString()).to.be.equal(alice_balance.rawNumber.sub(new BN(amount)).toString())

    await close()
  })
})
