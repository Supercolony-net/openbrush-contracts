import { consts, Roles } from './constants'
import {bnArg, expect, fromSigner, setupContract} from './helpers'

describe('MY_ACCESS_CONTROL', () => {
  async function setup() {
    return setupContract('my_access_control', 'new')
  }

  it('ERC 721 - mint works', async () => {
    const {
      contract,
      query,
      defaultSigner: sender
    } = await setup()

    await expect(contract.tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(0))

    const result = await query.balanceOf(sender.address)
    expect(result.output).to.equal(1)
  })

  it('ERC 721 - mint existing should fail', async () => {
    const {
      contract,
      query,
      defaultSigner: sender
    } = await setup()

    await expect(contract.tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(0))

    const balance = await query.balanceOf(sender.address)
    expect(balance.output).to.equal(1)

    const owner = await query.ownerOf(bnArg(0))
    expect(owner.output).to.equal(sender.address)

    await expect(contract.tx.mint(bnArg(0))).to.eventually.be.rejected
  })

  it('ERC 721 - approved transfer works', async () => {
    const {
      contract,
      query,
      defaultSigner: sender,
      accounts: [alice, bob]
    } = await setup()

    await expect(contract.tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(0))

    const owner = await query.ownerOf(bnArg(0))
    expect(owner.output).to.equal(sender.address)

    await expect(contract.tx.approve(alice.address, bnArg(0)))
      .to.emit(contract, 'Approval').withArgs(sender.address, alice.address, bnArg(0))

    await expect(fromSigner(contract, alice.address).tx.transferFrom(sender.address, bob.address, bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(sender.address, bob.address, bnArg(0))

    const ownerOf = await query.ownerOf(bnArg(0))
    expect(ownerOf.output).to.equal(bob.address)
  })

  it('ERC 721 - approved for all works', async () => {
    const {
      contract,
      query,
      defaultSigner: sender,
      accounts: [alice, bob]
    } = await setup()

    await expect(contract.tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(0))

    await expect(contract.tx.mint(bnArg(1)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(1))

    const balanceOf = await query.balanceOf(sender.address)
    expect(balanceOf.output).to.equal(2)

    await expect(contract.tx.setApprovalForAll(alice.address, true))
      .to.emit(contract, 'ApprovalForAll').withArgs(sender.address, alice.address, true)

    await expect(fromSigner(contract, alice.address).tx.transferFrom(sender.address, bob.address, bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(sender.address, bob.address, bnArg(0))

    const ownerOf0 = await query.ownerOf(bnArg(0))
    expect(ownerOf0.output).to.equal(bob.address)

    await expect(fromSigner(contract, alice.address).tx.transferFrom(sender.address, bob.address, bnArg(1)))
      .to.emit(contract, 'Transfer').withArgs(sender.address, bob.address, bnArg(1))

    const ownerOf1 = await query.ownerOf(bnArg(1))
    expect(ownerOf1.output).to.equal(bob.address)

    // Not approved transfer should fail
    await expect(fromSigner(contract, sender.address).tx.transferFrom(bob.address, alice.address, bnArg(0)))
      .to.eventually.be.rejected
  })

  it('ERC 721 - burn works', async () => {
    const {
      contract,
      query,
      defaultSigner: sender
    } = await setup()

    await expect(contract.tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(0))

    const resultMint = await query.balanceOf(sender.address)
    expect(resultMint.output).to.equal(1)

    await expect(contract.tx.burn(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(sender.address, consts.EMPTY_ADDRESS, bnArg(0))

    const resultBurn = await query.balanceOf(sender.address)
    expect(resultBurn.output).to.equal(0)
  })

  it('ACCESS CONTROL - only minter role is allowed to mint', async () => {
    const {
      contract,
      query,
      defaultSigner: sender,
      accounts: [alice]
    } = await setup()

    await expect(query.hasRole(Roles.DefaultAdminRole, sender.address)).to.have.output(true)
    await expect(query.hasRole(Roles.Minter, sender.address)).to.have.output(true)
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)

    await expect(contract.tx.mint(bnArg(0)))
      .to.emit(contract, 'Transfer').withArgs(consts.EMPTY_ADDRESS, sender.address, bnArg(0))

    await expect(fromSigner(contract, alice.address).tx.mint(bnArg(1))).to.eventually.be.rejected
  })

  it('ACCESS CONTROL - should not grant initial roles for random role', async () => {
    const {
      query,
      accounts: [alice]
    } = await setup()

    await expect(query.hasRole(Roles.DefaultAdminRole, alice.address)).to.have.output(false)
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
  })

  it('ACCESS CONTROL - should grant initial roles to default signer', async () => {
    const { query, defaultSigner: sender } = await setup()

    await expect(query.hasRole(Roles.DefaultAdminRole, sender.address)).to.have.output(true)
    await expect(query.hasRole(Roles.Minter, sender.address)).to.have.output(true)
  })

  it('ACCESS CONTROL - should not grant initial roles for random role', async () => {
    const {
      query,
      accounts: [alice]
    } = await setup()

    await expect(query.hasRole(Roles.DefaultAdminRole, alice.address)).to.have.output(false)
    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
  })

  it('ACCESS CONTROL - should grant role', async () => {
    const {
      query,
      tx,
      accounts: [alice]
    } = await setup()

    await tx.grantRole(Roles.Minter, alice.address)

    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)
  })

  it('ACCESS CONTROL - should not change old roles after grant role', async () => {
    const {
      query,
      tx,
      defaultSigner,
      accounts: [alice]
    } = await setup()

    await tx.grantRole(Roles.Minter, alice.address)

    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(true)
    await expect(query.hasRole(Roles.DefaultAdminRole, defaultSigner.address)).to.have.output(true)
    await expect(query.hasRole(Roles.Minter, defaultSigner.address)).to.have.output(true)
  })

  it('ACCESS CONTROL - should revoke role', async () => {
    const {
      query,
      tx,
      accounts: [alice]
    } = await setup()

    await tx.grantRole(Roles.Minter, alice.address)
    await tx.revokeRole(Roles.Minter, alice.address)

    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
  })

  it('ACCESS CONTROL - should renounce role', async () => {
    const {
      query,
      contract,
      accounts: [alice]
    } = await setup()

    await contract.tx.grantRole(Roles.Minter, alice.address)
    await fromSigner(contract, alice.address).tx.renounceRole(Roles.Minter, alice.address)

    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
  })

  it('ACCESS CONTROL - should reject when grant/revoke not by admin role', async () => {
    const {
      tx,
      contract,
      accounts: [alice, bob]
    } = await setup()

    await tx.grantRole(Roles.Minter, bob.address)

    await expect(fromSigner(contract, alice.address).tx.grantRole(Roles.Minter, alice.address)).to.eventually.be.rejected
    await expect(fromSigner(contract, alice.address).tx.revokeRole(Roles.Minter, bob.address)).to.eventually.be.rejected
  })

  it('ACCESS CONTROL - should reject when renounce not self role', async () => {
    const {
      tx,
      defaultSigner,
      contract,
      accounts: [alice]
    } = await setup()

    await tx.grantRole(Roles.Minter, alice.address)

    await expect(fromSigner(contract, defaultSigner.address).tx.renounceRole(Roles.Minter, alice.address)).to.eventually.be.rejected
  })

  it('ACCESS CONTROL - should reject burn if no minter role', async () => {
    const {
      query,
      contract,
      accounts: [alice]
    } = await setup()

    await expect(query.hasRole(Roles.Minter, alice.address)).to.have.output(false)
    await expect(fromSigner(contract, alice.address).tx.burn(alice.address, bnArg(0), 1)).to.eventually.be.rejected
  })
})
