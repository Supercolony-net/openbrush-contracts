import {expect, getSigners} from '../helpers'
import {AbiMessage} from '@polkadot/api-contract/types'
import {ApiPromise} from '@polkadot/api'
import ConstructorsPSP22 from '../../../typechain-generated/constructors/my_psp22_facet_v1'
import ContractPSP22 from '../../../typechain-generated/contracts/my_psp22_facet_v1'
import ConstructorsPSP22V2 from '../../../typechain-generated/constructors/my_psp22_facet_v2'
import ContractPSP22V2 from '../../../typechain-generated/contracts/my_psp22_facet_v2'
import ConstructorsDiamond from '../../../typechain-generated/constructors/my_diamond'
import ContractDiamond from '../../../typechain-generated/contracts/my_diamond'
import ConstructorsDiamondCaller from '../../../typechain-generated/constructors/diamond_caller'
import ContractDiamondCaller from '../../../typechain-generated/contracts/diamond_caller'
import ConstructorsPSP22Metadata from '../../../typechain-generated/constructors/my_psp22_metadata_facet'
import ContractPSP22Metadata from '../../../typechain-generated/contracts/my_psp22_metadata_facet'

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

const getSelectorByName = (messages: AbiMessage[], name: string): number[] => {
  return messages.filter((message) => {
    return message.identifier == name
  })[0].selector.toU8a() as unknown as number[]
}

const getSelectorByNameString = (messages: AbiMessage[], name: string): string => {
  return messages.filter((message) => {
    return message.identifier == name
  })[0].selector.toString()
}

describe.skip('DIAMOND_PSP22', () => {
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

  async function setupPSP22FacetV2() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsPSP22V2(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new ContractPSP22V2(contractAddress, defaultSigner, api)

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
  
  async function setupDiamondCaller() {
    const api = await ApiPromise.create()

    const signers = getSigners()
    const defaultSigner = signers[2]
    const alice = signers[0]
    const bob = signers[1]

    const contractFactory = new ConstructorsDiamondCaller(api, defaultSigner)
    const contractAddress = (await contractFactory.new()).address
    const contract = new ContractDiamondCaller(contractAddress, defaultSigner, api)

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

  function setupProxy<T>(contract: T, proxyAddress: string): T {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    // TODO: Add interface for Contract
    return contract.withAddress(proxyAddress)
  }

  it('Adding facets works', async () => {
    // abi of psp22 facet
    const { contract: psp22Facet, abi, defaultSigner, close: closePSP22 } = await setupPSP22Facet()

    const psp22Hash = abi.info.source.wasmHash.toString()
    const psp22Messages = abi.messages

    const psp22Init = getSelectorByName(psp22Messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(psp22Messages)
    const psp22Cut = [{hash: psp22Hash, selectors: psp22Selectors}]

    // initialize diamond contract
    const { contract: diamondContract, close: closeDiamond } = await setupDiamond(defaultSigner.address)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut(psp22Cut, {hash: psp22Hash, selector: psp22Init, input: []})

    // patch methods
    const proxyPSP22 = setupProxy(psp22Facet, diamondContract.address)

    // we called init function which mints tokens and sets owner
    await expect(proxyPSP22.query.balanceOf(defaultSigner.address)).to.bnToNumber(1000)

    // add metadata to contract
    const { contract: metadataFacet, abi: metadataAbi, close: closeMetadata } = await setupPSP22MetadataFacet()

    const metadataHash = metadataAbi.info.source.wasmHash.toString()
    const metadataMessages = metadataAbi.messages

    const metadataInit = getSelectorByName(metadataMessages, 'init_metadata')
    const metadataSelectors = getSelectorsFromMessages(metadataMessages)

    const metadataCut = [{hash: metadataHash, selectors: metadataSelectors}]

    // add metadata facet
    await expect(diamondContract.withSigner(defaultSigner).tx.diamondCut(metadataCut, {hash: metadataHash, selector: metadataInit, input: []})).to.eventually.be
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

    await closePSP22()
    await closeMetadata()
    await closeDiamond()
  })

  it('Only owner can call diamond cut', async () => {
    // abi of psp22 facet
    const { abi, alice, close: closePSP22 } = await setupPSP22Facet()

    const psp22Hash = abi.info.source.wasmHash.toString()
    const messages = abi.messages

    const initSelector = getSelectorByName(messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(messages)
    const facetCut = [{hash: psp22Hash, selectors: psp22Selectors}]

    // initialize diamond contract
    const { contract: diamondContract, bob: wrongSigner, close: closeDiamond } = await setupDiamond(alice.address)

    // add psp22 facet
    await expect(diamondContract.withSigner(wrongSigner).tx.diamondCut(facetCut, {hash: psp22Hash, selector: initSelector, input: []})).to.eventually.be.rejected

    await closePSP22()
    await closeDiamond()
  })

  it('Replacing facets works', async () => {
    // abi of psp22 facet
    const { abi, defaultSigner, close: closePSP22 } = await setupPSP22Facet()

    const psp22Hash = abi.info.source.wasmHash.toString()
    const messages = abi.messages

    const initSelector = getSelectorByName(messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(messages)
    const facetCut = [{hash: psp22Hash, selectors: psp22Selectors}]

    // initialize diamond contract
    const { contract: diamondContract, alice, close: closeDiamond } = await setupDiamond(defaultSigner.address)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut(facetCut, {hash: psp22Hash, selector: initSelector, input: []})

    // we will upgrade to psp22_2
    const { contract: newPsp22, abi: newPsp22Abi, close: closePSP22V2 } = await setupPSP22FacetV2()

    // first we need to remove psp22 and replace with psp22_2
    const facetCutRemove = [{hash: psp22Hash, selectors: []}]
    await diamondContract.withSigner(defaultSigner).tx.diamondCut(facetCutRemove, null)

    const psp22NewHash = newPsp22Abi.info.source.wasmHash.toString()
    const messagesNew = newPsp22Abi.messages

    const psp22NewSelectors = getSelectorsFromMessages(messagesNew)
    const facetCutNew = [{hash: psp22NewHash, selectors: psp22NewSelectors}]

    // add new psp22 facet, without init function
    await diamondContract.withSigner(defaultSigner).tx.diamondCut(facetCutNew, null)

    // patch methods
    const proxy = setupProxy(newPsp22, diamondContract.address)

    // if we send 1000 tokens total supply will be 900
    await proxy.withSigner(defaultSigner).tx.transfer(alice.address, 1000, [])
    await expect(proxy.query.totalSupply()).to.bnToNumber(900)

    expect((await diamondContract.query.facetFunctionSelectors(psp22NewHash)).value).to.be.deep.equal(getSelectorsFromMessagesString(messagesNew))
    expect((await diamondContract.query.facetFunctionSelectors(psp22Hash)).value).to.be.deep.equal([])

    expect((await diamondContract.query.facetCodeHashes()).value).to.be.deep.equal([psp22NewHash])

    await expect(diamondContract.query.facetCodeHash(initSelector)).to.output(null)

    await closePSP22()
    await closePSP22V2()
    await closeDiamond()
  })

  it('Can not replace existing selector', async () => {
    // abi of psp22 facet
    const { abi, defaultSigner, close: closePSP22 } = await setupPSP22Facet()

    const psp22Hash = abi.info.source.wasmHash.toString()
    const messages = abi.messages

    const initSelector = getSelectorByName(messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(messages)
    const facetCut = [{hash: psp22Hash, selectors: psp22Selectors}]

    // initialize diamond contract
    const { contract: diamondContract, close: closeDiamond } = await setupDiamond(defaultSigner.address)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut(facetCut, {hash: psp22Hash, selector: initSelector, input: []})

    const { abi: newPsp22Abi, close: closePSP22V2 } = await setupPSP22FacetV2()
    const hashReplace = newPsp22Abi.info.source.wasmHash.toString()
    const messagesReplace = newPsp22Abi.messages

    const replaceSelector = getSelectorByName(messagesReplace, 'PSP22::transfer_from')
    const facetCutReplace = [{hash: hashReplace, selectors: [replaceSelector]}]

    // replace functions
    await expect(diamondContract.withSigner(defaultSigner).tx.diamondCut(facetCutReplace, null)).to.eventually.be.rejected

    await closePSP22()
    await closePSP22V2()
    await closeDiamond()
  })

  it('Removing facets works', async () => {
    // abi of psp22 facet
    const { contract: psp22Facet, abi, defaultSigner, close: closePSP22 } = await setupPSP22Facet()

    const psp22Hash = abi.info.source.wasmHash.toString()
    const messages = abi.messages

    const initSelector = getSelectorByName(messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(messages)
    const facetCut = [{hash: psp22Hash, selectors: psp22Selectors}]

    // initialize diamond contract
    const { contract: diamondContract, close: closeDiamond } = await setupDiamond(defaultSigner.address)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut(facetCut, {hash: psp22Hash, selector: initSelector, input: []})

    // patch methods
    const proxy = setupProxy(psp22Facet, diamondContract.address)

    // we called init function which mints tokens and sets owner
    await expect(proxy.query.balanceOf(defaultSigner.address)).to.bnToNumber(1000)

    // we will remove the psp22 facet
    const facetCutRemove = [{hash: psp22Hash, selectors: []}]

    // remove facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut(facetCutRemove, null)

    expect((await diamondContract.query.facetFunctionSelectors(psp22Hash)).value).to.be.deep.equal([])

    expect((await diamondContract.query.facetCodeHashes()).value).to.be.deep.equal([])

    await expect(diamondContract.query.facetCodeHash(initSelector)).to.output(null)

    await closePSP22()
    await closeDiamond()
  })

  it('Removing last facet will leave us first facet', async () => {
    // abi of psp22 facet
    const { abi, defaultSigner, close: closePSP22 } = await setupPSP22Facet()

    const psp22Hash = abi.info.source.wasmHash.toString()
    const messages = abi.messages

    const initSelector = getSelectorByName(messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(messages)
    const facetCut = [{hash: psp22Hash, selectors: psp22Selectors}]

    // initialize diamond contract
    const { contract: diamondContract, close: closeDiamond } = await setupDiamond(defaultSigner.address)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut(facetCut, {hash: psp22Hash, selector: initSelector, input: []})

    // add metadata facet
    const { abi: metadataAbi, close: closePSP22Metadata } = await setupPSP22MetadataFacet()

    const metadataHash = metadataAbi.info.source.wasmHash.toString()
    const metadataMessages = metadataAbi.messages

    const metadataInit = getSelectorByName(metadataMessages, 'init_metadata')
    const metadataSelectors = getSelectorsFromMessages(
      metadataMessages.filter((message) => {
        return message.identifier != 'Ownable::owner' && message.identifier != 'Ownable::renounce_ownership' && message.identifier != 'Ownable::transfer_ownership'
      })
    )

    const metadataCut = [{hash: metadataHash, selectors: metadataSelectors}]

    // add metadata facet
    await expect(diamondContract.withSigner(defaultSigner).tx.diamondCut(metadataCut, {hash: metadataHash, selector: metadataInit, input: []})).to.eventually.be
      .fulfilled

    // we will remove the metadata facet
    const facetCutRemove = [{hash: metadataHash, selectors: []}]

    // remove facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut(facetCutRemove, null)

    expect((await diamondContract.query.facetCodeHashes()).value).to.be.deep.equal([psp22Hash])

    expect((await diamondContract.query.facetFunctionSelectors(psp22Hash)).value).to.be.deep.equal(getSelectorsFromMessagesString(messages))
    await expect(diamondContract.query.facetCodeHash(initSelector)).to.output(psp22Hash)

    expect((await diamondContract.query.facetFunctionSelectors(metadataHash)).value).to.be.deep.equal([])
    await expect(diamondContract.query.facetCodeHash(metadataInit)).to.output(null)

    await closePSP22()
    await closePSP22Metadata()
    await closeDiamond()
  })

  it('Removing first facet will leave us last facet', async () => {
    // abi of psp22 facet
    const { abi, defaultSigner, close: closePSP22 } = await setupPSP22Facet()

    const psp22Hash = abi.info.source.wasmHash.toString()
    const messages = abi.messages

    const initSelector = getSelectorByName(messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(messages)
    const facetCut = [{hash: psp22Hash, selectors: psp22Selectors}]

    // initialize diamond contract
    const { contract: diamondContract, close: closeDiamond } = await setupDiamond(defaultSigner.address)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut(facetCut, {hash: psp22Hash, selector: initSelector, input: []})

    // add metadata facet
    const { abi: metadataAbi, close: closePSP22Metadata } = await setupPSP22MetadataFacet()

    const metadataHash = metadataAbi.info.source.wasmHash.toString()
    const metadataMessages = metadataAbi.messages

    const metadataInit = getSelectorByName(metadataMessages, 'init_metadata')
    const metadataSelectors = getSelectorsFromMessages(
      metadataMessages.filter((message) => {
        return message.identifier != 'Ownable::owner' && message.identifier != 'Ownable::renounce_ownership' && message.identifier != 'Ownable::transfer_ownership'
      })
    )

    const metadataCut = [{hash: metadataHash, selectors: metadataSelectors}]

    // add metadata facet
    await expect(diamondContract.withSigner(defaultSigner).tx.diamondCut(metadataCut, {hash: metadataHash, selector: metadataInit, input: []})).to.eventually.be

    // we will remove the psp22 facet
    const facetCutRemove = [{hash: psp22Hash, selectors: []}]

    // remove facet
    await diamondContract.withSigner(defaultSigner).tx.diamondCut(facetCutRemove, null)

    expect((await diamondContract.query.facetCodeHashes()).value).to.be.deep.equal([metadataHash])

    expect((await diamondContract.query.facetFunctionSelectors(metadataHash)).value).to.be.deep.equal(getSelectorsFromMessagesString(metadataMessages))
    await expect(diamondContract.query.facetCodeHash(initSelector)).to.output(null)

    expect((await diamondContract.query.facetFunctionSelectors(metadataHash)).value).to.be.deep.equal(getSelectorsFromMessagesString(metadataMessages))
    expect((await diamondContract.query.facetCodeHash(metadataInit)).value).to.be.deep.equal(metadataHash)

    await closePSP22()
    await closePSP22Metadata()
    await closeDiamond()
  })

  it('Can call facet function via PSP22Ref', async () => {
    // abi of psp22 facet
    const { contract: psp22Facet, abi, defaultSigner, alice, close: closePSP22 } = await setupPSP22Facet()

    const psp22Hash = abi.info.source.wasmHash.toString()
    const psp22Messages = abi.messages

    const psp22Init = getSelectorByName(psp22Messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(psp22Messages)
    const psp22Cut = [{hash: psp22Hash, selectors: psp22Selectors}]

    // initialize diamond contract
    const { contract: diamondContract, close: closeDiamond } = await setupDiamond(defaultSigner.address)
    const proxy = setupProxy(psp22Facet, diamondContract.address)

    // add psp22 facet
    await expect(diamondContract.withSigner(defaultSigner).tx.diamondCut(psp22Cut, {hash: psp22Hash, selector: psp22Init, input: []}))
      .to.eventually.be.fulfilled

    // we will instantiate the caller contract with which we try to call PSP22Ref on the diamond contract
    const { contract: diamondCaller, close: closeDiamondCaller } = await setupDiamondCaller()

    await expect(diamondCaller.query.balanceOf(diamondContract.address, defaultSigner.address)).to.bnToNumber(1000)

    // we will give allowance to caller contract
    await proxy.withSigner(defaultSigner).tx.approve(diamondCaller.address, 1000)
    // calling transfer via diamondCaller should transfer balance
    await expect(diamondCaller.withSigner(defaultSigner).tx.transfer(diamondContract.address, alice.address, 1000)).to.eventually.be
      .fulfilled

    // calling diamondCaller.balanceOf should give us the right balance
    await expect(diamondCaller.query.balanceOf(diamondContract.address, defaultSigner.address)).to.bnToNumber(0)
    await expect(diamondCaller.query.balanceOf(diamondContract.address, alice.address)).to.bnToNumber(1000)

    await closePSP22()
    await closeDiamond()
    await closeDiamondCaller()
  })

  it('Can not call function after removing it', async () => {
    // abi of psp22 facet
    const { contract: psp22Facet, abi, defaultSigner, close: closePSP22 } = await setupPSP22Facet()

    const psp22Hash = abi.info.source.wasmHash.toString()
    const psp22Messages = abi.messages

    const psp22Init = getSelectorByName(psp22Messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(psp22Messages)
    const psp22Cut = [{hash: psp22Hash, selectors: psp22Selectors}]

    // initialize diamond contract
    const { contract: diamondContract, close: closeDiamond } = await setupDiamond(defaultSigner.address)
    const proxy = setupProxy(psp22Facet, diamondContract.address)

    // add psp22 facet
    await expect(diamondContract.withSigner(defaultSigner).tx.diamondCut(psp22Cut, {hash: psp22Hash, selector: psp22Init, input: []}))
      .to.eventually.be.fulfilled

    const filteredSelectors = psp22Selectors.filter((selector) => {
      return selector.toString() != psp22Init.toString()
    })
    const removalCut = [{hash: psp22Hash, selectors: filteredSelectors}]

    await diamondContract.withSigner(defaultSigner).tx.diamondCut(removalCut, null)

    // we will remove the init function and after that we should not be able to call it
    await expect(proxy.withSigner(defaultSigner).tx.initPsp22()).to.eventually.be.rejected

    expect((await diamondContract.query.facetFunctionSelectors(psp22Hash)).value).to.be.deep.equal(getSelectorsFromMessagesString(psp22Messages).filter((selector) => {
      return selector != getSelectorByNameString(psp22Messages, 'init_psp22')
    }))

    await closePSP22()
    await closeDiamond()
  })

  it('Can not perform diamond cut with empty hash', async () => {
    // abi of psp22 facet
    const { abi, defaultSigner, close: closePSP22 } = await setupPSP22Facet()

    const psp22Messages = abi.messages

    const psp22Selectors = getSelectorsFromMessages(psp22Messages)
    const psp22Cut = [{hash: '', selectors: psp22Selectors}]

    // initialize diamond contract
    const { contract: diamondContract, close: closeDiamond } = await setupDiamond(defaultSigner.address)

    // add psp22 facet
    await expect(diamondContract.withSigner(defaultSigner).tx.diamondCut(psp22Cut, null)).to.eventually.be.rejected

    await closePSP22()
    await closeDiamond()
  })
})
