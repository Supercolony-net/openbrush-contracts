import { consts } from './constants'
import { bnArg, expect, setupContract, fromSigner } from './helpers'

import BN from 'bn.js'
import { network } from 'redspot'
import { KeyringPair } from '@polkadot/keyring/types'
import { buildTx } from '@redspot/patract/buildTx'
import { Keyring } from '@polkadot/keyring'

const { api, getAddresses } = network

describe('MY_PSP1155', () => {
    async function setup() {
        return setupContract('my_psp1155', 'new', '')
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

    it('Balance of works', async () => {
        const { query, contract, defaultSigner: sender } = await setup()

        let tokenId = 1
        let mintAmount = 1
        await expect(() => contract.tx.balance_of(sender.address, tokenId)).to.have.output(0)
        await contract.tx.mint_to(sender.address, tokenId, mintAmount)
        await expect(() => contract.tx.balance_of(sender.address, tokenId)).to.have.output(mintAmount)
    })

    it('Balance of batch works', async () => {
        const { query, contract, defaultSigner: sender } = await setup()

        let token1 = 1
        let token2 = 2
        let token1Amount = 1
        let token2Amount = 20

        await expect(() => contract.tx.balance_of_batch([sender.address, sender.address], [token1, token2]))
            .to.have.output([0, 0])

        await contract.tx.mint_to(sender.address, token1, token1Amount)
        await expect(() => contract.tx.balance_of_batch([sender.address, sender.address], [token1, token2]))
            .to.have.output([token1Amount, 0])

        await contract.tx.mint_to(sender.address, token2, token2Amount)
        await expect(() => contract.tx.balance_of_batch([sender.address, sender.address], [token1, token2]))
            .to.have.output([token1Amount, token2Amount])
    })

    it('Set approval works', async () => {
        const { query, contract, defaultSigner: sender } = await setup()

        const BOB = await getSigner('Bob')
        await expect(() => contract.tx.is_approved_for_all(sender.address, BOB.address))
            .to.have.output(false)

        await contract.tx.set_approval_for_all(BOB.address, true)
        await expect(() => contract.tx.is_approved_for_all(sender.address, BOB.address))
            .to.have.output(true)

        await contract.tx.set_approval_for_all(BOB.address, false)
        await expect(() => contract.tx.is_approved_for_all(sender.address, BOB.address))
            .to.have.output(false)
    })

    it('Transfer from single works', async () => {
        const { query, contract, defaultSigner: sender } = await setup()

        let tokenId = 1
        let transferAmount = 1

        const BOB = await getSigner('Bob')

        await contract.tx.mint_to(sender.address, tokenId, transferAmount)
        await contract.tx.safe_transfer_from(sender.address, BOB.address, tokenId, transferAmount, [])
        await expect(() => contract.tx.balance_of(sender.address, tokenId)).to.have.output(0)
        await expect(() => contract.tx.balance_of(BOB.address, tokenId)).to.have.output(transferAmount)

        await fromSigner(contract, BOB.address).tx.set_approval_for_all(sender.address, true)
        await contract.tx.safe_transfer_from(BOB.address, sender.address, tokenId, transferAmount, [])
        await expect(() => contract.tx.balance_of(sender.address, tokenId)).to.have.output(transferAmount)
        await expect(() => contract.tx.balance_of(BOB.address, tokenId)).to.have.output(0)
    })

    it('Transfer from batch works', async () => {
        const { query, contract, defaultSigner: sender } = await setup()

        let token1 = 1
        let token2 = 2
        let amount1 = 1
        let amount2 = 10

        const BOB = await getSigner('Bob')

        await contract.tx.mint_to(sender.address, token1, amount1)
        await contract.tx.mint_to(sender.address, token2, amount2)
        await contract.tx.safe_transfer_from(sender.address, BOB.address, [token1, token2], [amount1, amount2], []);
        await expect(() => contract.tx.balance_of_batch([sender.address, sender.address], [token1, token2]))
            .to.have.output([0, 0])
        await expect(() => contract.tx.balance_of_batch([BOB.address, BOB.address], [token1, token2]))
            .to.have.output([amount1, amount2])

        await fromSigner(contract, BOB.address).tx.set_approval_for_all(sender.address, true)
        await contract.tx.safe_transfer_from(BOB.address, sender.address, [token1, token2], [amount1, amount2], []);
        await expect(() => contract.tx.balance_of_batch([BOB.address, BOB.address], [token1, token2]))
            .to.have.output([0, 0])
        await expect(() => contract.tx.balance_of_batch([sender.address, sender.address], [token1, token2]))
            .to.have.output([amount1, amount2])
    })

    it('Transfer from single insufficient balance should fail', async () => {
        const { query, contract, defaultSigner: sender } = await setup()

        let tokenId = 1
        let tokenAmount = 1

        const BOB = await getSigner('Bob')

        await contract.tx.mint_to(sender.address, tokenId, tokenAmount)
        await expect(() => contract.tx.safe_transfer_from(sender.address, BOB.address, tokenId, tokenAmount + 1, []))
            .to.eventually.be.rejected
    })

    it('Transfer from single without allowance should fail', async () => {
        const { query, contract, defaultSigner: sender } = await setup()

        let tokenId = 1
        let tokenAmount = 1

        const BOB = await getSigner('Bob')

        await contract.tx.mint_to(BOB.address, tokenId, tokenAmount)
        await expect(() => contract.tx.safe_transfer_from(BOB.address, sender.address, tokenId, tokenAmount, []))
            .to.eventually.be.rejected

    })

    it('Transfer from batch insufficient balance should fail', async () => {
        const { query, contract, defaultSigner: sender } = await setup()

        let token1 = 1
        let token2 = 2
        let amount1 = 1
        let amount2 = 10

        const BOB = await getSigner('Bob')

        await contract.tx.mint_to(BOB.address, token1, amount1)
        await contract.tx.mint_to(BOB.address, token2, amount2)
        await fromSigner(contract, BOB.address).tx.set_approval_for_all(sender.address, true)
        await expect(() => contract.tx.safe_transfer_from(BOB.address, sender.address, [token1, token2], [amount1 + 1, amount2], []))
            .to.eventually.be.rejected

    })

    it('Burn works', async () => {
        const { query, contract, defaultSigner: sender } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.output(1)

        let tokenId = 1
        let mintAmount = 1

        const BOB = await getSigner('Bob')

        await contract.tx.mint_to(sender.address, tokenId, mintAmount)
        await contract.tx.mint_to(BOB.address, tokenId, mintAmount)
        await fromSigner(contract, BOB.address).tx.set_approval_for_all(sender.address, true)
        await contract.tx.burn(tokenId, mintAmount)
        await contract.tx.burn_from(BOB.address, tokenId, mintAmount)
        await expect(() => contract.tx.balance_of(sender.address, tokenId)).to.have.output(0)
        await expect(() => contract.tx.balance_of(BOB.address, tokenId)).to.have.output(0)
    })

    it('Burn batch works', async () => {
        const { query, contract, defaultSigner: sender } = await setup()

        let token1 = 1
        let token2 = 2
        let amount1 = 1
        let amount2 = 10

        const BOB = await getSigner('Bob')

        await contract.tx.mint_to(sender.address, token1, amount1)
        await contract.tx.mint_to(sender.address, token2, amount2)
        await contract.tx.mint_to(BOB.address, token1, amount1)
        await contract.tx.mint_to(BOB.address, token2, amount2)

        await contract.tx.burn_batch([token1, token2], [amount1, amount2], []);
        await expect(() => contract.tx.balance_of_batch([sender.address, sender.address], [token1, token2]))
            .to.have.output([0, 0])

        await fromSigner(contract, BOB.address).tx.set_approval_for_all(sender.address, true)
        await contract.tx.burn_batch_from(BOB.address, [token1, token2], [amount1, amount2], []);
        await expect(() => contract.tx.balance_of_batch([BOB.address, BOB.address], [token1, token2]))
            .to.have.output([0, 0])
    })

    it('Burn from without allowance should fail', async () => {
        const { query, contract, defaultSigner: sender } = await setup()

        let token1 = 1
        let token2 = 2
        let amount1 = 1
        let amount2 = 10

        const BOB = await getSigner('Bob')

        await contract.tx.mint_to(BOB.address, token1, amount1)
        await contract.tx.mint_to(BOB.address, token2, amount2)

        await expect(() => contract.tx.burn_batch_from(BOB.address, [token1, token2], [amount1, amount2], []))
            .to.eventually.be.rejected
        await expect(() => contract.tx.burn_from(BOB.address, token1, amount1, []))
            .to.eventually.be.rejected
    })

    it('Burn from inssuficient balance should fail', async () => {
        const { query, contract, defaultSigner: sender } = await setup()

        let token1 = 1
        let token2 = 2
        let amount1 = 1
        let amount2 = 10

        const BOB = await getSigner('Bob')

        await contract.tx.mint_to(sender.address, token1, amount1)
        await contract.tx.mint_to(sender.address, token2, amount2)
        await contract.tx.mint_to(BOB.address, token1, amount1)
        await contract.tx.mint_to(BOB.address, token2, amount2)

        await expect(() => contract.tx.burn_batch([token1 + 1, token2], [amount1, amount2], []))
            .to.eventually.be.rejected
        await expect(() => contract.tx.burn(token1 + 1, amount1, []))
            .to.eventually.be.rejected

        await fromSigner(contract, BOB.address).tx.set_approval_for_all(sender.address, true)
        await expect(() => contract.tx.burn_batch_from(BOB.address, [token1, token2], [amount1 + 1, amount2], []))
            .to.eventually.be.rejected
        await expect(() => contract.tx.burn_from(BOB.address, token1, amount1 + 1, []))
            .to.eventually.be.rejected

    })
})
