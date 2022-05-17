import { bnArg, expect, setupContract, fromSigner } from '../../helpers'

describe('MY_PSP35_MINTABLE', () => {
    async function setup() {
        return setupContract('my_psp35_mintable', 'new')
    }

    it('Mint works', async () => {
        const { contract, defaultSigner: sender, query, accounts: [alice] } = await setup()

        let token1 = bnArg(1)
        let token2 = bnArg(2)
        let amount1 = 1
        let amount2 = 10

        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([0, 0])
        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0])

        await contract.tx.mint(sender.address, [[token1, amount1]])
        await contract.tx.mint(sender.address, [[token2, amount2]])
        await contract.tx.mint(alice.address, [[token1, amount1], [token2, amount2]])

        await expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([amount1, amount2])
        await expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([amount1, amount2])
    })
})
