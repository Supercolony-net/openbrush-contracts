import { bnArg, expect, setupContract } from '../../helpers'

describe('MY_PSP35_METADATA', () => {
  async function setup() {
    const encoder = new TextEncoder()
    return setupContract('my_psp35_metadata', 'new', bnArg(0), bnArg(1), bnArg(encoder.encode('https://www.supercolony.net/')))
  }

  it('Metadata works', async () => {
    const { query } = await setup()
    const encoder = new TextEncoder()

    await expect(query.getAttribute(bnArg(0), bnArg(1))).to.have.output(bnArg(encoder.encode('https://www.supercolony.net/')))
  })

})
