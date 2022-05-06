/* eslint-disable */
import {bnArg, expect, fromSigner, setupContractPalletAssets, setupContract} from '../../helpers'

describe('MY_PSP22_ASSET_MINTABLE', () => {

    function getRandomInt(max) {
        return Math.floor(Math.random() * max);
    }

    async function setup() {
        const random_asset_id = getRandomInt(10000).toString();
        return setupContractPalletAssets('my_psp22_pallet_asset', 'new', 
        'caller', 
        random_asset_id,
        '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d',
        '1',
        'name',
        'symbol',
        '12'
        )
    }

    it('Assigns initial balance', async () => {
        const { query, defaultSigner: sender } = await setup()
        
        await expect(query.balanceOf(sender.address)).to.have.output(0)
    })

    it('Minting requested amount', async () => {
        const { contract, query, accounts: [alice],  } = await setup()

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
        await expect(query.totalSupply()).to.have.output(0)

        // Act - Sender mint a token
        await expect(contract.tx.mint(sender.address, 1)).to.eventually.be.fulfilled

        // Assert - Sender balance is now 1
        await expect(query.totalSupply()).to.have.output(1)
    })
})
