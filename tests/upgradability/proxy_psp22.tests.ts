import { consts } from '../constants'
import { expect, setupContract, fromSigner, getSigner, patchProxyContractMethods } from '../helpers'

describe('MY_UPGRADEABLE_PSP22', () => {

  it('MY_UPGRADEABLE_PSP22 - delegate code is my_psp22 code hash', async () => {
    const {abi} = await setupContract('my_psp22', 'new', '1000')
    const hash = (await abi).source.hash
    const { query } = await setupContract('my_proxy', 'new', hash)

    // Assert - contract hash is my_psp22 contract hash
    await expect(query.getDelegateCode()).to.have.output(hash)
  })

  it('MY_UPGRADEABLE_PSP22 - Assigns initial balance', async () => {
    const { contract: psp22, abi, defaultSigner: sender } = await setupContract('my_psp22', 'new', '1000')
    const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi).source.hash)
    patchProxyContractMethods(psp22, proxy)

    await expect(proxy.query.totalSupply()).to.have.output(1000)
    await expect(proxy.query.balanceOf(sender.address)).to.have.output(1000)
  })

  it('MY_UPGRADEABLE_PSP22 - Transfer adds amount to destination account', async () => {
    const { contract: psp22, abi, accounts: [receiver] } = await setupContract('my_psp22', 'new', '1000')
    const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi).source.hash)
    patchProxyContractMethods(psp22, proxy)

    await expect(() => proxy.tx.transfer(receiver.address, 7, [])).to.changeTokenBalance(proxy, receiver, 7)
    await expect(() => proxy.tx.transfer(receiver.address, 7, [])).to.changeTokenBalances(proxy, [psp22.signer, receiver], [-7, 7])
  })

  it('MY_UPGRADEABLE_PSP22 - Transfers funds successfully if destination account is a receiver and supports transfers', async () => {
    const { contract: psp22, abi, accounts: [receiver] } = await setupContract('my_psp22', 'new', '1000')
    const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi).source.hash)
    patchProxyContractMethods(psp22, proxy)
    const { contract } = await setupContract('psp22_receiver', 'new')

    await expect(proxy.tx.transfer(contract.address, 7, [])).to.eventually.be.fulfilled
  })

  it('MY_UPGRADEABLE_PSP22 - Transfers funds successfully if destination account is a receiver a contract but not PSP22Receiver', async () => {
    const { contract: psp22, abi, accounts: [receiver] } = await setupContract('my_psp22', 'new', '1000')
    const { contract } = await setupContract('my_psp22', 'new', '1000')
    const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi).source.hash)
    patchProxyContractMethods(psp22, proxy)

    await expect(proxy.tx.transfer(contract.address, 7, [])).to.eventually.be.fulfilled
  })

  it('MY_UPGRADEABLE_PSP22 - Can not transfer above the amount', async () => {
    const { contract: psp22, abi, accounts: [receiver] } = await setupContract('my_psp22', 'new', '1000')
    const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi).source.hash)
    patchProxyContractMethods(psp22, proxy)

    await expect(proxy.tx.transfer(receiver.address, 1007, [])).to.eventually.be.rejected
  })

  it('MY_UPGRADEABLE_PSP22 - Can not transfer to hated account', async () => {
    const { contract: psp22, abi, accounts: [hated_account] } = await setupContract('my_psp22', 'new', '1000')
    const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi).source.hash)
    patchProxyContractMethods(psp22, proxy)

    // Check that we can transfer money while account is not hated
    await expect(proxy.tx.transfer(hated_account.address, 10, [])).to.eventually.be.fulfilled
    let result = await proxy.query.balanceOf(hated_account.address)
    expect(result.output).to.equal(10)

    await expect(proxy.query.getHatedAccount()).to.have.output(consts.EMPTY_ADDRESS)

    // Hate account
    await expect(proxy.tx.setHatedAccount(hated_account.address)).to.eventually.be.ok
    await expect(proxy.query.getHatedAccount()).to.have.output(hated_account.address)

    // Transfer must fail
    await expect(proxy.tx.transfer(hated_account.address, 10, [])).to.eventually.be.rejected

    // Amount of tokens must be the same
    result = await proxy.query.balanceOf(hated_account.address)
    expect(result.output).to.equal(10)
  })

  it('MY_UPGRADEABLE_PSP22 - update my_psp22 to my_psp22_burnable', async () => {
    const {abi: abi_psp22} = await setupContract('my_psp22', 'new', '1000')
    const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi_psp22).source.hash)

    const {abi: abi_psp22_burnable} = await setupContract('my_psp22_burnable', 'new', '1000')
    const hash = (await abi_psp22_burnable).source.hash

    await expect(proxy.tx.changeDelegateCode(hash)).to.eventually.be.fulfilled
    // Assert - contract hash is my_psp22_burnable contract hash
    await expect(proxy.query.getDelegateCode()).to.have.output(hash)
  })

  it('MY_UPGRADEABLE_PSP22_BURNABLE - Assigns initial balance', async () => {
    const {contract: psp22, abi: abi_psp22, accounts: [receiver]} = await setupContract('my_psp22', 'new', '1000')
    const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi_psp22).source.hash)

    await expect(() => proxy.tx.transfer(receiver.address, 7, [])).to.changeTokenBalance(proxy, receiver, 7)
    await expect(() => proxy.tx.transfer(receiver.address, 7, [])).to.changeTokenBalances(proxy, [psp22.signer, receiver], [-7, 7])

    const {contract, abi: abi_psp22_burnable, defaultSigner: sender} = await setupContract('my_psp22_burnable', 'new', '1000')
    const hash = await (await abi_psp22_burnable).source.hash
    await expect(proxy.tx.changeDelegateCode(hash)).to.eventually.be.fulfilled
    patchProxyContractMethods(contract, proxy)

    await expect(proxy.query.balanceOf(sender.address)).to.have.output(1000)
})

it('MY_UPGRADEABLE_PSP22_BURNABLE - Can burn', async () => {
    const {abi: abi_psp22} = await setupContract('my_psp22', 'new', '1000')
    const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi_psp22).source.hash)

    const {contract, abi: abi_psp22_burnable, defaultSigner: sender} = await setupContract('my_psp22_burnable', 'new', '1000')
    const hash = (await abi_psp22_burnable).source.hash
    await expect(proxy.tx.changeDelegateCode(hash)).to.eventually.be.fulfilled
    patchProxyContractMethods(contract, proxy)

    // Assert - Ensure sender initial balance is 1000
    await expect(proxy.query.balanceOf(sender.address)).to.have.output(1000);

    // Act - Burn sender's tokens
    await proxy.tx.burn(sender.address, 10)

    // Assert - Ensure sender balance is now 990
    await expect(proxy.query.balanceOf(sender.address)).to.have.output(990);
})

it('MY_UPGRADEABLE_PSP22_BURNABLE - Can burn without allowance', async () => {
  const {abi: abi_psp22, alice} = await setupContract('my_psp22', 'new', '1000')
  const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi_psp22).source.hash)

  const {contract, abi: abi_psp22_burnable, defaultSigner: sender} = await setupContract('my_psp22_burnable', 'new', '1000')
  const hash = (await abi_psp22_burnable).source.hash
  await expect(proxy.tx.changeDelegateCode(hash)).to.eventually.be.fulfilled
  patchProxyContractMethods(contract, proxy)

  // Assert - Ensure sender initial balance is 1000 and allowance is 0
  await expect(proxy.query.balanceOf(sender.address)).to.have.output(1000);
  await expect(proxy.query.allowance(sender.address, alice.address)).to.have.output(0);

  // Act - Burn sender's tokens
  const alice_proxy = proxy.connect(alice.address)
  patchProxyContractMethods(contract, alice_proxy)

  await alice_proxy.tx.burn(sender.address, 10)

  // Assert - Ensure sender balance is now 990
  await expect(proxy.query.balanceOf(sender.address)).to.have.output(990);
})

it('MY_UPGRADEABLE_PSP22_BURNABLE - Decreases total supply after burning', async () => {
  const {abi: abi_psp22} = await setupContract('my_psp22', 'new', '1000')
  const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi_psp22).source.hash)

  const {contract, abi: abi_psp22_burnable, defaultSigner: sender} = await setupContract('my_psp22_burnable', 'new', '1000')
  const hash = (await abi_psp22_burnable).source.hash
  await expect(proxy.tx.changeDelegateCode(hash)).to.eventually.be.fulfilled
  patchProxyContractMethods(contract, proxy)

  // Arrange - Ensure initial supply is correct
  await expect(proxy.query.totalSupply()).to.have.output(1000)

  // Act - Burn token from owner
  await proxy.tx.burn(sender.address, 1)

  // Assert - Ensure sender balance is now 999
  await expect(proxy.query.totalSupply()).to.have.output(999)
})

it('MY_UPGRADEABLE_PSP22_BURNABLE - Can burn from', async () => {
  const {abi: abi_psp22, defaultSigner: sender, alice} = await setupContract('my_psp22', 'new', '1000')
  const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi_psp22).source.hash)

  const {contract, abi: abi_psp22_burnable} = await setupContract('my_psp22_burnable', 'new', '1000')
  const hash = (await abi_psp22_burnable).source.hash
  await expect(proxy.tx.changeDelegateCode(hash)).to.eventually.be.fulfilled
  patchProxyContractMethods(contract, proxy)

    // Arrange - Transfer tokens to Alice
    await proxy.tx.transfer(alice.address, 10, []);

    // Act - burn from Alice address
    await proxy.tx.burn(alice.address, 10)

    // Assert - ensure needed amount was burnt
    await expect(proxy.query.balanceOf(alice.address)).to.have.output(0);
})

it('MY_UPGRADEABLE_PSP22_BURNABLE - Can burn from many', async () => {
  const {abi: abi_psp22, defaultSigner: sender} = await setupContract('my_psp22', 'new', '1000')
  const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi_psp22).source.hash)

  const {contract, abi: abi_psp22_burnable} = await setupContract('my_psp22_burnable', 'new', '1000')
  const hash = (await abi_psp22_burnable).source.hash
  await expect(proxy.tx.changeDelegateCode(hash)).to.eventually.be.fulfilled
  patchProxyContractMethods(contract, proxy)

    // Arrange - Create a signers, transfer tokens to them
    const alice = await getSigner('Alice')
    const bob = await getSigner('Bob')
    await proxy.tx.transfer(alice.address, 10, []);
    await proxy.tx.transfer(bob.address, 10, []);

    // Act - burn tokens from Alice and Bob
    await proxy.tx.burnFromMany([[alice.address, 10], [bob.address, 10]])

    // Assert - ensure needed amount was burnt
    await expect(proxy.query.balanceOf(alice.address)).to.have.output(0);
    await expect(proxy.query.balanceOf(bob.address)).to.have.output(0);
})

  it(`MY_UPGRADEABLE_PSP22_BURNABLE - Fails if one of the account's balance exceeds amount to burn`, async () => {
    const {abi: abi_psp22, defaultSigner: sender} = await setupContract('my_psp22', 'new', '1000')
    const { contract: proxy } = await setupContract('my_proxy', 'new', (await abi_psp22).source.hash)

    const {contract, abi: abi_psp22_burnable} = await setupContract('my_psp22_burnable', 'new', '1000')
    const hash = await (await abi_psp22_burnable).source.hash
    await expect(proxy.tx.changeDelegateCode(hash)).to.eventually.be.fulfilled
    patchProxyContractMethods(contract, proxy)

      // Arrange - Create a signers, transfer tokens to them
      const alice = await getSigner('Alice')
      const bob = await getSigner('Bob')
      await proxy.tx.transfer(alice.address, 10, []);
      await proxy.tx.transfer(bob.address, 5, []);

      // Act - burn tokens from Alice and Bob but burnt from Bob more than he own
      await expect(proxy.tx.burnFromMany([[alice.address, 10], [bob.address, 10]])).to.eventually.be.rejected

      // Assert - ensure tokens was not burnt from the accounts
      await expect(proxy.query.balanceOf(alice.address)).to.have.output(10);
      await expect(proxy.query.balanceOf(bob.address)).to.have.output(5);
  })

  it('MY_UPGRADEABLE_PSP22 - contract logic update from psp22 to psp22_burnable', async () => {
    const {contract: psp22, abi: abi_psp22, accounts: [receiver]} = await setupContract('my_psp22', 'new', '1000')
    const {contract: proxy} = await setupContract('my_proxy', 'new', (await abi_psp22).source.hash)
    patchProxyContractMethods(psp22, proxy)

    await proxy.tx.transfer(receiver.address, 7, [])
    await expect(proxy.query.balanceOf(receiver.address)).to.have.output(7)

    const {contract: psp2_burnable, abi: abi_psp22_burnable, defaultSigner: sender} = await setupContract('my_psp22_burnable', 'new', '1000')
    const hash = (await abi_psp22_burnable).source.hash
    await expect(proxy.tx.changeDelegateCode(hash)).to.eventually.be.fulfilled
    patchProxyContractMethods(psp2_burnable, proxy)
    
    await expect(proxy.query.balanceOf(receiver.address)).to.have.output(7)

    // Arrange - Ensure initial supply is correct
    await expect(proxy.query.totalSupply()).to.have.output(1000)

    // Act - Burn token from owner
    await proxy.tx.burn(receiver.address, 2)

    // Assert - Ensure sender balance is now 999
    await expect(proxy.query.totalSupply()).to.have.output(998)
    await expect(proxy.query.balanceOf(receiver.address)).to.have.output(5)
  })
})
