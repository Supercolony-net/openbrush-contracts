import { bnArg, expect, setupContract, fromSigner } from '../../helpers'

describe('MY_PSP1155_BURNABLE', () => {
    async function setup() {
        return setupContract('my_psp1155_burnable', 'new')
    }

    it('Burn works', async () => {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

        let tokenId = bnArg(0)
        let tokenId2 = bnArg(1)
        let mintAmount = 1
        let mintAmount2 = 20

        await contract.tx.transferFrom(sender.address, alice.address, tokenId, mintAmount, [])
        await expect(query.balanceOf(alice.address, tokenId)).to.have.output(mintAmount)
        await expect(query.balanceOf(sender.address, tokenId2)).to.have.output(mintAmount2)
        await fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true)

        await contract.tx.burn(sender.address, [[tokenId2, mintAmount2]])
        await contract.tx.burn(alice.address, [[tokenId, mintAmount]])

        await expect(query.balanceOf(sender.address, tokenId)).to.have.output(0)
        await expect(query.balanceOf(alice.address, tokenId2)).to.have.output(0)
    })

    it('Burn batch works', async () => {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = await setup()

        let token1 = bnArg(0)
        let token2 = bnArg(1)
        let amount1 = 1
        let amount2 = 10

        await contract.tx.transferFrom(sender.address, alice.address, token2, amount2, [])
        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([amount1, amount2])
        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, amount2])

        await contract.tx.burn(sender.address, [[token1, amount1], [token2, amount2]])
        await fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true)
        await contract.tx.burn(alice.address, [[token1, 0], [token2, amount2]])

        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([0, 0])
        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0])
    })

    it('Burn from without allowance should fail', async () => {
        const { contract, accounts: [alice], query, defaultSigner: sender } = await setup()

        let token1 = bnArg(0)
        let token2 = bnArg(1)
        let amount1 = 1
        let amount2 = 20

        await contract.tx.transferFrom(sender.address, alice.address, token1, amount1, [])
        await contract.tx.transferFrom(sender.address, alice.address, token2, amount2, [])
        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([amount1, amount2])

        await expect(contract.tx.burn(alice.address, [[token1, amount1], [token2, amount2]]))
            .to.eventually.be.rejected
        await expect(contract.tx.burn(alice.address, [[token1, amount1]]))
            .to.eventually.be.rejected

        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([amount1, amount2])
    })

    it('Burn inssuficient balance should fail', async () => {
        const { contract, defaultSigner: sender, query, accounts: [alice] } = await setup()

        let token1 = bnArg(0)
        let token2 = bnArg(1)
        let amount1 = 1
        let amount2 = 20

        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([amount1, amount2])
        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0])

        await expect(contract.tx.burn(sender.address, [[token1, amount1 + 1], [token2, amount2]]))
            .to.eventually.be.rejected
        await expect(contract.tx.burn(sender.address, [[token1, amount1 + 1]]))
            .to.eventually.be.rejected

        await fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true)
        await expect(contract.tx.burn(alice.address, [[token1, amount1 + 1], [token2, amount2]]))
            .to.eventually.be.rejected
        await expect(contract.tx.burn(alice.address, [[token1, amount1 + 1]]))
            .to.eventually.be.rejected

        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([amount1, amount2])
        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0])
    })

    it('Burn from hated account should fail', async () => {
        const {contract, defaultSigner: hated_account, query, tx} = await setup()

        let token1 = bnArg(0)
        let token2 = bnArg(1)
        let amount1 = 1
        let amount2 = 20

        await expect(query.balanceOfBatch([[hated_account.address, token1], [hated_account.address, token2]]))
          .to.have.output([amount1, amount2])

        // Check that we can burn token from account which is not hated
        await expect(fromSigner(contract, hated_account.address).tx.burn(hated_account.address, [[token1, amount1]]))
          .to.eventually.be.fulfilled

        // Hate account
        await expect(tx.setHatedAccount(hated_account.address)).to.eventually.be.ok
        await expect(query.getHatedAccount()).to.have.output(hated_account.address)

        // Burn must failed
        await expect(fromSigner(contract, hated_account.address).tx.burn(hated_account.address, [[token2, amount2]]))
          .to.eventually.be.rejected

        // Amount of tokens must be the same
        await expect(query.balanceOfBatch([[hated_account.address, token1], [hated_account.address, token2]]))
          .to.have.output([0, amount2])
    })
})
