import { expect, setupContract } from './../../helpers'

describe('MY_PSP22_FLASHMINT', () => {
    async function setup() {
        let flashmint = await setupContract('my_psp22_flashmint', 'new', '1000')
        let receiver = await setupContract('flash_borrower', 'new')
        return { flashmint, receiver }
    }

    it('New works', async () => {
        const { flashmint } = await setup()

        // flash fee should be 1%
        let fee = await flashmint.query.flashFee(flashmint.contract.address, 100)
        await expect(fee.output!!.toJSON()!!['ok']).to.be.equal(1)
    })

    it('Flashloan works', async () => {
        const { flashmint, receiver } = await setup()
        const { contract: flashmintContract, query: flashmintQuery } = flashmint
        const { contract: receiverContract } = receiver

        let borrowAmount = 100
        let fee = borrowAmount / 100
        let sendAmount = borrowAmount + fee
        let minted = 1000

        // sender has the initial supply of tokens, we send some to the receiver
        await expect(flashmintContract.tx.transfer(receiverContract.address, sendAmount, [])).to.eventually.be.fulfilled
        await expect(flashmintQuery.balanceOf(receiverContract.address)).to.have.output(sendAmount)
        await expect(flashmintQuery.totalSupply()).to.have.output(minted)
        // we call approve from the contract
        await expect(receiverContract.tx.approveToken(flashmintContract.address, flashmintContract.address, sendAmount))
            .to.eventually.be.fulfilled

        // we will do the flashloan
        await expect(flashmintContract.tx.flashloan(receiverContract.address, flashmintContract.address, borrowAmount, []))
            .to.eventually.be.fulfilled

        // reciver should have the fee deducted
        await expect(flashmintQuery.balanceOf(receiverContract.address))
            .to.have.output(sendAmount - fee)
        // one token should be burned
        await expect(flashmintQuery.totalSupply()).to.have.output(minted - fee)
    })

    it('Can not perform the flashloan without allowance for the contract', async () => {
        const { flashmint, receiver } = await setup()
        const { contract: flashmintContract, query: flashmintQuery } = flashmint
        const { contract: receiverContract } = receiver

        let borrowAmount = 100
        let fee = borrowAmount / 100
        let sendAmount = borrowAmount + fee
        let minted = 1000

        // sender has the initial supply of tokens, we send some to the receiver
        await expect(flashmintContract.tx.transfer(receiverContract.address, sendAmount, [])).to.eventually.be.fulfilled
        await expect(flashmintQuery.balanceOf(receiverContract.address)).to.have.output(sendAmount)
        await expect(flashmintQuery.totalSupply()).to.have.output(minted)

        // we perform the flash loan
        await expect(flashmintContract.tx.flashloan(receiverContract.address, flashmintContract.address, borrowAmount, []))
            .to.eventually.be.rejected

        // reciver should have the fee deducted
        await expect(flashmintQuery.balanceOf(receiverContract.address))
            .to.have.output(sendAmount)
        // one token should be burned
        await expect(flashmintQuery.totalSupply()).to.have.output(minted)
    })

})
