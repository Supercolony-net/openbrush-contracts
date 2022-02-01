import { expect, setupContract } from '../../helpers'

describe('MY_PSP34_METADATA', () => {
  async function setup() {
    return setupContract('my_psp34_metadata', 'new', 1, 'Non Fungible Token', 'NFT')
  }

  it('Metadata works', async () => {
    const { query } = await setup()

    await expect(query.getAttribute(1, 'name')).to.have.output('Non Fungible Token')
    await expect(query.getAttribute(1, 'symbol')).to.have.output('NFT')
  })
})
