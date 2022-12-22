import { consts } from '../constants'
import { expect, getSigners } from '../helpers'

import ContractProxy from '../../../typechain-generated/contracts/my_proxy'
import ConstructorProxy from '../../../typechain-generated/constructors/my_proxy'

import ContractPSP22 from '../../../typechain-generated/contracts/my_psp22_upgradeable'
import ConstructorPSP22 from '../../../typechain-generated/constructors/my_psp22_upgradeable'

import ContractPSP22Metadata from '../../../typechain-generated/contracts/my_psp22_metadata_upgradeable'
import ConstructorPSP22Metadata from '../../../typechain-generated/constructors/my_psp22_metadata_upgradeable'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP22Receiver from '../../../typechain-generated/constructors/psp22_receiver'
import ContractPSP22Receiver from '../../../typechain-generated/contracts/psp22_receiver'

describe.skip('MY_UPGRADEABLE_PSP22', () => {
  async function setupPSP22() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorPSP22(api, defaultSigner)
    const contractAddress = (await contractFactory.new(1000)).address
    const contract = new ContractPSP22(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx,
      abi: contract.abi,
      close: async () => {
        await api.disconnect()
      }
    }
  }

  async function setupProxyContract(hash: string) {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorProxy(api, defaultSigner)
    const contractAddress = (await contractFactory.new(hash)).address
    const contract = new ContractProxy(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx,
      abi: contract.abi,
      close: async () => {
        await api.disconnect()
      }
    }
  }

  async function setupPSP22Metadata() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorPSP22Metadata(api, defaultSigner)
    const contractAddress = (await contractFactory.new(0, [], [], 0)).address
    const contract = new ContractPSP22Metadata(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx,
      abi: contract.abi,
      close: async () => {
        await api.disconnect()
      }
    }
  }

  async function setup_receiver() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP22Receiver(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new ContractPSP22Receiver(contractAddress, defaultSigner, api)

    return {
      api,
      defaultSigner,
      alice,
      bob,
      contract,
      query: contract.query,
      tx: contract.tx,
      close: async () => {
        await api.disconnect()
      }
    }
  }

  function setupProxy<T>(contract: T, proxyAddress: string): T {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    // TODO: Add interface for Contract
    return contract.withAddress(proxyAddress)
  }

  it('MY_UPGRADEABLE_PSP22 - delegate code is my_psp22 code hash', async () => {
    const { abi, close: closePSP22 }  = await setupPSP22()
    const hash = abi.info.source.wasmHash.toString()
    const { query, close: closeProxy } = await setupProxyContract(hash)

    // Assert - contract hash is my_psp22 contract hash
    await expect(query.getDelegateCode()).to.have.output(hash)

    // Close
    await closeProxy()
    await closePSP22()
  })

  it('MY_UPGRADEABLE_PSP22 - only owner can change delegate code', async () => {
    const { abi, bob, close: closePSP22 } = await setupPSP22()
    const { contract: proxy, close: closeProxy } = await setupProxyContract('')
    await expect(proxy.withSigner(bob).tx.changeDelegateCode(abi.info.source.wasmHash.toString())).to.eventually.be.rejected

    // Close
    await closeProxy()
    await closePSP22()
  })

  it('MY_UPGRADEABLE_PSP22 - Assigns initial balance', async () => {
    const { contract: psp22, abi, close: closePSP22 } = await setupPSP22()
    const { contract, defaultSigner: sender, close: closeProxy } = await setupProxyContract(abi.info.source.wasmHash.toString())
    const proxy = setupProxy(psp22, contract.address)
    await proxy.tx.initialize(1000)

    await expect(proxy.query.totalSupply()).to.have.bnToNumber(1000)
    await expect(proxy.query.balanceOf(sender.address)).to.have.bnToNumber(1000)

    // Close
    await closeProxy()
    await closePSP22()
  })

  it('MY_UPGRADEABLE_PSP22 - wrong proxy setup leads to transaction fail', async () => {
    const { contract: psp22, close: closePSP22 } = await setupPSP22()
    const { contract, close: closeProxy } = await setupProxyContract('')
    const proxy = setupProxy(psp22, contract.address)

    await expect(proxy.tx.initialize(1000)).to.eventually.be.rejected

    // Close
    await closeProxy()
    await closePSP22()
  })

  it('MY_UPGRADEABLE_PSP22 - Transfer adds amount to destination account', async () => {
    const { contract: psp22, abi, bob: receiver, defaultSigner, close: closePSP22 } = await setupPSP22()
    const { contract, close: closeProxy } = await setupProxyContract(abi.info.source.wasmHash.toString())
    const proxy = setupProxy(psp22, contract.address)

    await proxy.tx.initialize(1000)

    await proxy.tx.transfer(receiver.address, 7, [])
    await expect(proxy.query.balanceOf(defaultSigner.address)).to.have.bnToNumber(1000 - 7)
    await expect(proxy.query.balanceOf(receiver.address)).to.have.bnToNumber(7)

    // Close
    await closeProxy()
    await closePSP22()
  })

  it('MY_UPGRADEABLE_PSP22 - Transfers funds successfully if destination account is a receiver and supports transfers', async () => {
    const { contract: psp22, abi, close: closePSP22} = await setupPSP22()
    const { contract, close: closeProxy } = await setupProxyContract(abi.info.source.wasmHash.toString())
    const proxy = setupProxy(psp22, contract.address)
    const { contract: psp22_receiver, close: closeReceiver } = await setup_receiver()
    await proxy.tx.initialize(1000)

    await proxy.tx.transfer(psp22_receiver.address, 7, [])

    // Close
    await closeProxy()
    await closePSP22()
    await closeReceiver()
  })

  it('MY_UPGRADEABLE_PSP22 - Transfers funds successfully if destination account is a receiver a contract but not PSP22Receiver', async () => {
    const { contract: psp22_0, abi, close: closePSP22V1 } = await setupPSP22()
    const { contract: psp22_1, close: closePSP22V2 } = await setupPSP22()
    const { contract, close: closeProxy } = await setupProxyContract(abi.info.source.wasmHash.toString())
    const proxy = setupProxy(psp22_0, contract.address)
    await proxy.tx.initialize(1000)

    await proxy.tx.transfer(psp22_1.address, 7, [])

    // Close
    await closeProxy()
    await closePSP22V1()
    await closePSP22V2()
  })

  it('MY_UPGRADEABLE_PSP22 - Can not transfer above the amount', async () => {
    const { contract: psp22, abi, bob: receiver, close: closePSP22 } = await setupPSP22()
    const { contract, close: closeProxy } = await setupProxyContract(abi.info.source.wasmHash.toString())
    const proxy = setupProxy(psp22, contract.address)
    await proxy.tx.initialize(1000)

    await expect(proxy.tx.transfer(receiver.address, 1007, [])).to.eventually.be.rejected

    // Close
    await closeProxy()
    await closePSP22()
  })

  it('MY_UPGRADEABLE_PSP22 - update psp22 to psp22_metadata', async () => {
    const { contract: psp22, abi: abi_psp22, bob: receiver, close: closePSP22 } = await setupPSP22()
    const { contract, close: closeProxy } = await setupProxyContract(abi_psp22.info.source.wasmHash.toString())
    const proxy = setupProxy(psp22, contract.address)
    await proxy.tx.initialize(1000)

    await expect(proxy.query.totalSupply()).to.have.bnToNumber(1000)
    await proxy.tx.transfer(receiver.address, 100, [])

    const { contract: psp22_metadata, abi: abi_psp22_metadata, close: closeMetadata } = await setupPSP22Metadata()
    const hash = abi_psp22_metadata.info.source.wasmHash.toString()

    await contract.tx.changeDelegateCode(hash)
    await expect(contract.query.getDelegateCode()).to.have.output(hash)

    const proxy_metadata = setupProxy(psp22_metadata, contract.address)
    await proxy_metadata.tx.initialize(0,'COLONY' as unknown as string[], 'COL' as unknown as string[], 18)

    await expect(proxy_metadata.query.totalSupply()).to.have.bnToNumber(1000)
    await expect(proxy_metadata.query.tokenName()).to.have.bytesToString('COLONY')
    await expect(proxy_metadata.query.tokenSymbol()).to.have.bytesToString('COL')
    await expect(proxy_metadata.query.tokenDecimals()).to.have.output(18)
    await expect(proxy_metadata.query.balanceOf(receiver.address)).to.have.bnToNumber(100)

    // Close
    await closeProxy()
    await closeMetadata()
    await closePSP22()
  })
})
