import { bnArg, expect, setupContract, fromSigner } from '../../helpers'

describe('MY_PSP721_MINTABLE', () => {
    async function setup() {
        return setupContract('my_psp721_mintable', 'new')
    }

    it('Mint works', async () => {
        const {
            contract,
            defaultSigner: sender,
            accounts: [alice],
            query
        } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(0)
        await expect(query.balanceOf(alice.address)).to.have.output(0)

        await contract.tx.mint(sender.address, bnArg(0))
        await contract.tx.mint(alice.address, bnArg(1))

        await expect(query.balanceOf(sender.address)).to.have.output(1)
        await expect(query.balanceOf(alice.address)).to.have.output(1)
    })

    it('Mint existing should fail', async () => {
        const {
            contract,
            accounts: [alice],
            defaultSigner: sender,
            query
        } = await setup()

        await expect(contract.tx.mint(sender.address, bnArg(0))).to.eventually.be.fulfilled
        await expect(query.balanceOf(sender.address)).to.have.output(1)
        await expect(query.balanceOf(alice.address)).to.have.output(0)

        await expect(contract.tx.mint(sender.address, bnArg(0))).to.eventually.be.rejected
        await expect(contract.tx.mint(alice.address, bnArg(0))).to.eventually.be.rejected

        await expect(query.balanceOf(sender.address)).to.have.output(1)
        await expect(query.balanceOf(alice.address)).to.have.output(0)
    })

    it('Mint to hated account should fail', async () => {
        const {
            contract,
            query,
            tx,
            defaultSigner: hated_account
        } = await setup()
        // Check that we can mint tokens to account which is not hated
        await expect(contract.tx.mint(hated_account.address, bnArg(0))).to.eventually.be.fulfilled
        await expect(query.balanceOf(hated_account.address)).to.have.output(1)

        // Hate account
        await expect(tx.setHatedAccount(hated_account.address)).to.eventually.be.ok
        await expect(query.getHatedAccount()).to.have.output(hated_account.address)

        // Mint must fail
        await expect(contract.tx.mint(hated_account.address, bnArg(1))).to.eventually.be.rejected

        // Amount of tokens must be the same
        await expect(query.balanceOf(hated_account.address)).to.have.output(1)
    })
})
