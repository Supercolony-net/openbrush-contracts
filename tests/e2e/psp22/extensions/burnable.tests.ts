/* eslint-disable */
import { expect, fromSigner, setupContract, getSigner } from '../../helpers';
import exp from 'constants'

describe('MY_PSP22_BURNABLE', () => {
    async function setup() {
        return setupContract('my_psp22_burnable', 'new', '1000')
    }

    it('Assigns initial balance', async () => {
        const { query, defaultSigner: sender } = await setup()

        expect(query.balanceOf(sender.address)).to.have.output(1000)
    })

    it('Can burn', async () => {
        const { query, contract, defaultSigner: sender } = await setup();

        // Assert - Ensure sender initial balance is 1000
        await expect(query.balanceOf(sender.address)).to.have.output(1000);

        // Act - Burn sender's tokens
        await expect(contract.tx.burn(sender.address, 10)).to.eventually.be.fulfilled

        // Assert - Ensure sender balance is now 990
        await expect(query.balanceOf(sender.address)).to.have.output(990)
    })

    it('Can burn without allowance', async () => {
        const { query, contract, defaultSigner: sender, accounts: [alice] } = await setup();

        // Assert - Ensure sender initial balance is 1000 and allowance is 0
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
        await expect(query.totalSupply()).to.have.output(1000)

        // Act - Burn token from owner
        await expect(contract.tx.burn(sender.address, 1)).to.eventually.be.fulfilled

        // Assert - Ensure sender balance is now 999
        await expect(query.totalSupply()).to.have.output(999)
    })

    it('Can burn from', async () => {
        const { query, tx, accounts: [alice] } = await setup();

        // Arrange - Transfer tokens to Alice
        await expect(tx.transfer(alice.address, 10, [])).to.eventually.be.fulfilled

        // Act - burn from Alice address
        await expect(tx.burn(alice.address, 10)).to.eventually.be.fulfilled

        // Assert - ensure needed amount was burnt
        await expect(query.balanceOf(alice.address)).to.have.output(0)
    })

    it('Can burn from many', async () => {
        const { query, tx, contract } = await setup();

        // Arrange - Create a signers, transfer tokens to them
        const alice = await getSigner('Alice')
        const bob = await getSigner('Bob')
        await expect(tx.transfer(alice.address, 10, [])).to.eventually.be.fulfilled
        await expect(tx.transfer(bob.address, 10, [])).to.eventually.be.fulfilled

        // Act - burn tokens from Alice and Bob
        await expect(contract.tx.burnFromMany([[alice.address, 10], [bob.address, 10]])).to.eventually.be.fulfilled

        // Assert - ensure needed amount was burnt
        await expect(query.balanceOf(alice.address)).to.have.output(0)
        await expect(query.balanceOf(bob.address)).to.have.output(0)
    })

    it(`Fails if one of the account's balance exceeds amount to burn`, async () => {
        const { query, tx, contract } = await setup();

        // Arrange - Create a signers, transfer tokens to them
        const alice = await getSigner('Alice')
        const bob = await getSigner('Bob')
        await expect(tx.transfer(alice.address, 10, [])).to.eventually.be.fulfilled
        await expect(tx.transfer(bob.address, 5, [])).to.eventually.be.fulfilled

        // Act - burn tokens from Alice and Bob but burnt from Bob more than he own
        await expect(contract.tx.burnFromMany([[alice.address, 10], [bob.address, 10]])).to.eventually.be.rejected

        // Assert - ensure tokens was not burnt from the accounts
        await expect(query.balanceOf(alice.address)).to.have.output(10);
        await expect(query.balanceOf(bob.address)).to.have.output(5);
    })
})
