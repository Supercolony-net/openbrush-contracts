import { expect, setupContract, setupContractPalletAssets, aliceAddress } from '../../helpers'

describe('MY_PSP22_ASSET_METADATA', () => {
    function getRandomInt(max) {
        return Math.floor(Math.random() * max);
    }

    async function setup() {
        const random_asset_id = getRandomInt(10000).toString();
        return setupContractPalletAssets('my_psp22_pallet_asset', 'new', 
        'caller', 
        random_asset_id,
        aliceAddress,
        '1',
        'token',
        'TKN',
        '18'
        )
    }

    it('Metadata works', async () => {
        const { query, defaultSigner: sender } = await setup()

        // await expect(query.assetName()).to.have.output('token')
        await expect(query.assetSymbol()).to.have.output('TKN')
        await expect(query.assetDecimals()).to.have.output(18)
    })

})
