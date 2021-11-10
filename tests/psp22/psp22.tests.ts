import { consts } from '../constants'
import { expect, setupContract } from '../helpers'

describe('MY_PSP22', () => {
  async function setup() {
    return setupContract('my_psp22', 'new', '1000')
  }

  async function setup_receiver() {
    return setupContract('psp22_receiver', 'new')
  }

  it('Assigns initial balance', async () => {
    const { query, defaultSigner: sender } = await setup()

    await expect(query.balanceOf(sender.address)).to.have.output(1000)
  })

  it('Transfer adds amount to destination account', async () => {
    const {
      contract,
      accounts: [receiver]
    } = await setup()

    await expect(() => contract.tx.transfer(receiver.address, 7, [])).to.changeTokenBalance(contract, receiver, 7)
    await expect(() => contract.tx.transfer(receiver.address, 7, [])).to.changeTokenBalances(contract, [contract.signer, receiver], [-7, 7])
  })

  it('Transfers funds successfully if destination account is a receiver and supports transfers', async () => {
    const { tx } = await setup()

    const { contract } = await setup_receiver()

    await expect(tx.transfer(contract.address, 7, [])).to.eventually.be.fulfilled
  })

  it('Transfers funds successfully if destination account is a receiver a contract but not PSP22Receiver', async () => {
    const { tx } = await setup()

    const { contract } = await setup()

    await expect(tx.transfer(contract.address, 7, [])).to.eventually.be.fulfilled
  })

  it('Can not transfer above the amount', async () => {
    const {
      contract,
      accounts: [receiver]
    } = await setup()

    await expect(contract.tx.transfer(receiver.address, 1007, [])).to.eventually.be.rejected
  })

  it('Can not transfer to hated account', async () => {
    const {
      query,
      tx,
      accounts: [hated_account]
    } = await setup()

    // Check that we can transfer money while account is not hated
    await expect(tx.transfer(hated_account.address, 10, [])).to.eventually.be.fulfilled
    let result = await query.balanceOf(hated_account.address)
    expect(result.output).to.equal(10)
    await expect(query.getHatedAccount()).to.have.output(consts.EMPTY_ADDRESS)

    // Hate account
    await expect(tx.setHatedAccount(hated_account.address)).to.eventually.be.ok
    await expect(query.getHatedAccount()).to.have.output(hated_account.address)

    // Transfer must fail
    await expect(tx.transfer(hated_account.address, 10, [])).to.eventually.be.rejected

    // Amount of tokens must be the same
    result = await query.balanceOf(hated_account.address)
    expect(result.output).to.equal(10)
  })
})
