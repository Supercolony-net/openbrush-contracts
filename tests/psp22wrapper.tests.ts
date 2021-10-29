import { expect, setupContract, fromSigner } from './helpers'

describe('MY_PSP22_WRAPPER', () => {
    async function setup() {
        let contracts = []
        let psp22 = await setupContract('my_psp22', 'new', '1000', 'TOKEN', 'TKN', 2)
        contracts['psp22'] = psp22
        contracts['wrapper'] = await setupContract('my_psp22_wrapper', 'new', psp22.contract.address)
        return contracts
    }

    it('Deposit for works', async () => {
        const contracts = await setup()
        const { contract: wrapper, query: wQuery } = contracts['wrapper']
        const { contract: underlying, query: uQuery, defaultSigner: sender } = contracts['psp22']

        // sender has 1000 tokens
        await expect(uQuery.balanceOf(sender.address)).to.have.output(1000)
        // sender has 0 wrapper tokens
        await expect(wQuery.balanceOf(sender.address)).to.have.output(0)
        // contract has 0 tokens
        await expect(uQuery.balanceOf(wrapper.address)).to.have.output(0)

        // sender allows the contract to spend their tokens
        await expect(underlying.tx.approve(wrapper.address, 100)).to.eventually.be.fulfilled
        // sender wraps 100 tokens
        await expect(fromSigner(wrapper, sender.address).tx.depositFor(sender.address, 100)).to.eventually.be.fulfilled

        // sender has 900 tokens
        await expect(uQuery.balanceOf(sender.address)).to.have.output(900)
        // sender has 100 wrapper tokens
        await expect(wQuery.balanceOf(sender.address)).to.have.output(100)
        // contract has 100 tokens
        await expect(uQuery.balanceOf(wrapper.address)).to.have.output(100)
    })

    it('Deposit without allowance should fail', async () => {
        const contracts = await setup()
        const { contract: wrapper, query: wQuery } = contracts['wrapper']
        const { query: uQuery, defaultSigner: sender } = contracts['psp22']

        await expect(uQuery.balanceOf(sender.address)).to.have.output(1000)
        await expect(wQuery.balanceOf(sender.address)).to.have.output(0)
        await expect(uQuery.balanceOf(wrapper.address)).to.have.output(0)

        await expect(fromSigner(wrapper, sender.address).tx.depositFor(sender.address, 100)).to.eventually.be.rejected

        await expect(uQuery.balanceOf(sender.address)).to.have.output(1000)
        await expect(wQuery.balanceOf(sender.address)).to.have.output(0)
        await expect(uQuery.balanceOf(wrapper.address)).to.have.output(0)
    })

    it('Deposit without underlying tokens should fail', async () => {
        const contracts = await setup()
        const { contract: wrapper, query: wQuery } = contracts['wrapper']
        const { contract: underlying, query: uQuery, defaultSigner: sender } = contracts['psp22']

        await expect(uQuery.balanceOf(sender.address)).to.have.output(1000)
        await expect(wQuery.balanceOf(sender.address)).to.have.output(0)
        await expect(uQuery.balanceOf(wrapper.address)).to.have.output(0)

        await expect(underlying.tx.approve(wrapper.address, 100)).to.eventually.be.fulfilled
        await expect(fromSigner(wrapper, sender.address).tx.depositFor(sender.address, 1001)).to.eventually.be.rejected

        await expect(uQuery.balanceOf(sender.address)).to.have.output(1000)
        await expect(wQuery.balanceOf(sender.address)).to.have.output(0)
        await expect(uQuery.balanceOf(wrapper.address)).to.have.output(0)
    })

    it('Withdraw to works', async () => {
        const contracts = await setup()
        const { contract: wrapper, query: wQuery } = contracts['wrapper']
        const { contract: underlying, query: uQuery, defaultSigner: sender } = contracts['psp22']

        // sender allows the contract to spend their tokens
        await expect(underlying.tx.approve(wrapper.address, 100)).to.eventually.be.fulfilled
        // sender wraps 100 tokens
        await expect(fromSigner(wrapper, sender.address).tx.depositFor(sender.address, 100)).to.eventually.be.fulfilled
        // sender has 900 tokens
        await expect(uQuery.balanceOf(sender.address)).to.have.output(900)
        // sender has 100 wrapper tokens
        await expect(wQuery.balanceOf(sender.address)).to.have.output(100)
        // contract has 100 tokens
        await expect(uQuery.balanceOf(wrapper.address)).to.have.output(100)

        // sender withdraws 100 tokens
        await expect(fromSigner(wrapper, sender.address).tx.withdrawTo(sender.address, 100)).to.eventually.be.fulfilled

        // sender has 1000 tokens
        await expect(uQuery.balanceOf(sender.address)).to.have.output(1000)
        // sender has 0 wrapped tokens
        await expect(wQuery.balanceOf(sender.address)).to.have.output(0)
        // contract has 0 tokens
        await expect(uQuery.balanceOf(wrapper.address)).to.have.output(0)
    })

    it('Withdraw without deposit should fail', async () => {
        const contracts = await setup()
        const { contract: wrapper, query: wQuery } = contracts['wrapper']
        const { query: uQuery, defaultSigner: sender } = contracts['psp22']

        await expect(uQuery.balanceOf(sender.address)).to.have.output(1000)
        await expect(wQuery.balanceOf(sender.address)).to.have.output(0)
        await expect(uQuery.balanceOf(wrapper.address)).to.have.output(0)

        await expect(fromSigner(wrapper, sender.address).tx.withdrawTo(sender.address, 100)).to.eventually.be.rejected

        await expect(uQuery.balanceOf(sender.address)).to.have.output(1000)
        await expect(wQuery.balanceOf(sender.address)).to.have.output(0)
        await expect(uQuery.balanceOf(wrapper.address)).to.have.output(0)
    })

    it('Recover works', async () => {
        const contracts = await setup()
        const { contract: wrapper, query: wQuery } = contracts['wrapper']
        const { contract: underlying, query: uQuery, defaultSigner: sender } = contracts['psp22']

        // sender allows the contract to spend their tokens
        await expect(underlying.tx.approve(wrapper.address, 100)).to.eventually.be.fulfilled
        // sender wraps 100 tokens
        await expect(fromSigner(wrapper, sender.address).tx.depositFor(sender.address, 100)).to.eventually.be.fulfilled
        // sender has 900 tokens
        await expect(uQuery.balanceOf(sender.address)).to.have.output(900)
        // sender has 100 wrapper tokens
        await expect(wQuery.balanceOf(sender.address)).to.have.output(100)
        // contract has 100 tokens
        await expect(uQuery.balanceOf(wrapper.address)).to.have.output(100)

        // sender accidentaly burns 100 tokens
        await expect(fromSigner(wrapper, sender.address).tx.burn(100)).to.eventually.be.fulfilled
        // sender calls recover function
        await expect(fromSigner(wrapper, sender.address).tx.recover()).to.eventually.be.fulfilled

        // sender has 900 tokens
        await expect(uQuery.balanceOf(sender.address)).to.have.output(900)
        // sender has 100 wrapped tokens
        await expect(wQuery.balanceOf(sender.address)).to.have.output(100)
        // contract has 100 tokens
        await expect(uQuery.balanceOf(wrapper.address)).to.have.output(100)
    })

})
