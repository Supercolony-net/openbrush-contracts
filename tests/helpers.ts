import Contract from '@redspot/patract/contract'
import BN from 'bn.js'
import { artifacts, network, patract } from 'redspot'
import { expect } from './setup/chai'
import { TransactionParams } from '@redspot/patract/types'

const { getContractFactory, getRandomSigner } = patract
const { api, getSigners } = network

export { expect } from './setup/chai'

const patchContractMethods = (contract: Contract): Contract => {
  patchMethods(contract.query)
  patchMethods(contract.tx)

  // @ts-ignore
  contract['tx'] = new Proxy(contract.tx, {
    get(target, prop: string) {
      return async function (...args: TransactionParams) {
        const result = await contract.query[prop](...args)
        const output = result.output?.toJSON()

        if (output && output['ok'] !== undefined) {
          return await target[prop](...args)
        } else {
          throw Error(output ? output['err'] : 'Unknown Error')
        }
      }
    }
  })

  return contract
}

// It removes prefix from the function and adds only name of method like a function
// PSP22::token_name
// query["PSP22,tokenName"]
// query.tokenName()
const patchMethods = (object) => {
  for (const prop in object) {
    if (prop.includes(',')) {
      const selectors = prop.split(',')
      const method = selectors[selectors.length - 1]
      object[method] = object[prop]
    }
  }
}

export const setupContract = async (name, constructor, ...args) => {
  const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]))
  const signers = await getSigners()
  const defaultSigner = await getRandomSigner(signers[0], one.muln(10000))
  const alice = await getRandomSigner(signers[1], one.muln(10000))

  const contractFactory = await getContractFactory(name, defaultSigner)
  const contract = await contractFactory.deploy(constructor, ...args)
  const abi = artifacts.readArtifact(name)
  patchContractMethods(contract)

  return {
    defaultSigner,
    alice,
    accounts: [alice, await getRandomSigner(), await getRandomSigner()],
    contractFactory,
    contract,
    abi,
    one,
    query: contract.query,
    tx: contract.tx
  }
}

export const fromSigner = (contract: Contract, address: string): Contract => {
  return patchContractMethods(contract.connect(address))
}

export const bnArg = (value: number | string | number[] | Uint8Array | Buffer | BN, length = 32) =>
  new BN(value, undefined, 'le').toArray('le', length)

export const expectRevert = async <T>(promise: Promise<T>, errorMessage = '') => {
  try {
    await promise
    expect.fail('Should be reverted.')
  } catch (e) {
    if (errorMessage) {
      expect(e.message, errorMessage)
    }
  }
}
