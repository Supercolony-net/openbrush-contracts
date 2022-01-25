import { bnArg, expect, setupContract, fromSigner } from '../../helpers'
import {consts} from '../../constants'

describe('MY_PSP721_BURNABLE', () => {
    async function setup() {
        return setupContract('my_psp721_burnable', 'new')
    }

    it('Burn works', async () => {
        const {
            contract,
            defaultSigner: sender,
            query
        } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(3)

        await contract.tx.burn(sender.address, bnArg(0))

        await expect(query.balanceOf(sender.address)).to.have.output(2)
    })

    it('Burn from works', async () => {
        const {
            contract,
            defaultSigner: sender,
            accounts: [alice],
            query
        } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(3)
        await contract.tx.setApprovalForAll(alice.address, true)

        await fromSigner(contract, alice.address).tx.burn(sender.address, bnArg(0))

        await expect(query.balanceOf(sender.address)).to.have.output(2)
    })

    it('Burn from without allowance should fail', async () => {
        const {
            contract,
            defaultSigner: sender,
            accounts: [alice],
            query
        } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(3)

        await expect(fromSigner(contract, alice.address).tx.burn(sender.address, bnArg(0)))
            .to.eventually.be.rejected

        await expect(query.balanceOf(sender.address)).to.have.output(3)
    })

    it('Burn from hated account should fail', async () => {
        const {
            contract,
            query,
            tx,
            defaultSigner: hated_account
        } = await setup()
        // Check that we can burn tokens while account is not hated
        await expect(query.balanceOf(hated_account.address)).to.have.output(3)
        await expect(fromSigner(contract, hated_account.address).tx.burn(hated_account.address, bnArg(0)))
          .to.eventually.be.fulfilled
        await expect(query.balanceOf(hated_account.address)).to.have.output(2)

        // Hate account
        await expect(tx.setHatedAccount(hated_account.address)).to.eventually.be.ok
        await expect(query.getHatedAccount()).to.have.output(hated_account.address)

        // Burn must fail
        await expect(fromSigner(contract, hated_account.address).tx.burn(hated_account.address, bnArg(1)))
          .to.eventually.be.rejected

        // Amount of tokens must be the same
        await expect(query.balanceOf(hated_account.address)).to.have.output(2)
    })
})
