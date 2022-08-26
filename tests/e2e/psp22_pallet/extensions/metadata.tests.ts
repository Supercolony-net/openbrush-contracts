import { expect, setupContract } from '../../helpers'

describe('MY_PSP22_METADATA', () => {
    async function setup() {
        return setupContract('my_psp22_pallet', 'new', 1, 1, 1000, 'TOKEN', 'TKN', 18, {value: 10000})
    }

    it('Metadata works', async () => {
        const { query, defaultSigner: sender } = await setup()

        await expect(query.tokenName()).to.have.output('TOKEN')
        await expect(query.tokenSymbol()).to.have.output('TKN')
        await expect(query.tokenDecimals()).to.have.output(18)
    })

})
