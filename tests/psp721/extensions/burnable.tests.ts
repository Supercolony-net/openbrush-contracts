import { bnArg, expect, setupContract, fromSigner } from '../../helpers'

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

    it('Burn from without allowance works', async () => {
        const {
            contract,
            defaultSigner: sender,
            accounts: [alice],
            query
        } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(3)

        await expect(fromSigner(contract, alice.address).tx.burn(sender.address, bnArg(0)))
            .to.eventually.be.fulfilled

        await expect(query.balanceOf(sender.address)).to.have.output(2)
    })
})
