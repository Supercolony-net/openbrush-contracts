/* eslint-disable */
import { expect, fromSigner, setupContract, getSigner, setupContractPalletAssets, aliceAddress } from '../../helpers';

describe('MY_PSP22_ASSET_BURNABLE', () => {

    function getRandomInt(max) {
        return Math.floor(Math.random() * max);
    }
    async function setup() {
        const random_asset_id = getRandomInt(10000).toString();
        return setupContractPalletAssets('my_psp22_pallet_asset', 'new', 
        'caller', 
        random_asset_id,
        aliceAddress,
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

    it('Can burn', async () => {
        const { query, contract, defaultSigner: sender } = await setup();

        // Act - Sender mint a token
        await expect(contract.tx.mint(sender.address, 1000)).to.eventually.be.fulfilled
        
        await expect(query.balanceOf(sender.address)).to.have.output(1000);

        // Act - Burn sender's tokens
        await contract.tx.burn(sender.address, 10)

        // Assert - Ensure sender balance is now 990
        await expect(query.balanceOf(sender.address)).to.have.output(990);
    })

    it('Can burn without allowance', async () => {
        const { query, contract, defaultSigner: sender, accounts: [alice] } = await setup();

        // Assert - Ensure sender initial balance is 1000 and allowance is 0
        await expect(contract.tx.mint(sender.address, 1000)).to.eventually.be.fulfilled
        await expect(query.balanceOf(sender.address)).to.have.output(1000);
        await expect(query.allowance(sender.address, alice.address)).to.have.output(0);

        // Act - Burn sender's tokens
        await fromSigner(contract, alice.address).tx.burn(sender.address, 10)

        // Assert - Ensure sender balance is now 990
        await expect(query.balanceOf(sender.address)).to.have.output(990);
    })

    it('Decreases total supply after burning', async () => {
        const { contract, query, defaultSigner: sender } = await setup()

        // Arrange - Ensure initial supply is correct
        await expect(contract.tx.mint(sender.address, 1000)).to.eventually.be.fulfilled
        await expect(query.totalSupply()).to.have.output(1000)

        // Act - Burn token from owner
        await contract.tx.burn(sender.address, 1)

        // Assert - Ensure sender balance is now 999
        await expect(query.totalSupply()).to.have.output(999)
    })

    it('Can burn from', async () => {
        const { contract, query, tx, accounts: [alice, bob] } = await setup();

        // Arrange - Transfer tokens to Alice
        await expect(contract.tx.mint(alice.address, 1000)).to.eventually.be.fulfilled
        await tx.transfer(bob.address, 10, []);
        await expect(query.balanceOf(bob.address)).to.have.output(10);

        // // Act - burn from Alice address
        await tx.burn(bob.address, 10)

        // // Assert - ensure needed amount was burnt
        await expect(query.balanceOf(bob.address)).to.have.output(0);
    })

    it(`Not fails if one of the account's balance exceeds amount to burn`, async () => {
        const { query, tx, contract, accounts: [alice, bob, eve] } = await setup();

        await expect(contract.tx.mint(alice.address, 1000)).to.eventually.be.fulfilled
        // Arrange - Create a signers, transfer tokens to them
        await tx.transfer(eve.address, 10, []);
        await tx.transfer(bob.address, 5, []);

        // Act - burn tokens from Alice and Bob but burnt from Bob more than he own
        await tx.burn(bob.address, 10)
        await tx.burn(eve.address, 10, []);

        // Assert - ensure tokens was not burnt from the accounts
        await expect(query.balanceOf(eve.address)).to.have.output(0);
        await expect(query.balanceOf(bob.address)).to.have.output(0);
    })
})
