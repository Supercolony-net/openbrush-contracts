/* eslint-disable */
import {bnArg, expect, fromSigner, setupContract} from '../../helpers'

describe('MY_PSP22_MINTABLE', () => {
    async function setup() {
        return setupContract('my_psp22_mintable', 'new', '1000')
    }

    it('Assigns initial balance', async () => {
        const { query, defaultSigner: sender } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(1000)
    })

    it('Minting requested amount', async () => {
        const { contract, query, accounts: [alice] } = await setup()

        // Arrange - Ensure receiver balance is 0
        await expect(query.balanceOf(alice.address, bnArg(0))).to.have.output(0)

        // Act - Sender mint a token
        await expect(contract.tx.mint(alice.address, 1)).to.eventually.be.fulfilled

        // Assert - Sender balance is now 1
        await expect(query.balanceOf(alice.address, bnArg(0))).to.have.output(1)
    })

    it('Increases total supply after minting', async () => {
        const { contract, query, defaultSigner: sender } = await setup()

        // Arrange - Ensure initial supply is correct
        await expect(query.totalSupply()).to.have.output(1000)

        // Act - Sender mint a token
        await expect(contract.tx.mint(sender.address, 1)).to.eventually.be.fulfilled

        // Assert - Sender balance is now 1
        await expect(query.totalSupply()).to.have.output(1001)
    })
})
