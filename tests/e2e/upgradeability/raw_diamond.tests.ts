import {expect, getSigners} from '../helpers'
import {ApiPromise} from '@polkadot/api'

import ConstructorsPSP22 from '../../../typechain-generated/constructors/my_psp22_facet_v1'
import ContractPSP22 from '../../../typechain-generated/contracts/my_psp22_facet_v1'

import ConstructorsDiamond from '../../../typechain-generated/constructors/my_diamond'
import ContractDiamond from '../../../typechain-generated/contracts/my_diamond'

import ConstructorsPSP22Metadata from '../../../typechain-generated/constructors/my_psp22_metadata_facet'
import ContractPSP22Metadata from '../../../typechain-generated/contracts/my_psp22_metadata_facet'

import ConstructorsRustDiamond from '../../../typechain-generated/constructors/rust_diamond'
import ContractRustDiamond from '../../../typechain-generated/contracts/rust_diamond'

import ConstructorsInkDiamond from '../../../typechain-generated/constructors/ink_diamond'
import ContractInkDiamond from '../../../typechain-generated/contracts/ink_diamond'

import {FacetCut as FacetCutRust} from '../../../typechain-generated/types-arguments/ink_diamond'
import {FacetCut as FacetCutInk} from '../../../typechain-generated/types-arguments/ink_diamond'
import {AbiMessage} from '@polkadot/api-contract/types'

const getSelectorsFromMessages = (messages: AbiMessage[]): number[][] => {
  return messages.map((message) => {
    return message.selector.toU8a() as unknown as number[]
  })
}

const getSelectorsFromMessagesString = (messages: AbiMessage[]): string[] => {
  return messages.map((message) => {
    return message.selector.toString()
  })
}

const getSelectorByName = (messages: AbiMessage[], name: string) => {
  return messages.filter((message) => {
    return message.identifier == name
  })[0].selector.toU8a() as unknown as number[]
}

describe.skip('RAW_DIAMOND', () => {
  async function setupPSP22Facet() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP22(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
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
  
  async function setupDiamond(owner: string) {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsDiamond(api, defaultSigner)
    const contractAddress = (await contractFactory.new(owner)).address
    const contract = new ContractDiamond(contractAddress, defaultSigner, api)

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

  async function setupPSP22MetadataFacet() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP22Metadata(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
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

  async function setupRustDiamond(diamondCut: FacetCutRust) {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsRustDiamond(api, defaultSigner)
    const contractAddress = (await contractFactory.new(diamondCut)).address
    const contract = new ContractRustDiamond(contractAddress, defaultSigner, api)

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

  async function setupInkDiamond(diamondCut: FacetCutInk) {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsInkDiamond(api, defaultSigner)
    const contractAddress = (await contractFactory.new(diamondCut)).address
    const contract = new ContractInkDiamond(contractAddress, defaultSigner, api)

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

  function setupProxy<T>(contract: T, proxyAddress): T {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    // TODO: Add interface for Contract
    return contract.withAddress(proxyAddress)
  }

  
  it('Adding facets works for rust diamond', async () => {
    // abi of psp22 facet
    const { contract: psp22Facet, abi: psp22Abi, close: closePSP22 } = await setupPSP22Facet()
    // abi of diamond facet
    const { contract: diamondFacet, abi: diamondAbi, close: closeDiamond } = await setupDiamond(psp22Facet.address)

    const psp22Hash = psp22Abi.info.source.wasmHash.toString()

    const psp22Messages = psp22Abi.messages
    const psp22Init = getSelectorByName(psp22Messages, 'init_psp22')

    const diamondHash: string = diamondAbi.info.source.wasmHash.toString()

    const diamondMessages = diamondAbi.messages

    const psp22Selectors = getSelectorsFromMessages(psp22Messages)
    const psp22Cut = {hash: psp22Hash, selectors: psp22Selectors}

    const diamondSelectors = getSelectorsFromMessages(diamondMessages)
    const diamondCut = {
      hash: diamondHash,
      selectors: diamondSelectors
    }

    const { contract: diamondContractOriginal, defaultSigner, close: closeRustDiamond } = await setupRustDiamond(diamondCut)

    const diamondContract = await setupProxy(diamondFacet, diamondContractOriginal.address)

    // add diamond facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut([diamondCut], null)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut([psp22Cut], {hash: psp22Hash, selector: psp22Init, input: []})

    // patch methods
    const proxyPSP22Facet = setupProxy(psp22Facet, diamondContract.address)

    // we called init function which mints tokens and sets owner
    await expect(proxyPSP22Facet.query.balanceOf(defaultSigner.address)).to.bnToNumber(1000)

    // add metadata to contract
    const { contract: metadataFacet, abi: metadataAbi, close: closePSP22Metadata } = await setupPSP22MetadataFacet()

    const metadataHash = metadataAbi.info.source.wasmHash.toString()
    const metadataMessages = metadataAbi.messages

    const metadataInit = getSelectorByName(metadataMessages, 'init_metadata')
    const metadataSelectors = getSelectorsFromMessages(metadataMessages)

    const metadataCut = {hash: metadataHash, selectors: metadataSelectors}

    // add metadata facet
    await expect(diamondContract.withSigner(defaultSigner).tx.diamondCut([metadataCut], {hash: metadataHash, selector: metadataInit, input: []})).to.eventually.be
      .fulfilled

    // patch methods
    const proxyPSP22Metadata = setupProxy(metadataFacet, diamondContract.address)

    await expect(proxyPSP22Metadata.query.tokenName()).to.bytesToString('PSP22 Diamond')
    await expect(proxyPSP22Metadata.query.tokenSymbol()).to.bytesToString('PSP22D')
    await expect(proxyPSP22Metadata.query.tokenDecimals()).to.output(18)

    // Test Loupe

    expect((await diamondContract.query.facets()).value).to.be.deep.equal([
      { hash: psp22Hash, selectors: getSelectorsFromMessagesString(psp22Messages) },
      { hash: metadataHash, selectors: getSelectorsFromMessagesString(metadataMessages) }
    ])

    expect((await diamondContract.query.facetFunctionSelectors(metadataHash)).value).to.be.deep.equal(getSelectorsFromMessagesString(metadataMessages))
    expect((await diamondContract.query.facetFunctionSelectors(psp22Hash)).value).to.be.deep.equal(getSelectorsFromMessagesString(psp22Messages))

    expect((await diamondContract.query.facetCodeHashes()).value).to.be.deep.equal([psp22Hash, metadataHash])

    await expect(diamondContract.query.facetCodeHash(psp22Init)).to.output(psp22Hash)
    await expect(diamondContract.query.facetCodeHash(metadataInit)).to.output(metadataHash)

    // Close
    await closePSP22()
    await closePSP22Metadata()
    await closeDiamond()
    await closeRustDiamond()
  })

  it('Adding facets works for ink diamond', async () => {
    // abi of psp22 facet
    const { contract: psp22Facet, abi: psp22Abi, defaultSigner: defaultSignerPsp22, close: closePSP22 } = await setupPSP22Facet()
    // abi of diamond facet
    const { contract: diamondFacet, abi: diamondAbi, close: closeDiamond } = await setupDiamond(psp22Facet.address)

    const psp22Hash = psp22Abi.info.source.wasmHash.toString()
    const psp22Messages = psp22Abi.messages
    const psp22Init = getSelectorByName(psp22Messages, 'init_psp22')

    const diamondHash = diamondAbi.info.source.wasmHash.toString()
    const diamondMessages = diamondAbi.messages

    const psp22Selectors = getSelectorsFromMessages(psp22Messages)
    const psp22Cut = [{hash: psp22Hash, selectors: psp22Selectors}]

    const diamondSelectors = getSelectorsFromMessages(diamondMessages)
    const diamondCut = {hash: diamondHash, selectors: diamondSelectors}

    // initialize diamond contract
    const { contract: diamondContractOriginal, defaultSigner, close: closeInkDiamond } = await setupInkDiamond(diamondCut)
    const diamondContract = setupProxy(diamondFacet, diamondContractOriginal.address)

    // add diamond facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut([diamondCut], null)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut(psp22Cut, {hash: psp22Hash, selector: psp22Init, input: []})

    // patch methods
    const proxyPSP22 = setupProxy(psp22Facet, diamondContract.address)

    // we called init function which mints tokens and sets owner
    await expect(proxyPSP22.query.balanceOf(defaultSigner.address)).to.bnToNumber(1000)

    // add metadata to contract
    const { contract: metadataFacet, abi: metadataAbi, close: closePSP22Metadata } = await setupPSP22MetadataFacet()

    const metadataHash = metadataAbi.info.source.wasmHash.toString()
    const metadataMessages = metadataAbi.messages

    const metadataInit = getSelectorByName(metadataMessages, 'init_metadata')
    const metadataSelectors = getSelectorsFromMessages(metadataMessages)

    const metadataCut = [{hash: metadataHash, selectors: metadataSelectors}]

    // add metadata facet
    await expect(diamondContract.withSigner(defaultSigner).tx.diamondCut(metadataCut, {hash: metadataHash, selector: metadataInit, input: []})).to.eventually.be
      .fulfilled

    // patch methods
    const proxyMetadata = setupProxy(metadataFacet, diamondContract.address)

    await expect(proxyMetadata.query.tokenName()).to.bytesToString('PSP22 Diamond')
    await expect(proxyMetadata.query.tokenSymbol()).to.bytesToString('PSP22D')
    await expect(proxyMetadata.query.tokenDecimals()).to.output(18)

    // Test Loupe

    expect((await diamondContract.query.facets()).value).to.be.deep.equal([
      { hash: psp22Hash, selectors: getSelectorsFromMessagesString(psp22Messages) },
      { hash: metadataHash, selectors: getSelectorsFromMessagesString(metadataMessages) }
    ])

    expect((await diamondContract.query.facetFunctionSelectors(metadataHash)).value).to.be.deep.equal(getSelectorsFromMessagesString(metadataMessages))
    expect((await diamondContract.query.facetFunctionSelectors(psp22Hash)).value).to.be.deep.equal(getSelectorsFromMessagesString(psp22Messages))

    expect((await diamondContract.query.facetCodeHashes()).value).to.be.deep.equal([psp22Hash, metadataHash])

    await expect(diamondContract.query.facetCodeHash(psp22Init)).to.output(psp22Hash)
    await expect(diamondContract.query.facetCodeHash(metadataInit)).to.output(metadataHash)

    // Close
    await closePSP22()
    await closePSP22Metadata()
    await closeDiamond()
    await closeInkDiamond()
  })
})
