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
        await contract.tx.burn(alice.address, [[token1, 0], [token2, amount2]])

        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([0, 0])
        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0])
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

        await expect(contract.tx.burn(alice.address, [[token1, amount1 + 1], [token2, amount2]]))
            .to.eventually.be.rejected
        await expect(contract.tx.burn(alice.address, [[token1, amount1 + 1]]))
            .to.eventually.be.rejected

        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([amount1, amount2])
        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0])
    })
})
