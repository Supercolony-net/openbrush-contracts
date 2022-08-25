import {expect, getSigners} from '../helpers'
import {ApiPromise} from '@polkadot/api'

import ConstructorsPSP22 from '../../../typechain-generated/constructors/my_psp22_facet_v1'
import ContractPSP22 from '../../../typechain-generated/contracts/my_psp22_facet_v1'

import ConstructorsDiamond from '../../../typechain-generated/constructors/my_diamond'
import ContractDiamond from '../../../typechain-generated/contracts/my_diamond'

import ConstructorsPSP22Metadata from '../../../typechain-generated/constructors/my_psp22_metadata_facet'
import ContractPSP22Metadata from '../../../typechain-generated/contracts/my_psp22_metadata_facet'

import ContractProxy from '../../../typechain-generated/contracts/my_proxy'
import ConstructorProxy from '../../../typechain-generated/constructors/my_proxy'

const getSelectorsFromMessages = (messages) => {
  return messages.map((message) => {
    return message.selector
  })
}

const getSelectorByName = (messages, name) => {
  return messages.filter((message) => {
    return message.label == name
  })[0].selector
}

// async function setupProxy(contractConstructor, proxy): ContractProxy =>  {
//   const proxied_contract = new ContractProxy(proxy.address, contract.abi, contract.api, proxy.signer)
//   return patchContractMethods(proxied_contract)
// }

describe('RAW_DIAMOND', () => {
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
      tx: contract.tx
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
      tx: contract.tx
    }
  }

  async function setupRustDiamond(owner: string) {
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
      tx: contract.tx
    }
  }

  async function setuptPSP22MetadataFacet() {
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
      tx: contract.tx
    }
  }


  
  it('Adding facets works for rust diamond', async () => {
    // abi of psp22 facet
    const { contract: psp22Facet, abi: psp22Abi, defaultSigner: defaultSignerPsp22 } = await setupPSP22Facet()
    // abi of diamond facet
    const { contract: diamondFacet, abi: diamondAbi } = await setupDiamond(psp22Facet.address)

    const psp22Hash = (await psp22Abi).source.hash
    const psp22Messages = (await psp22Abi).V3.spec.messages
    const psp22Init = getSelectorByName(psp22Messages, 'init_psp22')

    const diamondHash = (await diamondAbi).source.hash
    const diamondMessages = (await diamondAbi).V3.spec.messages

    const psp22Selectors = getSelectorsFromMessages(psp22Messages)
    const psp22Cut = [[psp22Hash, psp22Selectors]]

    const diamondSelectors = getSelectorsFromMessages(diamondMessages)
    const diamondCut = [diamondHash, diamondSelectors]

    // initialize diamond contract
    let { contract: diamondContract, defaultSigner } = await setupContract('rust_diamond', 'new', diamondCut)
    diamondContract = setupProxy(diamondFacet, diamondContract)

    // add diamond facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut([diamondCut], null)).to.eventually.be.fulfilled

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(psp22Cut, [psp22Hash, psp22Init, []])).to.eventually.be.fulfilled

    // patch methods
    let proxy = setupProxy(psp22Facet, diamondContract)

    // we called init function which mints tokens and sets owner
    await expect(proxy.query.balanceOf(defaultSigner.address)).to.output(1000)

    // add metadata to contract
    const { contract: metadataFacet, abi: metadataAbi } = await setuptPSP22MetadataFacet()

    const metadataHash = (await metadataAbi).source.hash
    const metadataMessages = (await metadataAbi).V3.spec.messages

    const metadataInit = getSelectorByName(metadataMessages, 'init_metadata')
    const metadataSelectors = getSelectorsFromMessages(metadataMessages)

    const metadataCut = [[metadataHash, metadataSelectors]]

    // add metadata facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(metadataCut, [metadataHash, metadataInit, []])).to.eventually.be
      .fulfilled

    // patch methods
    proxy = setupProxy(metadataFacet, diamondContract)

    await expect(proxy.query.tokenName()).to.output('PSP22 Diamond')
    await expect(proxy.query.tokenSymbol()).to.output('PSP22D')
    await expect(proxy.query.tokenDecimals()).to.output(18)

    // Test Loupe

    await expect(diamondContract.query.facets()).to.output([
      { hash: psp22Hash, selectors: psp22Selectors },
      { hash: metadataHash, selectors: metadataSelectors }
    ])

    await expect(diamondContract.query.facetFunctionSelectors(metadataHash)).to.output(metadataSelectors)
    await expect(diamondContract.query.facetFunctionSelectors(psp22Hash)).to.output(psp22Selectors)

    await expect(diamondContract.query.facetCodeHashes()).to.output([psp22Hash, metadataHash])

    await expect(diamondContract.query.facetCodeHash(psp22Init)).to.output(psp22Hash)
    await expect(diamondContract.query.facetCodeHash(metadataInit)).to.output(metadataHash)
  })

  it('Adding facets works for ink diamond', async () => {
    // abi of psp22 facet
    const { contract: psp22Facet, abi: psp22Abi, defaultSigner: defaultSignerPsp22 } = await setupPSP22Facet()
    // abi of diamond facet
    const { contract: diamondFacet, abi: diamondAbi } = await setupDiamond(psp22Facet.address)

    const psp22Hash = (await psp22Abi).source.hash
    const psp22Messages = (await psp22Abi).V3.spec.messages
    const psp22Init = getSelectorByName(psp22Messages, 'init_psp22')

    const diamondHash = (await diamondAbi).source.hash
    const diamondMessages = (await diamondAbi).V3.spec.messages

    const psp22Selectors = getSelectorsFromMessages(psp22Messages)
    const psp22Cut = [[psp22Hash, psp22Selectors]]

    const diamondSelectors = getSelectorsFromMessages(diamondMessages)
    const diamondCut = [diamondHash, diamondSelectors]

    // initialize diamond contract
    let { contract: diamondContract, defaultSigner } = await setupContract('ink_diamond', 'new', diamondCut)
    diamondContract = setupProxy(diamondFacet, diamondContract)

    // add diamond facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut([diamondCut], null)).to.eventually.be.fulfilled

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(psp22Cut, [psp22Hash, psp22Init, []])).to.eventually.be.fulfilled

    // patch methods
    let proxy = setupProxy(psp22Facet, diamondContract)

    // we called init function which mints tokens and sets owner
    await expect(proxy.query.balanceOf(defaultSigner.address)).to.output(1000)

    // add metadata to contract
    const { contract: metadataFacet, abi: metadataAbi } = await setuptPSP22MetadataFacet()

    const metadataHash = (await metadataAbi).source.hash
    const metadataMessages = (await metadataAbi).V3.spec.messages

    const metadataInit = getSelectorByName(metadataMessages, 'init_metadata')
    const metadataSelectors = getSelectorsFromMessages(metadataMessages)

    const metadataCut = [[metadataHash, metadataSelectors]]

    // add metadata facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(metadataCut, [metadataHash, metadataInit, []])).to.eventually.be
      .fulfilled

    // patch methods
    proxy = setupProxy(metadataFacet, diamondContract)

    await expect(proxy.query.tokenName()).to.output('PSP22 Diamond')
    await expect(proxy.query.tokenSymbol()).to.output('PSP22D')
    await expect(proxy.query.tokenDecimals()).to.output(18)

    // Test Loupe

    await expect(diamondContract.query.facets()).to.output([
      { hash: psp22Hash, selectors: psp22Selectors },
      { hash: metadataHash, selectors: metadataSelectors }
    ])

    await expect(diamondContract.query.facetFunctionSelectors(metadataHash)).to.output(metadataSelectors)
    await expect(diamondContract.query.facetFunctionSelectors(psp22Hash)).to.output(psp22Selectors)

    await expect(diamondContract.query.facetCodeHashes()).to.output([psp22Hash, metadataHash])

    await expect(diamondContract.query.facetCodeHash(psp22Init)).to.output(psp22Hash)
    await expect(diamondContract.query.facetCodeHash(metadataInit)).to.output(metadataHash)
  })
})
