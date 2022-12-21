/* eslint-disable */
import {expect, getSigners} from '../../helpers';
import exp from 'constants'
import {ApiPromise} from "@polkadot/api";
import ConstructorsPSP22 from "../../../../typechain-generated/constructors/my_psp22_pallet_burnable"
import ContractPSP22 from "../../../../typechain-generated/contracts/my_psp22_pallet_burnable"
import ConstructorsPSP22Receiver from "../../../../typechain-generated/constructors/psp22_receiver";
import ContractPSP22Receiver from "../../../../typechain-generated/contracts/psp22_receiver";

describe('MY_PSP22_PALLET_BURNABLE', () => {
    async function setup() {
        const api = await ApiPromise.create()

        const signers = getSigners()
        const defaultSigner = signers[2]
        const alice = signers[0]
        const bob = signers[1]


        const contractFactory = new ConstructorsPSP22(api, defaultSigner)
        const contractAddress = (await contractFactory.new(Math.floor(Math.random() * 10000) + 1, 1, 1000, {value: '10000000000000000'})).address
        const contract = new ContractPSP22(contractAddress, defaultSigner, api)

        return {
            api,
            defaultSigner,
            alice,
            bob,
            contract,
            query: contract.query,
            tx: contract.tx
        }
    }

    async function setup_receiver() {
        const api = await ApiPromise.create()

        const signers = getSigners()
        const defaultSigner = signers[2]
        const alice = signers[0]
        const bob = signers[1]

        const contractFactory = new ConstructorsPSP22Receiver(api, defaultSigner)
        const contractAddress = (await contractFactory.new()).address
        const contract = new ContractPSP22Receiver(contractAddress, defaultSigner, api)

        return {
            api,
            defaultSigner,
            alice,
            bob,
            contract,
            query: contract.query,
            tx: contract.tx
        }
    }

    it('Assigns initial balance', async () => {
        const { api, query, defaultSigner: sender } = await setup()

        expect((await query.balanceOf(sender.address)).value.unwrapRecursively().toNumber()).to.be.eq(1000)

        await api.disconnect()
    })

    it('Can burn', async () => {
        const { api, query, contract, defaultSigner: sender } = await setup();

        // Assert - Ensure sender initial balance is 1000
        let result = await query.balanceOf(sender.address);
        expect(result.value.unwrapRecursively().toNumber()).to.be.eq(1000);

        // Act - Burn sender's tokens
        await contract.tx.burn(sender.address, 10)

        // Assert - Ensure sender balance is now 990
        await expect(query.balanceOf(sender.address)).to.have.bnToNumber(990)

        await api.disconnect()
    })

    it('Can burn without allowance', async () => {
        const { api, query, contract, defaultSigner: sender, alice } = await setup();

        // Assert - Ensure sender initial balance is 1000 and allowance is 0
        await expect(query.balanceOf(sender.address)).to.have.bnToNumber(1000);
        await expect(query.allowance(sender.address, alice.address)).to.have.bnToNumber(0);

        // Act - Burn sender's tokens
        await contract.withSigner(alice).tx.burn(sender.address, 10)

        // Assert - Ensure sender balance is now 990
        await expect(query.balanceOf(sender.address)).to.have.bnToNumber(990);

        await api.disconnect()
    })

    it('Decreases total supply after burning', async () => {
        const { api, contract, query, defaultSigner: sender } = await setup()

        // Arrange - Ensure initial supply is correct
        await expect(query.totalSupply()).to.have.bnToNumber(1000)

        // Act - Burn token from owner
        await contract.tx.burn(sender.address, 1)

        // Assert - Ensure sender balance is now 999
        await expect(query.totalSupply()).to.have.bnToNumber(999)

        await api.disconnect()
    })

    it('Can burn from', async () => {
        const { api, query, tx, alice } = await setup();

        // Arrange - Transfer tokens to Alice
        await tx.transfer(alice.address, 10, [])

        // Act - burn from Alice address
        await tx.burn(alice.address, 10)

        // Assert - ensure needed amount was burnt
        await expect(query.balanceOf(alice.address)).to.have.bnToNumber(0)

        await api.disconnect()
    })

    it('Can burn from many', async () => {
        const { api, query, tx, contract, alice, bob } = await setup();

        await tx.transfer(alice.address, 10, [])
        await tx.transfer(bob.address, 10, [])

        // Act - burn tokens from Alice and Bob
        await contract.tx.burnFromMany([[alice.address, 10], [bob.address, 10]])

        // Assert - ensure needed amount was burnt
        await expect(query.balanceOf(alice.address)).to.have.bnToNumber(0)
        await expect(query.balanceOf(bob.address)).to.have.bnToNumber(0)

        await api.disconnect()
    })
})
