import { expect, expectRevert, setupContract } from './helpers'

describe('Revert on error', () => {
  async function setup() {
    return setupContract('revert_on_error_poc', 'new')
  }

  it('should flip', async () => {
    const { query, tx } = await setup()

    await expect(query.getValue()).to.have.output(false)
    await tx.flip()
    await expect(query.getValue()).to.have.output(true)

    // await expect(result?.output?.toString()).to.equal('{"err":"SomeError"}')
  })

  it('should flip with args', async () => {
    const { query, tx } = await setup()

    await expect(query.getValue()).to.have.output(false)
    await tx.flipWithArgs(3)
    await expect(query.getValue()).to.have.output(true)
  })

  it('should return error', async () => {
    const { query } = await setup()
    const result = await query.flipWithError()

    await expect(result?.output?.toString()).to.equal('{"err":"SomeError"}')
  })

  it('expect revert', async () => {
    const { query, tx } = await setup()

    await expect(query.getValue()).to.have.output(false)
    await expectRevert(tx.flipWithError(), 'SomeError')
    await expect(query.getValue()).to.have.output(false)
  })

  it('should fail', async () => {
    const { tx } = await setup()

    try {
      await tx.flipWithError()
      expect.fail()
    } catch (e) {
      expect(e.message, 'SomeError')
    }
  })
})
