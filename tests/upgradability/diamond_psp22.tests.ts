import { consts } from '../constants'
import { hexToU8a } from '@polkadot/util'
import { expect, setupContract, fromSigner, setupProxy } from '../helpers'

const ADD = 0
const REPLACE = 1
const REMOVE = 2

describe('DIAMOND_PSP22', () => {
  it('We can call functions on the diamond after adding facets', async () => {
    // get abi of diamond
    const { abi: diamondAbi } = await setupContract('my_diamond', 'new', consts.EMPTY_ADDRESS, '')
    const diamondHash = (await diamondAbi).source.hash

    // abi of psp22 facet
    const { contract: psp22Facet, abi, defaultSigner } = await setupContract('my_psp22_facet', 'new')

    let psp22Hash = (await abi).source.hash
    let messages = (await abi).V3.spec.messages

    let initSelector

    let facetCut = messages.map((message) => {
      if (message.label == 'initialize') {
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
    const proxy = setupProxy(psp22Facet, diamondContract)

    // we called init function which mints tokens and sets owner
    await expect(proxy.query.balanceOf(defaultSigner.address)).to.output(1000)
    await expect(proxy.query.owner()).to.output(defaultSigner.address)
  })
})
