import { bnArg, expect, setupContract, fromSigner } from './helpers'

describe('MY_PSP721', () => {
    async function setup() {
        return setupContract('my_psp721', 'new', 'Non Fungible Token', 'NFT')
    }

    async function setup_receiver() {
        return setupContract('psp721_receiver', 'new')
    }

    it('Assigns initial balance', async () => {
        const { query, defaultSigner: sender } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(1)
    })

    it('Metadata works', async () => {
        const { query } = await setup()

        await expect(query.name()).to.have.output('Non Fungible Token')
        await expect(query.symbol()).to.have.output('NFT')
    })

    it('Transfer changes token balance', async () => {
        const {
            contract,
            defaultSigner: sender,
            accounts: [alice],
            query
        } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(1)
        await contract.tx.mintTo(alice.address, bnArg(1))
        await expect(query.balanceOf(alice.address)).to.have.output(1)
        await fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true)

        await contract.tx.transferFrom(sender.address, alice.address, bnArg(0))
        await contract.tx.transferFrom(alice.address, sender.address, bnArg(1))

        await expect(query.balanceOf(sender.address)).to.have.output(1)
        await expect(query.balanceOf(alice.address)).to.have.output(1)
    })

    it('PSP 721 - safe transfer works', async () => {
        const {
            tx,
            query,
            defaultSigner: sender,
        } = await setup()

        const { contract } = await setup_receiver()

        // Arrange - Sender mint a Token and Approve Receiver as spender of this token
        await expect(query.ownerOf(bnArg(0))).to.have.output(sender.address)

        // Act - Alice transfer the token form sender to bob
        await expect(contract.query.getCallCounter()).to.have.output(0)
        await expect(tx.safeTransferFrom(sender.address, contract.address, bnArg(0), 'data')).to.eventually.be.fulfilled
        await expect(contract.query.getCallCounter()).to.have.output(1)

        // Assert - Bob is now owner of the token
        await expect(query.ownerOf(bnArg(0))).to.have.output(contract.address.toString())
    })

    it('PSP 721 - receiver can reject the transfer', async () => {
        const { tx, query, defaultSigner: sender } = await setup()

        const { contract } = await setup_receiver()

        // Arrange - Sender mint a token
        await expect(query.ownerOf(bnArg(0))).to.have.output(sender.address)

        // Act - Receiver wants to reject the next transfer
        await expect(contract.tx.revertNextTransfer()).to.eventually.be.fulfilled

        // Assert - Sender cannot send token to receiver & Sender still own the token
        await expect(tx.safeTransferFrom(sender.address, contract.address, bnArg(0), 'data')).to.eventually.be.rejected
        await expect(query.ownerOf(bnArg(0))).to.have.output(sender.address)
    })

    it('Can not transfer non-existing token', async () => {
        const {
            contract,
            accounts: [receiver],
            defaultSigner: sender,
            query
        } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(1)

        await expect(contract.tx.transferFrom(sender.address, receiver.address, bnArg(1))).to.eventually.be.rejected

        await expect(query.balanceOf(sender.address)).to.have.output(1)
    })

    it('Can not transfer without allowance', async () => {
        const {
            contract,
            accounts: [alice],
            defaultSigner: sender,
            query,
        } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(1)

        await expect(fromSigner(contract, alice.address).tx.transferFrom(sender.address, alice.address, bnArg(0)))
            .to.eventually.be.rejected

        await expect(query.balanceOf(sender.address)).to.have.output(1)
    })

    it('Mint works', async () => {
        const {
            contract,
            defaultSigner: sender,
            accounts: [alice],
            query
        } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(1)
        await expect(query.balanceOf(alice.address)).to.have.output(0)

        await contract.tx.mint(bnArg(1))
        await contract.tx.mintTo(alice.address, bnArg(2))

        await expect(query.balanceOf(sender.address)).to.have.output(2)
        await expect(query.balanceOf(alice.address)).to.have.output(1)
    })

    it('Burn works', async () => {
        const {
            contract,
            defaultSigner: sender,
            query
        } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(1)

        await contract.tx.burn(bnArg(0))

        await expect(query.balanceOf(sender.address)).to.have.output(0)
    })

    it('Burn from works', async () => {
        const {
            contract,
            defaultSigner: sender,
            accounts: [alice],
            query
        } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(1)
        await contract.tx.setApprovalForAll(alice.address, true)

        await fromSigner(contract, alice.address).tx.burnFrom(sender.address, bnArg(0))

        await expect(query.balanceOf(alice.address)).to.have.output(0)
    })

    it('Mint existing should fail', async () => {
        const {
            contract,
            accounts: [alice],
            defaultSigner: sender,
            query
        } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(1)
        await expect(query.balanceOf(alice.address)).to.have.output(0)

        await expect(contract.tx.mint(bnArg(0))).to.eventually.be.rejected
        await expect(contract.tx.mintTo(alice.address, bnArg(0))).to.eventually.be.rejected

        await expect(query.balanceOf(sender.address)).to.have.output(1)
        await expect(query.balanceOf(alice.address)).to.have.output(0)
    })

})
