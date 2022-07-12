import { consts } from '../constants'
import { expect, setupContract, fromSigner, setupProxy } from '../helpers'

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

describe('DIAMOND_PSP22', () => {
  it('Adding facets works', async () => {
    // abi of psp22 facet
    const { contract: psp22Facet, abi, defaultSigner } = await setupContract('my_psp22_facet_v1', 'new')

    const psp22Hash = (await abi).source.hash
    const psp22Messages = (await abi).V3.spec.messages

    const psp22Init = getSelectorByName(psp22Messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(psp22Messages)
    const psp22Cut = [[psp22Hash, psp22Selectors]]

    // initialize diamond contract
    const { contract: diamondContract } = await setupContract('my_diamond', 'new', defaultSigner.address)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(psp22Cut, [psp22Hash, psp22Init, []])).to.eventually.be.fulfilled

    // patch methods
    let proxy = setupProxy(psp22Facet, diamondContract)

    // we called init function which mints tokens and sets owner
    await expect(proxy.query.balanceOf(defaultSigner.address)).to.output(1000)
    await expect(proxy.query.owner()).to.output(defaultSigner.address)

    // add metadata to contract
    const { contract: metadataFacet, abi: metadataAbi } = await setupContract('my_psp22_metadata_facet', 'new')

    const metadataHash = (await metadataAbi).source.hash
    const metadataMessages = (await metadataAbi).V3.spec.messages

    const metadataInit = getSelectorByName(metadataMessages, 'init_metadata')
    const metadataSelectors = getSelectorsFromMessages(
      metadataMessages.filter((message) => {
        return message.label != 'Ownable::owner' && message.label != 'Ownable::renounce_ownership' && message.label != 'Ownable::transfer_ownership'
      })
    )

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

  it('Only owner can call diamond cut', async () => {
    // abi of psp22 facet
    const { abi, defaultSigner } = await setupContract('my_psp22_facet_v1', 'new')

    const psp22Hash = (await abi).source.hash
    const messages = (await abi).V3.spec.messages

    const initSelector = getSelectorByName(messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(messages)
    const facetCut = [[psp22Hash, psp22Selectors]]

    // initialize diamond contract
    const { contract: diamondContract, defaultSigner: wrongSigner } = await setupContract('my_diamond', 'new', defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, wrongSigner.address).tx.diamondCut(facetCut, [psp22Hash, initSelector, []])).to.eventually.be.rejected
  })

  it('Replacing facets works', async () => {
    // abi of psp22 facet
    const { abi, defaultSigner } = await setupContract('my_psp22_facet_v1', 'new')

    const psp22Hash = (await abi).source.hash
    const messages = (await abi).V3.spec.messages

    const initSelector = getSelectorByName(messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(messages)
    const facetCut = [[psp22Hash, psp22Selectors]]

    // initialize diamond contract
    const { contract: diamondContract, alice } = await setupContract('my_diamond', 'new', defaultSigner.address)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCut, [psp22Hash, initSelector, []])).to.eventually.be.fulfilled

    // we will upgrade to psp22_2
    const { contract: newPsp22, abi: newPsp22Abi } = await setupContract('my_psp22_facet_v2', 'new')

    // first we need to remove psp22 and replace with psp22_2
    const facetCutRemove = [[psp22Hash, []]]
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCutRemove, null)).to.eventually.be.fulfilled

    const psp22NewHash = (await newPsp22Abi).source.hash
    const messagesNew = (await newPsp22Abi).V3.spec.messages

    const psp22NewSelectors = getSelectorsFromMessages(messagesNew)
    const facetCutNew = [[psp22NewHash, psp22NewSelectors]]

    // add new psp22 facet, without init function
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCutNew, null)).to.eventually.be.fulfilled

    // patch methods
    const proxy = setupProxy(newPsp22, diamondContract)

    // if we send 1000 tokens total supply will be 900
    await expect(fromSigner(proxy, defaultSigner.address).tx.transfer(alice.address, 1000, [])).to.eventually.be.fulfilled
    await expect(proxy.query.totalSupply()).to.output(900)

    await expect(diamondContract.query.facetFunctionSelectors(psp22NewHash)).to.output(psp22NewSelectors)
    await expect(diamondContract.query.facetFunctionSelectors(psp22Hash)).to.output([])

    await expect(diamondContract.query.facetCodeHashes()).to.output([psp22NewHash])

    await expect(diamondContract.query.facetCodeHash(initSelector)).to.output(null)
  })

  it('Can not replace existing selector', async () => {
    // abi of psp22 facet
    const { abi, defaultSigner } = await setupContract('my_psp22_facet_v1', 'new')

    const psp22Hash = (await abi).source.hash
    const messages = (await abi).V3.spec.messages

    const initSelector = getSelectorByName(messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(messages)
    const facetCut = [[psp22Hash, psp22Selectors]]

    // initialize diamond contract
    const { contract: diamondContract } = await setupContract('my_diamond', 'new', defaultSigner.address)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCut, [psp22Hash, initSelector, []])).to.eventually.be.fulfilled

    const { abi: newPsp22Abi } = await setupContract('my_psp22_facet_v2', 'new')
    const hashReplace = (await newPsp22Abi).source.hash
    const messagesReplace = (await newPsp22Abi).V3.spec.messages

    const replaceSelector = getSelectorByName(messagesReplace, 'PSP22::transfer_from')
    const facetCutReplace = [[hashReplace, [replaceSelector]]]

    // replace functions
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCutReplace, null)).to.eventually.be.rejected
  })

  it('Removing facets works', async () => {
    // abi of psp22 facet
    const { contract: psp22Facet, abi, defaultSigner } = await setupContract('my_psp22_facet_v1', 'new')

    const psp22Hash = (await abi).source.hash
    const messages = (await abi).V3.spec.messages

    const initSelector = getSelectorByName(messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(messages)
    const facetCut = [[psp22Hash, psp22Selectors]]

    // initialize diamond contract
    const { contract: diamondContract } = await setupContract('my_diamond', 'new', defaultSigner.address)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCut, [psp22Hash, initSelector, []])).to.eventually.be.fulfilled

    // patch methods
    const proxy = setupProxy(psp22Facet, diamondContract)

    // we called init function which mints tokens and sets owner
    await expect(proxy.query.balanceOf(defaultSigner.address)).to.output(1000)
    await expect(proxy.query.owner()).to.output(defaultSigner.address)

    // we will remove the psp22 facet
    const facetCutRemove = [[psp22Hash, []]]

    // remove facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCutRemove, null)).to.eventually.be.fulfilled

    await expect(diamondContract.query.facetFunctionSelectors(psp22Hash)).to.output([])

    await expect(diamondContract.query.facetCodeHashes()).to.output([])

    await expect(diamondContract.query.facetCodeHash(initSelector)).to.output(null)
  })

  it('Removing last facet will leave us first facet', async () => {
    // abi of psp22 facet
    const { abi, defaultSigner } = await setupContract('my_psp22_facet_v1', 'new')

    const psp22Hash = (await abi).source.hash
    const messages = (await abi).V3.spec.messages

    const initSelector = getSelectorByName(messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(messages)
    const facetCut = [[psp22Hash, psp22Selectors]]

    // initialize diamond contract
    const { contract: diamondContract } = await setupContract('my_diamond', 'new', defaultSigner.address)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCut, [psp22Hash, initSelector, []])).to.eventually.be.fulfilled

    // add metadata facet
    const { abi: metadataAbi } = await setupContract('my_psp22_metadata_facet', 'new')

    const metadataHash = (await metadataAbi).source.hash
    const metadataMessages = (await metadataAbi).V3.spec.messages

    const metadataInit = getSelectorByName(metadataMessages, 'init_metadata')
    const metadataSelectors = getSelectorsFromMessages(
      metadataMessages.filter((message) => {
        return message.label != 'Ownable::owner' && message.label != 'Ownable::renounce_ownership' && message.label != 'Ownable::transfer_ownership'
      })
    )

    const metadataCut = [[metadataHash, metadataSelectors]]

    // add metadata facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(metadataCut, [metadataHash, metadataInit, []])).to.eventually.be
      .fulfilled

    // we will remove the metadata facet
    const facetCutRemove = [[metadataHash, []]]

    // remove facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCutRemove, null)).to.eventually.be.fulfilled

    await expect(diamondContract.query.facetCodeHashes()).to.output([psp22Hash])

    await expect(diamondContract.query.facetFunctionSelectors(psp22Hash)).to.output(psp22Selectors)
    await expect(diamondContract.query.facetCodeHash(initSelector)).to.output(psp22Hash)

    await expect(diamondContract.query.facetFunctionSelectors(metadataHash)).to.output([])
    await expect(diamondContract.query.facetCodeHash(metadataInit)).to.output(null)
  })

  it('Removing first facet will leave us last facet', async () => {
    // abi of psp22 facet
    const { abi, defaultSigner } = await setupContract('my_psp22_facet_v1', 'new')

    const psp22Hash = (await abi).source.hash
    const messages = (await abi).V3.spec.messages

    const initSelector = getSelectorByName(messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(messages)
    const facetCut = [[psp22Hash, psp22Selectors]]

    // initialize diamond contract
    const { contract: diamondContract } = await setupContract('my_diamond', 'new', defaultSigner.address)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCut, [psp22Hash, initSelector, []])).to.eventually.be.fulfilled

    // add metadata facet
    const { abi: metadataAbi } = await setupContract('my_psp22_metadata_facet', 'new')

    const metadataHash = (await metadataAbi).source.hash
    const metadataMessages = (await metadataAbi).V3.spec.messages

    const metadataInit = getSelectorByName(metadataMessages, 'init_metadata')
    const metadataSelectors = getSelectorsFromMessages(
      metadataMessages.filter((message) => {
        return message.label != 'Ownable::owner' && message.label != 'Ownable::renounce_ownership' && message.label != 'Ownable::transfer_ownership'
      })
    )

    const metadataCut = [[metadataHash, metadataSelectors]]

    // add metadata facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(metadataCut, [metadataHash, metadataInit, []])).to.eventually.be
      .fulfilled

    // we will remove the psp22 facet
    const facetCutRemove = [[psp22Hash, []]]

    // remove facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCutRemove, null)).to.eventually.be.fulfilled

    await expect(diamondContract.query.facetCodeHashes()).to.output([metadataHash])

    await expect(diamondContract.query.facetFunctionSelectors(psp22Hash)).to.output([])
    await expect(diamondContract.query.facetCodeHash(initSelector)).to.output(null)

    await expect(diamondContract.query.facetFunctionSelectors(metadataHash)).to.output(metadataSelectors)
    await expect(diamondContract.query.facetCodeHash(metadataInit)).to.output(metadataHash)
  })

  it('Can call facet function via PSP22Ref', async () => {
    // abi of psp22 facet
    const { contract: psp22Facet, abi, defaultSigner, alice } = await setupContract('my_psp22_facet_v1', 'new')

    const psp22Hash = (await abi).source.hash
    const psp22Messages = (await abi).V3.spec.messages

    const psp22Init = getSelectorByName(psp22Messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(psp22Messages)
    const psp22Cut = [[psp22Hash, psp22Selectors]]

    // initialize diamond contract
    const { contract: diamondContract } = await setupContract('my_diamond', 'new', defaultSigner.address)
    const proxy = setupProxy(psp22Facet, diamondContract)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(psp22Cut, [psp22Hash, psp22Init, []])).to.eventually.be.fulfilled

    // we will instantiate the caller contract with which we try to call PSP22Ref on the diamond contract
    const { contract: diamondCaller } = await setupContract('diamond_caller', 'new')

    await expect(diamondCaller.query.balanceOf(diamondContract.address, defaultSigner.address)).to.output(1000)

    // we will give allowance to caller contract
    await expect(fromSigner(proxy, defaultSigner.address).tx.approve(diamondCaller.address, 1000)).to.eventually.be.fulfilled
    // calling transfer via diamondCaller should transfer balance
    await expect(fromSigner(diamondCaller, defaultSigner.address).tx.transfer(diamondContract.address, alice.address, 1000)).to.eventually.be
      .fulfilled

    // calling diamondCaller.balanceOf should give us the right balance
    await expect(diamondCaller.query.balanceOf(diamondContract.address, defaultSigner.address)).to.output(0)
    await expect(diamondCaller.query.balanceOf(diamondContract.address, alice.address)).to.output(1000)
  })

  it('Can not call function after removing it', async () => {
    // abi of psp22 facet
    const { contract: psp22Facet, abi, defaultSigner } = await setupContract('my_psp22_facet_v1', 'new')

    const psp22Hash = (await abi).source.hash
    const psp22Messages = (await abi).V3.spec.messages

    const psp22Init = getSelectorByName(psp22Messages, 'init_psp22')
    const psp22Selectors = getSelectorsFromMessages(psp22Messages)
    const psp22Cut = [[psp22Hash, psp22Selectors]]

    // initialize diamond contract
    const { contract: diamondContract } = await setupContract('my_diamond', 'new', defaultSigner.address)
    const proxy = setupProxy(psp22Facet, diamondContract)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(psp22Cut, [psp22Hash, psp22Init, []])).to.eventually.be.fulfilled

    const filteredSelectors = psp22Selectors.filter((selector) => {
      return selector != psp22Init
    })
    const removalCut = [[psp22Hash, filteredSelectors]]

    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(removalCut, null)).to.eventually.be.fulfilled

    // we will remove the init function and after that we should not be able to call it
    await expect(fromSigner(proxy, defaultSigner.address).tx.initPsp22()).to.eventually.be.rejected
    await expect(diamondContract.query.facetFunctionSelectors(psp22Hash)).to.output(filteredSelectors)
  })

  it('Can not perform diamond cut with empty hash', async () => {
    // abi of psp22 facet
    const { abi, defaultSigner } = await setupContract('my_psp22_facet_v1', 'new')

    const psp22Messages = (await abi).V3.spec.messages

    const psp22Selectors = getSelectorsFromMessages(psp22Messages)
    const psp22Cut = [[null, psp22Selectors]]

    // initialize diamond contract
    const { contract: diamondContract } = await setupContract('my_diamond', 'new', defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(psp22Cut, null)).to.eventually.be.rejected
  })
})