import { bnArg, expect, setupContract, fromSigner } from '../../helpers'

describe('MY_PSP34_MINTABLE', () => {
    async function setup() {
        return setupContract('my_psp34_mintable', 'new')
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

        await contract.tx.mint(bnArg(0))
        await contract.tx.mintTo(alice.address, bnArg(1))

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

        await expect(contract.tx.mint(bnArg(0))).to.eventually.be.fulfilled
        await expect(query.balanceOf(sender.address)).to.have.output(1)
        await expect(query.balanceOf(alice.address)).to.have.output(0)

        await expect(contract.tx.mint(bnArg(0))).to.eventually.be.rejected
        await expect(contract.tx.mintTo(alice.address, bnArg(0))).to.eventually.be.rejected

        await expect(query.balanceOf(sender.address)).to.have.output(1)
        await expect(query.balanceOf(alice.address)).to.have.output(0)
    })

})
