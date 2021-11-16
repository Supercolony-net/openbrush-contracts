import { expect, setupContract } from './helpers'

describe('REENTRANCY_GUARD', () => {
  async function setup() {
    return setupContract('my_flipper_guard', 'new')
  }

  async function setup_flip_on_me() {
    return setupContract('flip_on_me', 'new')
  }

  it('One flip works correct', async () => {
    const { contract, query, defaultSigner: sender } = await setup()

    // Arrange - Ensure flip value is false
    await expect(query.getValue()).to.have.output(false)

    // Act - Flip
    await expect(contract.tx.flip()).to.eventually.be.fulfilled

    // Assert - Flip value must be true after flip
    await expect(query.getValue()).to.have.output(true)
  })

  it('Two flips work correct', async () => {
    const { contract, query, defaultSigner: sender } = await setup()

    // Arrange - Ensure flip value is false
    await expect(query.getValue()).to.have.output(false)

    // Act - Flip
    await expect(contract.tx.flip()).to.eventually.be.fulfilled
    await expect(contract.tx.flip()).to.eventually.be.fulfilled

    // Assert - After two flips value must be false again
    await expect(query.getValue()).to.have.output(false)
  })

  it('Flip on target works', async () => {
    const { query, contract } = await setup()

    const { tx } = await setup_flip_on_me()

    // Arrange - Ensure flip value is false
    await expect(query.getValue()).to.have.output(false)

    // Act
    await expect(tx.flipOnTarget(contract.address)).to.eventually.be.fulfilled

    // Assert - Value still must be true
    await expect(query.getValue()).to.have.output(true)
  })

  it('Call flip on me must fail', async () => {
    const { tx, query, defaultSigner: sender } = await setup()

    const { contract } = await setup_flip_on_me()

    // Arrange - Ensure flip value is false
    await expect(query.getValue()).to.have.output(false)

    // Assert
    await expect(tx.callFlipOnMe(contract.address)).to.eventually.be.rejected

    // Assert - Value still must be false, because flip failed
    await expect(query.getValue()).to.have.output(false)
  })
})
