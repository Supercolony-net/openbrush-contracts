import { consts } from '../constants'
import { expect, setupContractPalletAssets, setupContract } from '../helpers'

describe('MY_PSP22_ASSET', () => {
  function getRandomInt(max) {
    return Math.floor(Math.random() * max);
}

async function setup() {
    const random_asset_id = getRandomInt(10000).toString();
    return setupContractPalletAssets('my_psp22_pallet_asset', 'new', 
    'caller', 
    random_asset_id,
    '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d',
    '1',
    'name',
    'symbol',
    '12'
    )
  }

  it('Assigns initial balance', async () => {
    const { query, defaultSigner: sender } = await setup()

    await expect(query.balanceOf(sender.address)).to.have.output(0)
  })

  it(`Transfer should succeed`, async () => {
    const { query, tx, contract, accounts: [alice, bob, eve] } = await setup();

    await expect(contract.tx.mint(alice.address, 1000)).to.eventually.be.fulfilled
    // Arrange - Create a signers, transfer tokens to them
    await tx.transfer(eve.address, 10, []);
    await tx.transfer(bob.address, 5, []);

    // Assert - ensure tokens was not burnt from the accounts
    await expect(query.balanceOf(eve.address)).to.have.output(10);
    await expect(query.balanceOf(bob.address)).to.have.output(5);
  })

  it(`Transfer approve succeed`, async () => {
    const { query, tx, contract, accounts: [alice, bob, eve] } = await setup();

    await expect(contract.tx.mint(alice.address, 1000)).to.eventually.be.fulfilled
    // Arrange - Create a signers, transfer tokens to them
    await tx.approve(bob.address, 500, []);
    
    // Assert - ensure tokens was not burnt from the accounts
    await expect(query.allowance(alice.address, bob.address)).to.have.output(500);
  })

})
