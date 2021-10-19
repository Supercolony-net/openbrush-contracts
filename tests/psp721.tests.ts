import { consts } from './constants'
import { bnArg, expect, setupContract, fromSigner } from './helpers'

import BN from 'bn.js'
import { network } from 'redspot'
import { KeyringPair } from '@polkadot/keyring/types'
import { buildTx } from '@redspot/patract/buildTx'
import { Keyring } from '@polkadot/keyring'

const { api, getAddresses } = network

describe('MY_PSP721', () => {
    async function setup() {
        return setupContract('my_psp721', 'new', 'Non Fungible Token', 'NFT')
    }

    // TODO this is in helper in Mashas PR, dont wanna do merge conflicts so i put it here, will remove it then
    async function addPairWithAmount(pair: KeyringPair): Promise<KeyringPair> {
        const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]))
        const redspotPair = network.addPair(pair)
        await buildTx(api.registry, api.tx.balances.transfer(redspotPair.address, one.muln(10000)), (await getAddresses())[0])
        return redspotPair
    }

    // TODO same 
    async function getSigner(account: string) {
        const signer = await addPairWithAmount(new Keyring().addFromUri(`//${account}`))
        return signer
    }

    it('Assigns initial balance', async () => {
        const { query, defaultSigner: sender } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(1)
    })

    it('Transfer changes token balance', async () => {
        const {
            contract,
            defaultSigner: sender,
        } = await setup()

        const ALICE = await getSigner('Alice')

        // transfer from sender to Alice (token 0 was minted in mypsp_721 constructor)
        await expect(() => contract.tx.transfer_from(sender.address, ALICE.address, 0)).to.changeTokenBalance(contract, ALICE, 1)
        // allow sender to spend Alice's tokens
        await fromSigner(contract, ALICE.address).tx.set_approval_for_all(sender.address, true)
        // transfer from Alice back to sender
        await expect(() => contract.tx.transfer_from(ALICE.address, sender.address, 0)).to.changeTokenBalance(contract, sender, 1)
    })

    it('Can not transfer non-existing token', async () => {
        const {
            contract,
            accounts: [receiver],
            defaultSigner: sender,
        } = await setup()

        await expect(contract.tx.transfer_from(sender.address, receiver.address, 1)).to.eventually.be.rejected
    })

    it('Can not transfer without allowance', async () => {
        const {
            contract,
            accounts: [receiver],
            defaultSigner: sender,
        } = await setup()

        const ALICE = await getSigner('Alice')
        await expect(fromSigner(contract, ALICE.address).tx.transfer_from(sender.address, receiver.address, 0))
            .to.eventually.be.rejected
    })

    it('Mint works', async () => {
        const {
            contract,
            defaultSigner: sender,
        } = await setup()

        const ALICE = await getSigner('Alice')
        await expect(() => contract.tx.mint(1)).to.changeTokenBalance(contract, sender, 1)
        await expect(() => contract.tx.mint_to(ALICE.address, 2)).to.changeTokenBalance(contract, ALICE, 1)
    })

    it('Burn works', async () => {
        const {
            contract,
            defaultSigner: sender,
        } = await setup()

        await expect(() => contract.tx.burn(0)).to.changeTokenBalance(contract, sender, -1)
    })

    it('Burn from works', async () => {
        const {
            contract,
            defaultSigner: sender,
        } = await setup()

        const ALICE = await getSigner('Alice')

        // allow ALICE to spend sender's tokens
        await contract.tx.set_approval_for_all(ALICE.address, true)
        // transfer from Alice back to sender
        await expect(fromSigner(contract, ALICE.address).tx.burn_from(sender.address, 0))
            .to.changeTokenBalance(contract, sender, -1)

    })

    it('Mint existing', async () => {
        const {
            contract,
        } = await setup()

        const ALICE = await getSigner('Alice')
        await expect(() => contract.tx.mint(0)).to.eventually.be.rejected
        await expect(() => contract.tx.mint_to(ALICE.address, 0)).to.eventually.be.rejected
    })

})
