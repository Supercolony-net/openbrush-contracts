import BN from 'bn.js'
import Contract from '@redspot/patract/contract'
import { artifacts, network, patract } from 'redspot'
import { expect } from './setup/chai'
import { KeyringPair } from '@polkadot/keyring/types'
import { buildTx } from '@redspot/patract/buildTx'
import { Keyring } from '@polkadot/keyring'
import { TransactionParams, TransactionResponse } from '@redspot/patract/types'


const { getContractFactory, getRandomSigner } = patract
const { api, getSigners, getAddresses } = network


export { expect } from './setup/chai'

const patchContractMethods = (contract: Contract): Contract => {
  patchMethods(contract.query)
  patchMethods(contract.tx)

  for (const prop in contract.tx) {
    const original_tx = contract.tx[prop]
    contract.tx[prop] = async function (...args: TransactionParams): Promise<TransactionResponse> {
      return new Promise<TransactionResponse>(((resolve, reject) => {
        contract.query[prop](...args).then((_ => {
          // TODO: Check output of RPC call when Redspot will process it correct
          resolve(original_tx(...args))
        })).catch((reason => reject(reason)))
      }))
    }
  }

  return contract
}

// It removes prefix from the function and adds only name of method like a function
// PSP22::token_name
// query["PSP22,tokenName"]
// query.tokenName()
const patchMethods = (object) => {
  for (const prop in object) {
    if (prop.includes('::')) {
      const selectors = prop.split('::')
      const method = selectors[selectors.length - 1]
      object[method] = object[prop]
    }
  }
}

export const setupProxy = (contract, proxy): Contract =>  {
  const proxied_contract = new Contract(proxy.address, contract.abi, contract.api, proxy.signer)
  return patchContractMethods(proxied_contract)
}

function timeout(ms) {
  return new Promise(resolve => setTimeout(resolve, ms))
}

export const setupContract = async (name, constructor, ...args) => {
  await api.disconnect()
  await timeout(100)
  await api.connect()
  const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]))
  const signers = await getSigners()
  const defaultSigner = await getRandomSigner(signers[0], one.muln(10))
  const alice = await getRandomSigner(signers[1], one.muln(10))

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

export const addPairWithAmount = async (pair: KeyringPair): Promise<KeyringPair> => {
  const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]))
  const redspotPair = network.addPair(pair)
  await buildTx(api.registry, api.tx.balances.transfer(redspotPair.address, one.muln(10000)), (await getAddresses())[0])
  return redspotPair
}

export const getSigner = async (account: string) => {
  const signer = await addPairWithAmount(new Keyring().addFromUri(`//${account}`))
  return signer
}

export const fromSigner = (contract: Contract, address: string): Contract => {
  return patchContractMethods(contract.connect(address))
}

export const bnArg = (value: number | string | number[] | Uint8Array | Buffer | BN, len = 32) => {
  return new BN(value, undefined, 'le').toArray('le', len)
}

export const oneDay = () => (24 * 60 * 60 * 1000)
