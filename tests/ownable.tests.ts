import {consts} from './constants'
import {bnArg, expect, fromSigner, setupContract} from './helpers'

describe('MY_OWNABLE', () => {
    async function setup() {
        return setupContract('my_ownable', 'new')
    }

    async function setup_receiver() {
        return setupContract('psp1155_receiver', 'new')
    }

    it('PSP 1155 - mint works', async () => {
        const {
            contract,
            query,
            defaultSigner: sender
        } = await setup()

        // Arrange - Ensure sender balance is 0
        await expect(query.balanceOf(sender.address, bnArg(0))).to.have.output(0)

        // Act - Sender mint a token
        await expect(contract.tx.mint(sender.address, bnArg(0), 1)).to.eventually.be.fulfilled

        // Assert - Sender balance is now 1
        await expect(query.balanceOf(sender.address, bnArg(0))).to.have.output(1)
    })

    it('PSP 1155 - burn works', async () => {
        const {
            contract,
            query,
            defaultSigner: sender
        } = await setup()

        // Arrange - Sender mint a token & have a balance of 1
        await expect(contract.tx.mint(sender.address, bnArg(0), 1)).to.eventually.be.fulfilled
        await expect(query.balanceOf(sender.address, bnArg(0))).to.have.output(1)

        // Act - Sender burn the token
        await expect(contract.tx.burn(sender.address, bnArg(0), 1)).to.eventually.be.fulfilled

        // Assert - Sender balance is 0
        await expect(query.balanceOf(sender.address, bnArg(0))).to.have.output(0)
    })

    it('PSP 1155 - transfer works', async () => {
        const {
            contract,
            query,
            defaultSigner: sender,
            accounts: [alice]
        } = await setup()

        // Arrange - Sender mint 100 token
        await expect(contract.tx.mint(sender.address, bnArg(0), 100)).to.eventually.be.fulfilled

        // Act - Sender transfer 50 token to Alice
        await expect(contract.tx.safeTransferFrom(sender.address, alice.address, bnArg(0), 50, 'data')).to.eventually.be.fulfilled

        // Assert - Alice own 50 token
        await expect(query.balanceOf(alice.address, bnArg(0))).to.have.output(50)
    })

    it('PSP 1155 - safe batch transfer works', async () => {
        const {
            tx,
            query,
            defaultSigner: sender
        } = await setup()

        const {
            contract
        } = await setup_receiver()

        // Arrange - Sender mint 100 of token 0 and 100 of token 1
        await expect(tx.mint(sender.address, bnArg(0), 100)).to.eventually.be.fulfilled
        await expect(tx.mint(sender.address, bnArg(1), 100)).to.eventually.be.fulfilled

        // Act - Sender transfer 20 of token 0 and 70 of token 1
        await expect(contract.query.getCallCounter()).to.have.output(0)
        await expect(tx.safeBatchTransferFrom(sender.address, contract.address, [bnArg(0), bnArg(1)], [20, 70], 'data')).to.eventually.be.fulfilled
        await expect(contract.query.getCallCounter()).to.have.output(1)

        // Assert - Receiver own 20 of token 0 and own 70 of token 1
        await expect(query.balanceOfBatch([contract.address, sender.address, contract.address, sender.address],
            [bnArg(0), bnArg(0), bnArg(1), bnArg(1)])).to.have.output([20, 80, 70, 30])
    })

    it('PSP 1155 - receiver can reject the transfer', async () => {
        const {
            tx,
            query,
            defaultSigner: sender
        } = await setup()

        const {
            contract
        } = await setup_receiver()

        // Arrange - Sender mint a token
        await expect(tx.mint(sender.address, bnArg(0), 1)).to.eventually.be.fulfilled

        // Act - Receiver wants to reject the next transfer
        await expect(contract.tx.revertNextTransfer()).to.eventually.be.fulfilled

        // Assert - Sender cannot send token to receiver
        await expect(tx.safeTransferFrom(sender.address, contract.address, bnArg(0), 1, 'data')).to.eventually.be.rejected
        await expect(query.balanceOfBatch([contract.address, sender.address],
            [bnArg(0), bnArg(0)])).to.have.output([0, 1])
    })

    it('PSP 1155 - approved for all works', async () => {
        const {
            contract,
            query,
            defaultSigner: sender,
            accounts: [alice, bob]
        } = await setup()

        // Arrange - Sender mint 2 tokens and Approve Alice to spend on all his tokens
        await expect(contract.tx.mint(sender.address, bnArg(0), 1)).to.eventually.be.fulfilled
        await expect(contract.tx.mint(sender.address, bnArg(1), 1)).to.eventually.be.fulfilled
        await expect(query.balanceOfBatch([sender.address, sender.address],
            [bnArg(0), bnArg(1)])).to.have.output([1, 1])
        await expect(contract.tx.setApprovalForAll(alice.address, true)).to.eventually.be.fulfilled

        // Act - Alice Transfer the two tokens to bob
        await expect(fromSigner(contract, alice.address).tx.safeBatchTransferFrom(sender.address, bob.address, [bnArg(0), bnArg(1)], [1, 1], 'data'))
            .to.eventually.be.fulfilled

        // Assert - Bob owns the two tokens+
        await expect(query.balanceOfBatch([bob.address, bob.address], [bnArg(0), bnArg(1)]))
            .to.have.output([1, 1])

        // Assert - Bob is now owner of the tokens: check that sender cannot transfer tokens on his behalf
        await expect(fromSigner(contract, alice.address).tx.safeBatchTransferFrom(bob.address, alice.address, [bnArg(0), bnArg(1)], [1, 1], 'data'))
            .to.eventually.be.rejected
    })

    it('OWNABLE - owner is by default contract deployer', async () => {
        const {
            query,
            defaultSigner: sender
        } = await setup()

        // Assert - Sender is by default the owner of the contract
        await expect(query.owner()).to.have.output(sender.address)
    })

    it('OWNABLE - only owner is allowed to mint', async () => {
        const {
            contract,
            query,
            defaultSigner: sender,
            accounts: [alice]
        } = await setup()

        // Arrange - Alice is not the owner hence minting should fail
        await expect(query.owner()).to.have.output(sender.address)
        await expect(contract.tx.mint(sender.address, bnArg(0), 1)).to.eventually.be.fulfilled

        // Act & Assert - Alice can mint a token
        await expect(fromSigner(contract, alice.address).tx.mint(alice.address, bnArg(0), 100)).to.eventually.be.rejected
    })

    it('OWNABLE - transfer ownership works', async () => {
        const {
            contract,
            query,
            tx,
            defaultSigner: sender,
            accounts: [alice]
        } = await setup()

        // Arrange - Alice is not the owner hence minting should fail
        await expect(query.owner()).to.have.output(sender.address)
        await expect(fromSigner(contract, alice.address).tx.mint(alice.address, bnArg(0), 100)).to.eventually.be.rejected

        // Act - transfer ownership to Alice
        await tx.transferOwnership(alice.address)
        await expect(query.owner()).to.have.output(alice.address)

        // Assert - Alice can mint a token
        await expect(fromSigner(contract, alice.address).tx.mint(alice.address, bnArg(0), 100)).to.eventually.be.fulfilled
        await expect(query.balanceOf(alice.address, bnArg(0))).to.have.output(100)
    })

    it('OWNABLE - renounce ownership works', async () => {
        const {
            query,
            tx,
            defaultSigner: sender
        } = await setup()

        // Arrange - Sender is the owner
        await expect(query.owner()).to.have.output(sender.address)

        // Act - Sender renounce his role
        await expect(tx.renounceOwnership()).to.eventually.be.fulfilled

        // Assert - Zero account is now the owner
        await expect(query.owner()).to.have.output(consts.EMPTY_ADDRESS)
    })

    it('OWNABLE - cannot renounce ownership if not owner', async () => {
        const {
            contract,
            query,
            defaultSigner: sender,
            accounts: [alice]
        } = await setup()

        // Arrange - Sender is the owner
        await expect(query.owner()).to.have.output(sender.address)

        // Act - Alice try to call renounce his role
        await expect(fromSigner(contract, alice.address).tx.renounceOwnership()).to.eventually.be.rejected

        // Assert - Sender is still the owner
        await expect(query.owner()).to.have.output(sender.address)
    })
})
