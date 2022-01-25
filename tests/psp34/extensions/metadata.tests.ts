import { expect, setupContract } from '../../helpers'

describe('MY_PSP34_METADATA', () => {
    async function setup() {
        return setupContract('my_psp34_metadata', 'new', 'Non Fungible Token', 'NFT')
    }

    it('Metadata works', async () => {
        const { query } = await setup()

        // TODO: add test for getting attributes using get_attribute
    })

})
