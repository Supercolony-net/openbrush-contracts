import { consts } from '../constants'
import { expect, setupContract, fromSigner, setupProxy } from '../helpers'

describe('MY_UPGRADEABLE_PSP22', () => {
  it('MY_UPGRADEABLE_PSP22 - delegate code is my_psp22 code hash', async () => {
    const { abi}  = await setupContract('my_psp22_upgradeable', 'new', '1000')
    const hash = (await abi).source.hash
    const { query } = await setupContract('my_proxy', 'new', hash)

    // Assert - contract hash is my_psp22 contract hash
    await expect(query.getDelegateCode()).to.have.output(hash)
  })

  it('MY_UPGRADEABLE_PSP22 - only owner can change delegate code', async () => {
    const { abi } = await setupContract('my_psp22_upgradeable', 'new', '0')
    const { contract: proxy } = await setupContract('my_proxy', 'new', '')
    await expect(fromSigner(proxy, consts.EMPTY_ADDRESS).tx.changeDelegateCode((await abi).source.hash)).to.eventually.be.rejected
  })

  it('MY_UPGRADEABLE_PSP22 - Assigns initial balance', async () => {
    const { contract: psp22, abi } = await setupContract('my_psp22_upgradeable', 'new', '0')
    const { contract, defaultSigner: sender } = await setupContract('my_proxy', 'new', (await abi).source.hash)
    const proxy = setupProxy(psp22, contract)
    await expect(proxy.tx.initialize(1000)).to.eventually.be.fulfilled

    await expect(proxy.query.totalSupply()).to.have.output(1000)
    await expect(proxy.query.balanceOf(sender.address)).to.have.output(1000)
  })

  it('MY_UPGRADEABLE_PSP22 - wrong proxy setup leads to transaction fail', async () => {
    const { contract: psp22 } = await setupContract('my_psp22_upgradeable', 'new', '0')
    const { contract } = await setupContract('my_proxy', 'new', '')
    const proxy = setupProxy(psp22, contract)

    await expect(proxy.tx.initialize(1000)).to.eventually.be.rejected
  })

  it('MY_UPGRADEABLE_PSP22 - Transfer adds amount to destination account', async () => {
    const { contract: psp22, abi, accounts: [receiver] } = await setupContract('my_psp22_upgradeable', 'new', '0')
    const { contract } = await setupContract('my_proxy', 'new', (await abi).source.hash)
    const proxy = setupProxy(psp22, contract)

    await expect(proxy.tx.initialize(1000)).to.eventually.be.fulfilled

    await expect(() => proxy.tx.transfer(receiver.address, 7, [])).to.changeTokenBalance(proxy, receiver, 7)
    await expect(() => proxy.tx.transfer(receiver.address, 7, [])).to.changeTokenBalances(proxy, [proxy.signer, receiver], [-7, 7])
  })

  it('MY_UPGRADEABLE_PSP22 - Transfers funds successfully if destination account is a receiver and supports transfers', async () => {
    const { contract: psp22, abi} = await setupContract('my_psp22_upgradeable', 'new', '0')
    const { contract } = await setupContract('my_proxy', 'new', (await abi).source.hash)
    const proxy = setupProxy(psp22, contract)
    const { contract: psp22_receiver } = await setupContract('psp22_receiver', 'new')
    await expect(proxy.tx.initialize(1000)).to.eventually.be.fulfilled

    await expect(proxy.tx.transfer(psp22_receiver.address, 7, [])).to.eventually.be.fulfilled
  })

  it('MY_UPGRADEABLE_PSP22 - Transfers funds successfully if destination account is a receiver a contract but not PSP22Receiver', async () => {
    const { contract: psp22_0, abi } = await setupContract('my_psp22_upgradeable', 'new', '0')
    const { contract: psp22_1 } = await setupContract('my_psp22_upgradeable', 'new', '0')
    const { contract } = await setupContract('my_proxy', 'new', (await abi).source.hash)
    const proxy = setupProxy(psp22_0, contract)
    await expect(proxy.tx.initialize(1000)).to.eventually.be.fulfilled

    await expect(proxy.tx.transfer(psp22_1.address, 7, [])).to.eventually.be.fulfilled
  })

  it('MY_UPGRADEABLE_PSP22 - Can not transfer above the amount', async () => {
    const { contract: psp22, abi, accounts: [receiver]} = await setupContract('my_psp22_upgradeable', 'new', '0')
    const { contract } = await setupContract('my_proxy', 'new', (await abi).source.hash)
    const proxy = setupProxy(psp22, contract)
    await expect(proxy.tx.initialize(1000)).to.eventually.be.fulfilled

    await expect(proxy.tx.transfer(receiver.address, 1007, [])).to.eventually.be.rejected
  })

  it('MY_UPGRADEABLE_PSP22 - update psp22 to psp22_metadata', async () => {
    const {contract: psp22, abi: abi_psp22, accounts: [receiver]} = await setupContract('my_psp22_upgradeable', 'new', '0')
    const { contract } = await setupContract('my_proxy', 'new', (await abi_psp22).source.hash)
    const proxy = setupProxy(psp22, contract)
    await expect(proxy.tx.initialize(1000)).to.eventually.be.fulfilled

    await expect(proxy.query.totalSupply()).to.have.output(1000)
    await expect(proxy.tx.transfer(receiver.address, 100, [])).to.eventually.be.fulfilled

    const {contract: psp22_metadata, abi: abi_psp22_metadata} = await setupContract('my_psp22_metadata_upgradeable', 'new', '0', '', '', '0')
    const hash = (await abi_psp22_metadata).source.hash

    await expect(contract.tx.changeDelegateCode(hash)).to.eventually.be.fulfilled
    await expect(contract.query.getDelegateCode()).to.have.output(hash)

    const proxy_metadata = setupProxy(psp22_metadata, contract)
    await expect(proxy_metadata.tx.initialize(0,'COLONY', 'COL', 18)).to.eventually.be.fulfilled

    await expect(proxy_metadata.query.totalSupply()).to.have.output(1000)
    await expect(proxy_metadata.query.tokenName()).to.have.output('COLONY')
    await expect(proxy_metadata.query.tokenSymbol()).to.have.output('COL')
    await expect(proxy_metadata.query.tokenDecimals()).to.have.output(18)
    await expect(proxy_metadata.query.balanceOf(receiver.address)).to.have.output(100)
  })
})
