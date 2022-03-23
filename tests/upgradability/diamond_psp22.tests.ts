import { consts } from '../constants'
import { expect, setupContract, getSigner, fromSigner, setupProxy } from '../helpers'

const ADD = 0
const REPLACE = 1
const REMOVE = 2

describe('DIAMOND_PSP22', () => {
  it('Added facet can call the contract', async () => {
    // get abi of diamond
    let signerAddress = '0x01e552298e47454041ea31273b4b630c64c104e4514aa3643490b8aaca9cf8ed' //(await getSigner('Alice')).address
    const { abi: diamondAbi } = await setupContract('my_diamond', 'new', [], signerAddress, signerAddress)
    const diamondHash = (await diamondAbi).source.hash

    // abi of psp22 facet
    const { contract: psp22Facet, abi, defaultSigner } = await setupContract('my_psp22_facet', 'new', 1000)
    let constructor = (await abi).V3.spec.constructors[0].selector
    let psp22Hash = (await abi).source.hash
    let messages = (await abi).V3.spec.messages

    // initialize diamond
    const diamondContract = await setupContract('my_diamond', 'new', [[psp22Hash, [[messages[0].selector, ADD]]]], signerAddress, diamondHash)

    const proxy = setupProxy(psp22Facet, diamondContract.contract)

    let balance = await proxy.query.balanceOf(defaultSigner.address)
    console.log(balance.output?.toHuman())

    let balance2 = await psp22Facet.query.balanceOf(defaultSigner.address)
    console.log(balance2.output?.toHuman())
  })
})
