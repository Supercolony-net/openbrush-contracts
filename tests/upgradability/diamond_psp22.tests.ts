import { consts } from '../constants'
import { expect, setupContract, fromSigner, setupProxy } from '../helpers'

describe('DIAMOND_PSP22', () => {
  it('Adding facets works', async () => {
    // get abi of diamond
    const { abi: diamondAbi } = await setupContract('my_diamond', 'new', consts.EMPTY_ADDRESS, '')
    const diamondHash = (await diamondAbi).source.hash

    // abi of psp22 facet
    const { contract: psp22Facet, abi, defaultSigner } = await setupContract('my_psp22_facet', 'new')

    const psp22Hash = (await abi).source.hash
    const psp22Messages = (await abi).V3.spec.messages

    let psp22Init

    const psp22Cut = psp22Messages.map((message) => {
      if (message.label == 'init_psp22') {
        psp22Init = message.selector
      }
      return [psp22Hash, [message.selector]]
    })

    // initialize diamond contract
    const { contract: diamondContract } = await setupContract('my_diamond', 'new', defaultSigner.address, diamondHash)

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
    let metadataInit

    const metadataCut = metadataMessages
      .filter((message) => {
        return message.label != 'Ownable::owner' && message.label != 'Ownable::renounce_ownership' && message.label != 'Ownable::transfer_ownership'
      })
      .map((message) => {
        if (message.label == 'init_metadata') {
          metadataInit = message.selector
        }
        return [metadataHash, [message.selector]]
      })

    // add metadata facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(metadataCut, [metadataHash, metadataInit, []])).to.eventually.be
      .fulfilled

    // patch methods
    proxy = setupProxy(metadataFacet, diamondContract)

    await expect(proxy.query.tokenName()).to.output('PSP22 Diamond')
    await expect(proxy.query.tokenSymbol()).to.output('PSP22D')
    await expect(proxy.query.tokenDecimals()).to.output(18)

    // Test Loupe
    let metadataSelectors = metadataMessages
      .filter((message) => {
        return message.label != 'Ownable::owner' && message.label != 'Ownable::renounce_ownership' && message.label != 'Ownable::transfer_ownership'
      })
      .map((message) => {
        return message.selector
      })

    const psp22Selectors = psp22Messages.map((message) => {
      return message.selector
    })

    await expect(diamondContract.query.facets()).to.output([
      [psp22Hash, psp22Selectors],
      [metadataHash, metadataSelectors]
    ])

    await expect(diamondContract.query.facetFunctionSelectors(metadataHash)).to.output(metadataSelectors)
    await expect(diamondContract.query.facetFunctionSelectors(psp22Hash)).to.output(psp22Selectors)

    await expect(diamondContract.query.facetCodeHashes()).to.output([psp22Hash, metadataHash])

    await expect(diamondContract.query.facetCodeHash(psp22Init)).to.output(psp22Hash)
    await expect(diamondContract.query.facetCodeHash(metadataInit)).to.output(metadataHash)
  })

  it('Only owner can call diamond cut', async () => {
    // get abi of diamond
    const { abi: diamondAbi } = await setupContract('my_diamond', 'new', consts.EMPTY_ADDRESS, '')
    const diamondHash = (await diamondAbi).source.hash

    // abi of psp22 facet
    const { abi, defaultSigner } = await setupContract('my_psp22_facet', 'new')

    let psp22Hash = (await abi).source.hash
    let messages = (await abi).V3.spec.messages

    let initSelector

    let facetCut = messages.map((message) => {
      if (message.label == 'init_psp22') {
        initSelector = message.selector
      }
      return [psp22Hash, [message.selector]]
    })

    // initialize diamond contract
    const { contract: diamondContract, defaultSigner: wrongSigner } = await setupContract('my_diamond', 'new', defaultSigner.address, diamondHash)

    // add psp22 facet
    await expect(fromSigner(diamondContract, wrongSigner.address).tx.diamondCut(facetCut, [psp22Hash, initSelector, []])).to.eventually.be.rejected
  })

  it('Replacing facets works', async () => {
    // get abi of diamond
    const { abi: diamondAbi } = await setupContract('my_diamond', 'new', consts.EMPTY_ADDRESS, '')
    const diamondHash = (await diamondAbi).source.hash

    // abi of psp22 facet
    const { abi, defaultSigner } = await setupContract('my_psp22_facet', 'new')

    let psp22Hash = (await abi).source.hash
    let messages = (await abi).V3.spec.messages

    let initSelector

    let facetCut = messages.map((message) => {
      if (message.label == 'init_psp22') {
        initSelector = message.selector
      }
      return [psp22Hash, [message.selector]]
    })

    // initialize diamond contract
    const { contract: diamondContract, alice } = await setupContract('my_diamond', 'new', defaultSigner.address, diamondHash)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCut, [psp22Hash, initSelector, []])).to.eventually.be.fulfilled

    // we will upgrade to psp22_2
    const { contract: newPsp22, abi: newPsp22Abi } = await setupContract('my_psp22_2_facet', 'new')

    let psp22NewHash = (await newPsp22Abi).source.hash
    let messagesNew = (await newPsp22Abi).V3.spec.messages

    facetCut = messagesNew.map((message) => {
      return [psp22NewHash, [message.selector]]
    })

    const psp22NewSelectors = messagesNew.map((message) => {
      return message.selector
    })

    // add new psp22 facet, without init function
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCut, null)).to.eventually.be.fulfilled

    // patch methods
    let proxy = setupProxy(newPsp22, diamondContract)

    // if we send 1000 tokens total supply will be 900
    await expect(fromSigner(proxy, defaultSigner.address).tx.transfer(alice.address, 1000, [])).to.eventually.be.fulfilled
    await expect(proxy.query.totalSupply()).to.output(900)

    await expect(diamondContract.query.facetFunctionSelectors(psp22NewHash)).to.output(psp22NewSelectors)
    await expect(diamondContract.query.facetFunctionSelectors(psp22Hash)).to.output([])

    await expect(diamondContract.query.facetCodeHashes()).to.output([psp22NewHash])

    await expect(diamondContract.query.facetCodeHash(initSelector)).to.output(null)
  })

  it('We can not replace facet with the same facet', async () => {
    // get abi of diamond
    const { abi: diamondAbi } = await setupContract('my_diamond', 'new', consts.EMPTY_ADDRESS, '')
    const diamondHash = (await diamondAbi).source.hash

    // abi of psp22 facet
    const { abi, defaultSigner } = await setupContract('my_psp22_facet', 'new')

    let psp22Hash = (await abi).source.hash
    let messages = (await abi).V3.spec.messages

    let initSelector

    let facetCut = messages.map((message) => {
      if (message.label == 'init_psp22') {
        initSelector = message.selector
      }
      return [psp22Hash, [message.selector]]
    })

    // initialize diamond contract
    const { contract: diamondContract } = await setupContract('my_diamond', 'new', defaultSigner.address, diamondHash)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCut, [psp22Hash, initSelector, []])).to.eventually.be.fulfilled

    facetCut = messages.map((message) => {
      if (message.label == 'init_psp22') {
        initSelector = message.selector
      }
      return [psp22Hash, [message.selector]]
    })

    // replace functions
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCut, null)).to.eventually.be.rejected
  })

  it('Removing facets works', async () => {
    // get abi of diamond
    const { abi: diamondAbi } = await setupContract('my_diamond', 'new', consts.EMPTY_ADDRESS, '')
    const diamondHash = (await diamondAbi).source.hash

    // abi of psp22 facet
    const { contract: psp22Facet, abi, defaultSigner } = await setupContract('my_psp22_facet', 'new')

    let psp22Hash = (await abi).source.hash
    let messages = (await abi).V3.spec.messages

    let initSelector

    let facetCut = messages.map((message) => {
      if (message.label == 'init_psp22') {
        initSelector = message.selector
      }
      return [psp22Hash, [message.selector]]
    })

    // initialize diamond contract
    const { contract: diamondContract } = await setupContract('my_diamond', 'new', defaultSigner.address, diamondHash)

    await expect(diamondContract.query.owner()).to.output(defaultSigner.address)

    // add psp22 facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCut, [psp22Hash, initSelector, []])).to.eventually.be.fulfilled

    // patch methods
    let proxy = setupProxy(psp22Facet, diamondContract)

    // we called init function which mints tokens and sets owner
    await expect(proxy.query.balanceOf(defaultSigner.address)).to.output(1000)
    await expect(proxy.query.owner()).to.output(defaultSigner.address)

    // we will remove the psp22 facet
    facetCut = [[psp22Hash, []]]

    // remove facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCut, null)).to.eventually.be.fulfilled

    await expect(diamondContract.query.facetFunctionSelectors(psp22Hash)).to.output([])

    await expect(diamondContract.query.facetCodeHashes()).to.output([])

    await expect(diamondContract.query.facetCodeHash(initSelector)).to.output(null)
  })
})
