import {expect, getSigners} from './helpers'
import {ApiPromise} from '@polkadot/api'
import ConstructorsFlipperGuard from '../../typechain-generated/constructors/my_flipper_guard'
import ContractFlipperGuard from '../../typechain-generated/contracts/my_flipper_guard'
import ConstructorsFlipOnMe from '../../typechain-generated/constructors/flip_on_me'
import ContractFlipOnMe from '../../typechain-generated/contracts/flip_on_me'

describe('REENTRANCY_GUARD', () => {
  async function setup() {
    const api = await ApiPromise.create()
    
    const signers = getSigners()
    
    const contractFactory = new ConstructorsFlipperGuard(api, signers[0])
    const contractAddress = (await contractFactory.new()).address
    
    const contract = new ContractFlipperGuard(contractAddress, signers[0], api)
    
    return {
      api,
      contract,
      query: contract.query,
      tx: contract.tx,
      defaultSigner: signers[0]
    }
  }

  async function setup_flip_on_me() {
    const api = await ApiPromise.create()
    
    const signers = getSigners()
    
    const contractFactory = new ConstructorsFlipOnMe(api, signers[0])
    const contractAddress = (await contractFactory.new()).address
    
    const contract = new ContractFlipOnMe(contractAddress, signers[0], api)
    
    return {
      api,
      contract,
      query: contract.query,
      tx: contract.tx,
      defaultSigner: signers[0]
    }
  }

  it('One flip works correct', async () => {
    const { api, contract, query, defaultSigner: sender } = await setup()

    // Arrange - Ensure flip value is false
    await expect(query.getValue()).to.have.output(false)

    // Act - Flip
    await expect(contract.tx.flip()).to.eventually.be.fulfilled

    // Assert - Flip value must be true after flip
    await expect(query.getValue()).to.have.output(true)

    await api.disconnect()
  })

  it('Two flips work correct', async () => {
    const { api, contract, query, defaultSigner: sender } = await setup()

    // Arrange - Ensure flip value is false
    await expect(query.getValue()).to.have.output(false)

    // Act - Flip
    await expect(contract.tx.flip()).to.eventually.be.fulfilled
    await expect(contract.tx.flip()).to.eventually.be.fulfilled

    // Assert - After two flips value must be false again
    await expect(query.getValue()).to.have.output(false)

    await api.disconnect()
  })

  it('Flip on target works', async () => {
    const { api: api1, query, contract } = await setup()

    const { api: api2, tx } = await setup_flip_on_me()

    // Arrange - Ensure flip value is false
    await expect(query.getValue()).to.have.output(false)

    // Act
    await expect(tx.flipOnTarget(contract.address)).to.eventually.be.fulfilled

    // Assert - Value still must be true
    await expect(query.getValue()).to.have.output(true)

    await api1.disconnect()
    await api2.disconnect()
  })

  it('Call flip on me must fail', async () => {
    const { api: api1, tx, query, defaultSigner: sender } = await setup()

    const { api: api2, contract } = await setup_flip_on_me()

    // Arrange - Ensure flip value is false
    await expect(query.getValue()).to.have.output(false)

    // Assert
    await expect(tx.callFlipOnMe(contract.address)).to.eventually.be.rejected

    // Assert - Value still must be false, because flip failed
    await expect(query.getValue()).to.have.output(false)

    await api1.disconnect()
    await api2.disconnect()
  })
})
