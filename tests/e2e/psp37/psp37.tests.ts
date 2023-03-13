import {expect, getSigners} from '../helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP37 from '../../../typechain-generated/constructors/my_psp37'
import ContractPSP37 from '../../../typechain-generated/contracts/my_psp37'
import {IdBuilder} from '../../../typechain-generated/types-arguments/my_psp37'

describe('MY_PSP37', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP37(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new ContractPSP37(contractAddress, defaultSigner, api)

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

  it('Balance of works', async () => {
    const { query, defaultSigner: sender, tx, close } = await setup()

    const token1 = IdBuilder.U8(0)
    const token2 = IdBuilder.U8(1)

    const amount1 = 1
    const amount2 = 20

    await expect(query.balanceOf(sender.address, null)).to.have.bnToNumber(0)
    await tx.mintTokens(token1, amount1)
    await tx.mintTokens(token2, amount2)

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(amount1)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(amount2)
    await expect(query.balanceOf(sender.address, null)).to.have.bnToNumber(2)

    await close()
  })

  it('Total supply works', async () => {
    const { query, tx, close } = await setup()

    const token1 = IdBuilder.U8(0)
    const token2 = IdBuilder.U8(1)

    const amount1 = 1
    const amount2 = 20

    await expect(query.totalSupply(null)).to.have.bnToNumber(0)
    await tx.mintTokens(token1, amount1)
    
    await expect(query.totalSupply(token1)).to.have.bnToNumber(amount1)
    await expect(query.totalSupply(null)).to.have.bnToNumber(1)

    await tx.mintTokens(token2, amount2)

    await expect(query.totalSupply(token2)).to.have.bnToNumber(amount2)
    await expect(query.totalSupply(null)).to.have.bnToNumber(2)

    await close()
  })

  it('Allowance works', async () => {
    const { query, defaultSigner: sender, alice, tx, close } = await setup()

    const token = IdBuilder.U8(0)


    await expect(query.allowance(sender.address, alice.address, token)).to.have.bnToNumber(0)
    await tx.approve(alice.address, token, 10)
    await expect(query.allowance(sender.address, alice.address, token)).to.have.bnToNumber(10)

    await close()
  })

  it('Approve works', async () => {
    const { contract, query, defaultSigner: sender, alice, close } = await setup()

    const token = IdBuilder.U8(0)

    const tokenAmount = 20

    expect((await query.allowance(sender.address, alice.address, token)).value.unwrapRecursively().toNumber())
      .to.be.eq(0)

    await contract.tx.approve(alice.address, token, tokenAmount)
    expect((await query.allowance(sender.address, alice.address, token)).value.unwrapRecursively().toNumber())
      .to.be.eq(tokenAmount)

    await contract.tx.approve(alice.address, null, 1)
    expect((await query.allowance(sender.address, alice.address, token)).value.unwrapRecursively().toString())
      .to.be.eq('340282366920938463463374607431768211455')

    await close()
  })

  it('Transfer works', async () => {
    const { contract, query, defaultSigner: sender, alice, tx, close } = await setup()

    const token1 = IdBuilder.U8(0)
    const token2 = IdBuilder.U8(1)

    const token1Amount = 1
    const token2Amount = 10
    await tx.mintTokens(token1, token1Amount)
    await tx.mintTokens(token2, token2Amount)

    await expect(query.balanceOf(sender.address, null)).to.have.bnToNumber(2)
    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(token1Amount)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(token2Amount)
    await expect(query.totalSupply(null)).to.have.bnToNumber(2)

    await contract.tx.transfer(alice.address, token2, token2Amount, [])

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(token1Amount)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(token2Amount)
    await expect(query.balanceOf(sender.address, null)).to.have.bnToNumber(1)
    await expect(query.balanceOf(alice.address, null)).to.have.bnToNumber(1)

    await contract.tx.transfer(alice.address, token1, token1Amount, [])
    await contract.withSigner(alice).tx.transfer(sender.address, token2, token1Amount, [])

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(token1Amount)
    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(token1Amount)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(token2Amount - token1Amount)
    await expect(query.balanceOf(sender.address, null)).to.have.bnToNumber(1)
    await expect(query.balanceOf(alice.address, null)).to.have.bnToNumber(2)

    await close()
  })

  it('Transfer from works', async () => {
    const { contract, query, defaultSigner: sender, alice, tx, close } = await setup()

    const token1 = IdBuilder.U8(0)
    const token2 = IdBuilder.U8(1)

    const token1Amount = 1
    const token2Amount = 10

    await tx.mintTokens(token1, token1Amount)
    await tx.mintTokens(token2, token2Amount)

    await contract.withSigner(alice).tx.approve(sender.address, null, 1)
    await contract.tx.transferFrom(sender.address, alice.address, token2, token2Amount, [])

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(token1Amount)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(token2Amount)

    await contract.tx.transferFrom(sender.address, alice.address, token1, token1Amount, [])
    await contract.tx.transferFrom(alice.address, sender.address, token2, token1Amount, [])

    await expect(query.balanceOf(sender.address, token1)).to.have.bnToNumber(0)
    await expect(query.balanceOf(sender.address, token2)).to.have.bnToNumber(token1Amount)
    await expect(query.balanceOf(alice.address, token1)).to.have.bnToNumber(token1Amount)
    await expect(query.balanceOf(alice.address, token2)).to.have.bnToNumber(token2Amount - token1Amount)

    await close()
  })

  it('Transfer from insufficient balance should fail', async () => {
    const { contract, defaultSigner: sender, query, alice, tx, close } = await setup()

    const token = IdBuilder.U8(0)

    const tokenAmount = 1
    await tx.mintTokens(token, tokenAmount)

    await expect(query.balanceOf(sender.address, token)).to.have.bnToNumber(tokenAmount)
    await contract.withSigner(alice).tx.approve(sender.address, token, tokenAmount)

    await expect(contract.tx.transferFrom(sender.address, alice.address, token, tokenAmount + 1, []))
      .to.eventually.be.rejected

    await expect(query.balanceOf(sender.address, token)).to.have.bnToNumber(tokenAmount)

    await close()
  })

  it('Transfer from without allowance should fail', async () => {
    const { contract, defaultSigner: sender, alice, query, tx, close } = await setup()

    const token = IdBuilder.U8(0)

    const tokenAmount = 1
    await tx.mintTokens(token, tokenAmount)

    await expect(query.balanceOf(sender.address, token)).to.have.bnToNumber(tokenAmount)

    await expect(contract.withSigner(alice).tx.transferFrom(sender.address, alice.address, token, tokenAmount, []))
      .to.eventually.be.rejected

    await expect(query.balanceOf(sender.address, token)).to.have.bnToNumber(tokenAmount)

    await close()
  })
})
