import { bnArg, expect, setupContract, fromSigner } from './helpers'


describe('MY_PSP721', () => {
    async function setup() {
        return setupContract('my_psp721', 'new', 'Non Fungible Token', 'NFT')
    }

    it('Assigns initial balance', async () => {
        const { query, defaultSigner: sender } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(1)
    })

    it('Transfer changes token balance', async () => {
        const {
            contract,
            defaultSigner: sender,
            accounts: [alice],
            query
        } = await setup()

        // transfer from sender to Alice (token 0 was minted in mypsp_721 constructor)
        await contract.tx.transferFrom(sender.address, alice.address, bnArg(0))
        await contract.tx.mintTo(alice.address, bnArg(1))
        // allow sender to spend Alice's tokens
        await fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true)
        // transfer from Alice to sender
        await contract.tx.transferFrom(alice.address, sender.address, bnArg(1))

        await expect(query.balanceOf(sender.address)).to.have.output(1)
        await expect(query.balanceOf(alice.address)).to.have.output(1)
    })

    it('Can not transfer non-existing token', async () => {
        const {
            contract,
            accounts: [receiver],
            defaultSigner: sender,
        } = await setup()

        let tokenId = bnArg(1)

        await expect(contract.tx.transferFrom(sender.address, receiver.address, tokenId)).to.eventually.be.rejected
    })

    it('Can not transfer without allowance', async () => {
        const {
            contract,
            accounts: [alice],
            defaultSigner: sender,
        } = await setup()

        let tokenId = bnArg(0)

        await expect(fromSigner(contract, alice.address).tx.transferFrom(sender.address, alice.address, tokenId))
            .to.eventually.be.rejected
    })

    it('Mint works', async () => {
        const {
            contract,
            defaultSigner: sender,
            accounts: [alice],
            query
        } = await setup()

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

        // allow ALICE to spend sender's tokens
        await contract.tx.setApprovalForAll(alice.address, true)
        // transfer from Alice back to sender
        await fromSigner(contract, alice.address).tx.burnFrom(sender.address, bnArg(0))
        await expect(query.balanceOf(alice.address)).to.have.output(0)
    })

    it('Mint existing should fail', async () => {
        const {
            contract,
            accounts: [alice],
            query
        } = await setup()

        await expect(contract.tx.mint(bnArg(0))).to.eventually.be.rejected
        await expect(contract.tx.mintTo(alice.address, bnArg(0))).to.eventually.be.rejected
    })

})
