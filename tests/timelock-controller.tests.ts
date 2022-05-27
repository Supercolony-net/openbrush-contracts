import { bnArg, expect, fromSigner, setupContract } from './helpers'
import { network, patract } from 'redspot'
import BN from 'bn.js'

const { getRandomSigner } = patract
const { api, getSigners } = network

function getMessageAbi(contract, ident) {
  return contract.contract.abi.messages.find((message) => message.identifier === ident)!
}

describe('MY_TIMELOCK_CONTROLLER', () => {
  async function setup() {
    const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]))
    const signers = await getSigners()
    const bob = await getRandomSigner(signers[1], one.muln(10000))
    const contract = await setupContract('my_timelock_controller', 'new', 0, [bob.address], [bob.address])

    return { contract, bob }
  }

  it('TIMELOCK CONTROLLER - can schedule', async () => {
    const { contract, bob } = await setup()

    // Arrange - Prepare data for schedule
    const transaction = {
      callee: contract.contract.address,
      selector: [0, 0, 0, 0],
      input: [],
      transferred_value: 0,
      gas_limit: 0
    }
    const salt = bnArg(0)

    // Act - Bob scheduled the transaction
    const id = (await contract.query.hashOperation(transaction, undefined, salt)).output!
    await expect(contract.query.isOperationPending(id)).to.have.output(false)
    await expect(fromSigner(contract.contract, bob.address).tx.schedule(transaction, undefined, salt, 0)).to.eventually.be.fulfilled

    // Assert - Operation must be scheduled, it should be in Pending state and in Ready state(because min delay is zero)
    await expect(contract.query.isOperationPending(id)).to.have.output(true)
    await expect(contract.query.isOperationReady(id)).to.have.output(true)
    await expect(contract.query.isOperationDone(id)).to.have.output(false)
  })

  it('TIMELOCK CONTROLLER - schedule and execute without input data `TimelockController::get_min_delay`', async () => {
    const { contract, bob } = await setup()

    // Arrange - Prepare data for execute `get_min_delay`
    const message = getMessageAbi(contract, 'TimelockController::get_min_delay')
    const transaction = {
      callee: contract.contract.address,
      selector: message.selector,
      input: [],
      transferred_value: 0,
      gas_limit: 0
    }
    const salt = bnArg(0)

    // Act - Bob scheduled the transaction
    const id = (await contract.query.hashOperation(transaction, undefined, salt)).output!
    await expect(fromSigner(contract.contract, bob.address).tx.schedule(transaction, undefined, salt, 0)).to.eventually.be.fulfilled

    // Assert - Transaction must be updated and now the state is Done
    await expect(contract.query.isOperationDone(id)).to.have.output(false)
    await expect(fromSigner(contract.contract, bob.address).tx.execute(transaction, undefined, salt)).to.eventually.be.fulfilled
    await expect(contract.query.isOperationDone(id)).to.have.output(true)
  })

  it('TIMELOCK CONTROLLER - schedule and execute by passing value into `TimelockController::update_delay`, and update', async () => {
    const { contract, bob } = await setup()

    // Arrange - Prepare data for execute `update_delay` with a new `min_delay`
    const message = getMessageAbi(contract, 'TimelockController::update_delay')
    const new_min_delay = 15
    const dataWithSelector = message.toU8a([new_min_delay])

    // --------
    // Remove selector id
    const data = new Uint8Array(dataWithSelector.length - 4)
    let dataLength = dataWithSelector[0]
    dataLength -= 4 * 4
    data.set([dataLength])
    data.set(dataWithSelector.slice(5), 1)
    // --------

    const transaction = {
      callee: contract.contract.address,
      selector: message.selector,
      input: data,
      transferred_value: 0,
      gas_limit: 0
    }
    const salt = bnArg(0)

    // Act - Bob scheduled the transaction
    await expect(fromSigner(contract.contract, bob.address).tx.schedule(transaction, undefined, salt, 0)).to.eventually.be.fulfilled

    // Assert - Min delay must be updated via `execute` method
    await expect(contract.query.getMinDelay()).to.have.output(0)
    await expect(fromSigner(contract.contract, bob.address).tx.execute(transaction, undefined, salt)).to.eventually.be.fulfilled
    await expect(contract.query.getMinDelay()).to.have.output(new_min_delay)
  })

  it('TIMELOCK CONTROLLER - fails schedule because signer is not proposal', async () => {
    const { contract } = await setup()

    // Arrange - Prepare data for schedule
    const transaction = {
      callee: contract.contract.address,
      selector: [0, 0, 0, 0],
      input: [],
      transferred_value: 0,
      gas_limit: 0
    }
    const salt = bnArg(0)

    // Assert - Alice can't schedule the transaction
    await expect(fromSigner(contract.contract, contract.alice.address).tx.schedule(transaction, undefined, salt, 0)).to.eventually.be.rejected
  })

  it('TIMELOCK CONTROLLER - fails execute because signer is not executor', async () => {
    const { contract, bob } = await setup()

    // Arrange - Prepare data for schedule
    const transaction = {
      callee: contract.contract.address,
      selector: [0, 0, 0, 0],
      input: [],
      transferred_value: 0,
      gas_limit: 0
    }
    const salt = bnArg(0)

    // Act - Bob scheduled the transaction
    await expect(fromSigner(contract.contract, bob.address).tx.schedule(transaction, undefined, salt, 0)).to.eventually.be.fulfilled

    // Assert - Alice can't execute the transaction
    await expect(fromSigner(contract.contract, contract.alice.address).tx.execute(transaction, undefined, salt)).to.eventually.be.rejected
  })

  it('TIMELOCK CONTROLLER - fails update_delay', async () => {
    const { contract, bob } = await setup()

    // Assert - Bob is not contract itself, then it must fails
    await expect(fromSigner(contract.contract, bob.address).tx.updateDelay(15)).to.eventually.be.rejected
  })
})
