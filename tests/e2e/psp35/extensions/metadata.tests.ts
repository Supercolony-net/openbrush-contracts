import { bnArg, expect, setupContract } from '../../helpers'

describe('MY_PSP35_METADATA', () => {
  async function setup() {
    return setupContract('my_psp35_metadata', 'new')
  }

  it('Metadata works', async () => {
    const { query, tx } = await setup()
    const encoder = new TextEncoder()

    await expect(tx.setAttribute(bnArg(0), bnArg(1), bnArg(encoder.encode('https://www.supercolony.net/')))).to.eventually.be.fulfilled

    await expect(query.getAttribute(bnArg(0), bnArg(1))).to.have.output(bnArg(encoder.encode('https://www.supercolony.net/')))
  })

})
