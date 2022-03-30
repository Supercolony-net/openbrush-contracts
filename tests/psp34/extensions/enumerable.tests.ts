import { bnArg, expect, fromSigner, setupContract } from '../../helpers'

interface Result {
  ok: Ok;
}
interface Ok{
  u8 : number;
}

describe('MY_PSP34_ENUMERABLE', () => {
  async function setup() {
    return setupContract('my_psp34_enumerable', 'new')
  }

  function result(s : string | undefined) {
    const result: Result = s != null ? JSON.parse(s) : null;
    return result;
  }

  it('Enumerable should fail', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query
    } = await setup()

    await expect(contract.tx.ownersTokenByIndex(sender.address,0)).to.eventually.be.rejected
    await expect(contract.tx.ownersTokenByIndex(alice.address,0)).to.eventually.be.rejected
  })

  it('Enumerable works', async () => {
    const {
      contract,
      defaultSigner: sender,
      accounts: [alice],
      query
    } = await setup()

    await expect(contract.tx.ownersTokenByIndex(sender.address,0)).to.eventually.be.rejected
    await expect(contract.tx.ownersTokenByIndex(alice.address,0)).to.eventually.be.rejected

    let psp34_id1 = {
      "u8" : 1,
    };
    let psp34_id2 = {
      "u8" : 2,
    };
    
    await contract.tx.mint(alice.address, psp34_id1)
    await contract.tx.mint(alice.address, psp34_id2)

    expect(result((await query.tokenByIndex(0)).output?.toString()).ok.u8).equal(1);
    expect(result((await query.tokenByIndex(1)).output?.toString()).ok.u8).equal(2);

    expect(result((await query.ownersTokenByIndex(alice.address,0)).output?.toString()).ok.u8).equal(1);
    expect(result((await query.ownersTokenByIndex(alice.address,1)).output?.toString()).ok.u8).equal(2);
  })

//   it('Mint existing should fail', async () => {
//     const {
//       contract,
//       accounts: [alice],
//       defaultSigner: sender,
//       query
//     } = await setup()

//     await expect(contract.tx.mint(sender.address, 0)).to.eventually.be.fulfilled
//     await expect(query.balanceOf(sender.address)).to.have.output(1)
//     await expect(query.balanceOf(alice.address)).to.have.output(0)

//     await expect(contract.tx.mint(sender.address, 0)).to.eventually.be.rejected
//     await expect(contract.tx.mint(alice.address, 0)).to.eventually.be.rejected

//     await expect(query.balanceOf(sender.address)).to.have.output(1)
//     await expect(query.balanceOf(alice.address)).to.have.output(0)
//   })

})
