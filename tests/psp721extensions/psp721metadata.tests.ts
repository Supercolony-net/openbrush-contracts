import { expect, setupContract } from '../helpers'

describe('MY_PSP721_METADATA', () => {
    async function setup() {
        return setupContract('my_psp721_metadata', 'new', 'Non Fungible Token', 'NFT')
    }

    it('Metadata works', async () => {
        const { query } = await setup()

        await expect(query.name()).to.have.output('Non Fungible Token')
        await expect(query.symbol()).to.have.output('NFT')
    })

})
