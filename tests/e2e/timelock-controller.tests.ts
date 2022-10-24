import {bnArg, expect, getSigners} from './helpers'
import Constructors from '../../typechain-generated/constructors/my_timelock_controller'
import Contract from '../../typechain-generated/contracts/my_timelock_controller'
import {ApiPromise} from '@polkadot/api'

function getMessageAbi(contract: Contract, identifier: string) {
  return contract.abi.findMessage(identifier)!
}

describe('MY_TIMELOCK_CONTROLLER', () => {
  async function setup() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const bob = signers[1]
    const defaultSigner = signers[0]

    const contractFactory = new Constructors(api, defaultSigner)
    const { address: contractAddress } = await contractFactory.new(0, [bob.address], [bob.address])

    const contract = new Contract(contractAddress, defaultSigner, api)

    // const contract = await setupContract('my_timelock_controller', 'new', 0, [bob.address], [bob.address])

    return { api, contract, bob, alice: defaultSigner }
  }

  it('TIMELOCK CONTROLLER - can schedule', async () => {
    const { api, contract, bob } = await setup()

    // Arrange - Prepare data for schedule
    const transaction = {
      callee: contract.address,
      selector: [0, 0, 0, 0],
      input: [],
      transferred_value: 0,
      gas_limit: 0
    }
    const salt = bnArg(0)

    // Act - Bob scheduled the transaction
    const id = (await contract.query.hashOperation(transaction, null, salt)).value!
    await expect(contract.query.isOperationPending(id)).to.have.output(false)
    await expect(contract.withSigner(bob).tx.schedule(transaction, null, salt, 0)).to.eventually.be.fulfilled

    // Assert - Operation must be scheduled, it should be in Pending state and in Ready state(because min delay is zero)
    await expect(contract.query.isOperationPending(id)).to.have.output(true)
    await expect(contract.query.isOperationReady(id)).to.have.output(true)
    await expect(contract.query.isOperationDone(id)).to.have.output(false)

    await api.disconnect()
  })

  it('TIMELOCK CONTROLLER - schedule and execute without input data `TimelockController::get_min_delay`', async () => {
    const { api, contract, bob } = await setup()

    // Arrange - Prepare data for execute `get_min_delay`
    const message = getMessageAbi(contract, 'TimelockController::get_min_delay')
    const transaction = {
      callee: contract.address,
      selector: message.selector.toU8a() as unknown as number[],
      input: [],
      transferred_value: 0,
      gas_limit: 0
    }
    const salt = bnArg(0)

    // Act - Bob scheduled the transaction
    const id = (await contract.query.hashOperation(transaction, null, salt)).value!
    await expect(contract.withSigner(bob).tx.schedule(transaction, null, salt, 0)).to.eventually.be.fulfilled

    // Assert - Transaction must be updated and now the state is Done
    await expect(contract.query.isOperationDone(id)).to.have.output(false)
    await expect(contract.withSigner(bob).tx.execute(transaction, null, salt)).to.eventually.be.fulfilled
    await expect(contract.query.isOperationDone(id)).to.have.output(true)

    await api.disconnect()
  })

  it('TIMELOCK CONTROLLER - schedule and execute by passing value into `TimelockController::update_delay`, and update', async () => {
    const { api, contract, bob } = await setup()

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
      callee: contract.address,
      selector: message.selector.toU8a() as unknown as number[],
      input: data as unknown as number[],
      transferred_value: 0,
      gas_limit: 0
    }
    const salt = bnArg(0)

    // Act - Bob scheduled the transaction
    await expect(contract.withSigner(bob).tx.schedule(transaction, null, salt, 0)).to.eventually.be.fulfilled

    // Assert - Min delay must be updated via `execute` method
    await expect(contract.query.getMinDelay()).to.have.output(0)
    await expect(contract.withSigner(bob).tx.execute(transaction, null, salt)).to.eventually.be.fulfilled
    await expect(contract.query.getMinDelay()).to.have.output(new_min_delay)

    await api.disconnect()
  })

  it('TIMELOCK CONTROLLER - fails schedule because signer is not proposal', async () => {
    const { api, contract, alice } = await setup()

    // Arrange - Prepare data for schedule
    const transaction = {
      callee: contract.address,
      selector: [0, 0, 0, 0],
      input: [],
      transferred_value: 0,
      gas_limit: 0
    }
    const salt = bnArg(0)

    // Assert - Alice can't schedule the transaction
    await expect(contract.withSigner(alice).tx.schedule(transaction, null, salt, 0)).to.eventually.be.rejected

    await api.disconnect()
  })

  it('TIMELOCK CONTROLLER - fails execute because signer is not executor', async () => {
    const { api, contract, bob, alice } = await setup()

    // Arrange - Prepare data for schedule
    const transaction = {
      callee: contract.address,
      selector: [0, 0, 0, 0],
      input: [],
      transferred_value: 0,
      gas_limit: 0
    }
    const salt = bnArg(0)

    // Act - Bob scheduled the transaction
    await expect(contract.withSigner(bob).tx.schedule(transaction, null, salt, 0)).to.eventually.be.fulfilled

    // Assert - Alice can't execute the transaction
    await expect(contract.withSigner(alice).tx.execute(transaction, null, salt)).to.eventually.be.rejected

    await api.disconnect()
  })

  it('TIMELOCK CONTROLLER - fails update_delay', async () => {
    const { api, contract, bob } = await setup()

    // Assert - Bob is not contract itself, then it must fails
    await expect(contract.withSigner(bob).tx.updateDelay(15)).to.eventually.be.rejected

    await api.disconnect()
  })
})
