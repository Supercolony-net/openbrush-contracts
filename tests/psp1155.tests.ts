import { bnArg, expect, setupContract, fromSigner } from './helpers'

describe('MY_PSP1155', () => {
    async function setup() {
        return setupContract('my_psp1155', 'new', '')
    }

    it('Balance of works', async () => {
        const { contract, query, defaultSigner: sender } = await setup()

        let tokenId = bnArg(1)
        let mintAmount = 1
        await expect(query.balanceOf(sender.address, tokenId)).to.have.output(0)
        await contract.tx.mint(tokenId, mintAmount)
        await expect(query.balanceOf(sender.address, tokenId)).to.have.output(mintAmount)
    })

    it('Balance of batch works', async () => {
        const { contract, query, defaultSigner: sender } = await setup()

        let token1 = bnArg(1)
        let token2 = bnArg(2)
        let token1Amount = 1
        let token2Amount = 20

        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([0, 0])

        await contract.tx.mint(token1, token1Amount)
        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([token1Amount, 0])

        await contract.tx.mintTo(sender.address, token2, token2Amount)
        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([token1Amount, token2Amount])
    })

    it('Set approval works', async () => {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

        await expect(query.isApprovedForAll(sender.address, alice.address))
            .to.have.output(false)

        await contract.tx.setApprovalForAll(alice.address, true)
        await expect(query.isApprovedForAll(sender.address, alice.address))
            .to.have.output(true)

        await contract.tx.setApprovalForAll(alice.address, false)
        await expect(query.isApprovedForAll(sender.address, alice.address))
            .to.have.output(false)
    })

    it('Transfer from single works', async () => {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

        let tokenId = bnArg(1)
        let transferAmount = 1

        await contract.tx.mint(tokenId, transferAmount)
        await contract.tx.safeTransferFrom(sender.address, alice.address, tokenId, transferAmount, [])
        await expect(query.balanceOf(sender.address, tokenId)).to.have.output(0)
        await expect(query.balanceOf(alice.address, tokenId)).to.have.output(transferAmount)

        await fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true)
        await contract.tx.safeTransferFrom(alice.address, sender.address, tokenId, transferAmount, [])
        await expect(query.balanceOf(sender.address, tokenId)).to.have.output(transferAmount)
        await expect(query.balanceOf(alice.address, tokenId)).to.have.output(0)
    })

    it('Transfer from batch works', async () => {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

        let token1 = bnArg(1)
        let token2 = bnArg(2)
        let amount1 = 1
        let amount2 = 10

        await contract.tx.mint(token1, amount1)
        await contract.tx.mint(token2, amount2)
        await contract.tx.safeBatchTransferFrom(sender.address, alice.address, [[token1, amount1], [token2, amount2]], [])
        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([0, 0])
        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([amount1, amount2])

        await fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true)
        await contract.tx.safeBatchTransferFrom(alice.address, sender.address, [[token1, amount1], [token2, amount2]], [])
        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0])
        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([amount1, amount2])
    })

    it('Transfer from single insufficient balance should fail', async () => {
        const { contract, defaultSigner: sender, accounts: [alice] } = await setup()

        let tokenId = bnArg(1)
        let tokenAmount = 1

        await contract.tx.mint(tokenId, tokenAmount)
        await expect(contract.tx.safeTransferFrom(sender.address, alice.address, tokenId, tokenAmount + 1, []))
            .to.eventually.be.rejected
    })

    it('Transfer from single without allowance should fail', async () => {
        const { contract, defaultSigner: sender, accounts: [alice] } = await setup()

        let tokenId = bnArg(1)
        let tokenAmount = 1

        await contract.tx.mintTo(alice.address, tokenId, tokenAmount)
        await expect(contract.tx.safeTransferFrom(alice.address, sender.address, tokenId, tokenAmount, []))
            .to.eventually.be.rejected

    })

    it('Transfer from batch insufficient balance should fail', async () => {
        const { contract, defaultSigner: sender, accounts: [alice] } = await setup()

        let token1 = bnArg(1)
        let token2 = bnArg(2)
        let amount1 = 1
        let amount2 = 10

        await contract.tx.mintTo(alice.address, token1, amount1)
        await contract.tx.mintTo(alice.address, token2, amount2)
        await fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true)
        await expect(contract.tx.safeBatchTransferFrom(alice.address, sender.address, [[token1, amount1 + 1], [token2, amount2]], []))
            .to.eventually.be.rejected

    })

    it('Burn works', async () => {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

        let tokenId = bnArg(1)
        let mintAmount = 1

        await contract.tx.mint(tokenId, mintAmount)
        await contract.tx.mintTo(alice.address, tokenId, mintAmount)
        await fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true)
        await contract.tx.burn(tokenId, mintAmount)
        await contract.tx.burnFrom(alice.address, tokenId, mintAmount)
        await expect(query.balanceOf(sender.address, tokenId)).to.have.output(0)
        await expect(query.balanceOf(alice.address, tokenId)).to.have.output(0)
    })

    it('Burn batch works', async () => {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

        let token1 = bnArg(1)
        let token2 = bnArg(2)
        let amount1 = 1
        let amount2 = 10

        await contract.tx.mint(token1, amount1)
        await contract.tx.mint(token2, amount2)
        await contract.tx.mintTo(alice.address, token1, amount1)
        await contract.tx.mintTo(alice.address, token2, amount2)

        await contract.tx.burnBatch([[token1, amount1], [token2, amount2]], [])
        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([0, 0])

        await fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true)
        await contract.tx.burnBatchFrom(alice.address, [[token1, amount1], [token2, amount2]], [])
        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0])
    })

    it('Burn from without allowance should fail', async () => {
        const { contract, accounts: [alice] } = await setup()

        let token1 = bnArg(1)
        let token2 = bnArg(2)
        let amount1 = 1
        let amount2 = 10

        await contract.tx.mintTo(alice.address, token1, amount1)
        await contract.tx.mintTo(alice.address, token2, amount2)

        await expect(contract.tx.burnBatchFrom(alice.address, [[token1, amount1], [token2, amount2]], []))
            .to.eventually.be.rejected
        await expect(contract.tx.burnFrom(alice.address, token1, amount1, []))
            .to.eventually.be.rejected
    })

    it('Burn from inssuficient balance should fail', async () => {
        const { contract, defaultSigner: sender, accounts: [alice] } = await setup()

        let token1 = bnArg(1)
        let token2 = bnArg(2)
        let amount1 = 1
        let amount2 = 10

        await contract.tx.mint(token1, amount1)
        await contract.tx.mint(token2, amount2)
        await contract.tx.mintTo(alice.address, token1, amount1)
        await contract.tx.mintTo(alice.address, token2, amount2)

        await expect(contract.tx.burnBatch([[token1, amount1 + 1], [token2, amount2]], []))
            .to.eventually.be.rejected
        await expect(contract.tx.burn(token1, amount1 + 1, []))
            .to.eventually.be.rejected

        await fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true)
        await expect(contract.tx.burnBatchFrom(alice.address, [[token1, amount1 + 1], [token2, amount2]], []))
            .to.eventually.be.rejected
        await expect(contract.tx.burnFrom(alice.address, token1, amount1 + 1, []))
            .to.eventually.be.rejected

    })
})
