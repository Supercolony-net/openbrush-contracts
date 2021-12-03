import { bnArg, expect, setupContract, fromSigner } from '../../helpers'

describe('MY_PSP721_ENUMERABLE', () => {
    async function setup() {
        return setupContract('my_psp721_enumerable', 'new')
    }

    async function compareToken(token, tokenId) {
        await expect(token.output!!.toJSON()!!['ok']).to.be.equal(tokenId)
    }

    it('Mint first token', async () => {
        const {
            contract,
            defaultSigner: sender,
            query
        } = await setup()

        let tokenId = bnArg(1)

        await expect(query.totalSupply()).to.have.output(0)
        await expect(query.balanceOf(sender.address)).to.have.output(0)

        // mint first token for alice
        await expect(contract.tx.mint(sender.address, tokenId)).to.eventually.be.fulfilled

        // check balances of contract
        await expect(query.totalSupply()).to.have.output(1)
        await expect(query.balanceOf(sender.address)).to.have.output(1)
        // check enumerable data
        compareToken(await query.tokenOfOwnerByIndex(sender.address, 0), tokenId)
        compareToken(await query.tokenByIndex(0), tokenId)
    })

    it('Mint more tokens', async () => {
        const {
            contract,
            defaultSigner: sender,
            accounts: [alice],
            query
        } = await setup()

        let tokenIds = [bnArg(1), bnArg(2), bnArg(3)]

        await expect(query.totalSupply()).to.have.output(0)
        await expect(query.balanceOf(sender.address)).to.have.output(0)

        for (let i = 0; i < tokenIds.length; ++i) {
            await expect(contract.tx.mint(sender.address, tokenIds[i])).to.eventually.be.fulfilled
        }

        // check balances of contract
        await expect(query.totalSupply()).to.have.output(3)
        await expect(query.balanceOf(sender.address)).to.have.output(3)
        // // check enumerable data
        for (let i = 0; i < tokenIds.length; ++i) {
            compareToken(await query.tokenOfOwnerByIndex(sender.address, i), tokenIds[i])
            compareToken(await query.tokenByIndex(i), tokenIds[i])
        }
    })

})
