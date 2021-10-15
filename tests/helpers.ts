import Contract from '@redspot/patract/contract'
import BN from 'bn.js'
import {artifacts, network, patract} from 'redspot'
import {expect} from './setup/chai'
import {KeyringPair} from '@polkadot/keyring/types'
import {buildTx} from '@redspot/patract/buildTx'
import {Keyring} from '@polkadot/keyring'


const {getContractFactory, getRandomSigner} = patract
const {api, getSigners, getAddresses} = network

export { expect } from './setup/chai'

const patchContractMethods = (contract: Contract): Contract => {
  patchMethods(contract.query)
  patchMethods(contract.tx)
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

export const addPairWithAmount = async (pair: KeyringPair): Promise<KeyringPair> => {
  const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]))
  const redspotPair = network.addPair(pair)
  await buildTx(api.registry, api.tx.balances.transfer(redspotPair.address, one.muln(10000)), (await getAddresses())[0])
  return redspotPair
}

export const getSigner = async (account : string) => {
  const signer = await addPairWithAmount(new Keyring().addFromUri(`//${account}`))
  return signer
}

export const fromSigner = (contract: Contract, address: string): Contract => {
  return patchContractMethods(contract.connect(address))
}

export const bnArg = (value: number | string | number[] | Uint8Array | Buffer | BN, length = 32) =>
  new BN(value, undefined, 'le').toArray('le', length)
