import { bnArg, expect, setupContract, fromSigner } from '../../helpers'

describe('MY_PSP1155_MINTABLE', () => {
    async function setup() {
        return setupContract('my_psp1155_mintable', 'new')
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

    it('Can not mint to hated account', async () => {
        const {contract, defaultSigner: hated_account, query, tx} = await setup()

        let token1 = bnArg(0)
        let token2 = bnArg(1)
        let amount1 = 1
        let amount2 = 10

        await expect(query.balanceOfBatch([[hated_account.address, token1], [hated_account.address, token2]]))
          .to.have.output([0, 0])

        // Check that we can mint token to account which is not hated
        await expect(contract.tx.mint(hated_account.address, [[token1, amount1]])).to.eventually.be.fulfilled
        await expect(query.balanceOfBatch([[hated_account.address, token1], [hated_account.address, token2]]))
          .to.have.output([amount1, 0])

        // Hate account
        await expect(tx.setHatedAccount(hated_account.address)).to.eventually.be.ok
        await expect(query.getHatedAccount()).to.have.output(hated_account.address)

        // Mint must failed
        await expect(contract.tx.mint(hated_account.address, [[token2, amount2]])).to.eventually.be.rejected

        // Amount of tokens must be the same
        await expect(query.balanceOfBatch([[hated_account.address, token1], [hated_account.address, token2]]))
          .to.have.output([amount1, 0])
    })
})
