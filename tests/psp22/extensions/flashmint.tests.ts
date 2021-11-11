import { expect, setupContract, fromSigner } from './../../helpers'

describe('MY_PSP22_FLASHMINT', () => {
    async function setup() {
        let flashmint = await setupContract('my_psp22_flashmint', 'new', '1000')
        let receiver = await setupContract('psp3156_flash_borrower', 'new')
        return { flashmint, receiver }
    }

    it('Deposit for works', async () => {
        const { flashmint, receiver } = await setup()
        
    })

})
