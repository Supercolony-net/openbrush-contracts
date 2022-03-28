import { consts } from '../constants'
import { hexToU8a, stringToU8a } from '@polkadot/util'
import { expect, setupContract, fromSigner, setupProxy } from '../helpers'

const ADD = 0
const REPLACE = 1
const REMOVE = 2

describe('DIAMOND_PSP22', () => {
  it('Adding facets work', async () => {
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
      return [psp22Hash, [[message.selector, ADD]]]
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

    // add metadata to contract
    const { contract: metadataFacet, abi: metadataAbi } = await setupContract('my_psp22_metadata_facet', 'new')

    let metadataHash = (await metadataAbi).source.hash
    messages = (await metadataAbi).V3.spec.messages

    facetCut = messages
      .filter((message) => {
        return message.label != 'Ownable::owner' && message.label != 'Ownable::renounce_ownership' && message.label != 'Ownable::transfer_ownership'
      })
      .map((message) => {
        if (message.label == 'init_metadata') {
          initSelector = message.selector
        }
        return [metadataHash, [[message.selector, ADD]]]
      })

    // add metadata facet
    await expect(fromSigner(diamondContract, defaultSigner.address).tx.diamondCut(facetCut, [metadataHash, initSelector, []])).to.eventually.be
      .fulfilled

    // patch methods
    proxy = setupProxy(metadataFacet, diamondContract)

    await expect(proxy.query.tokenName()).to.output('PSP22 Diamond')
    await expect(proxy.query.tokenSymbol()).to.output('PSP22D')
    await expect(proxy.query.tokenDecimals()).to.output(18)
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
      return [psp22Hash, [[message.selector, ADD]]]
    })

    // initialize diamond contract
    const { contract: diamondContract, defaultSigner: wrongSigner } = await setupContract('my_diamond', 'new', defaultSigner.address, diamondHash)

    // add psp22 facet
    await expect(fromSigner(diamondContract, wrongSigner.address).tx.diamondCut(facetCut, [psp22Hash, initSelector, []])).to.eventually.be.rejected
  })
})
