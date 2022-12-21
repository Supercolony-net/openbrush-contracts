/* eslint-disable */
import {bnArg, expect, getSigners} from '../../helpers'
import {ApiPromise} from "@polkadot/api";
import Constructors from "../../../../typechain-generated/constructors/my_psp22_mintable";
import Contract from "../../../../typechain-generated/contracts/my_psp22_mintable";

describe('MY_PSP22_MINTABLE', () => {
    async function setup() {
        // return setupContract('my_psp22_mintable', 'new', '1000')
        const api = await ApiPromise.create()

        const signers = getSigners()
        const defaultSigner = signers[2]
        const alice = signers[0]
        const bob = signers[1]

        const contractFactory = new Constructors(api, defaultSigner)
        const contractAddress = (await contractFactory.new(1000)).address
        const contract = new Contract(contractAddress, defaultSigner, api)

        return {
            api,
            defaultSigner,
            alice,
            bob,
            contract,
            query: contract.query,
            tx: contract.tx
        }
    }

    it('Assigns initial balance', async () => {
        const { api, query, defaultSigner: sender } = await setup()

        await expect(query.balanceOf(sender.address)).to.have.bnToNumber(1000)

        await api.disconnect()
    })

    it('Minting requested amount', async () => {
        const { api, contract, query, alice } = await setup()

        // Arrange - Ensure receiver balance is 0
        await expect(query.balanceOf(alice.address)).to.have.bnToNumber(0)

        // Act - Sender mint a token
        await contract.tx.mint(alice.address, 1)

        // Assert - Sender balance is now 1
        await expect(query.balanceOf(alice.address)).to.have.bnToNumber(1)

        await api.disconnect()
    })

    it('Increases total supply after minting', async () => {
        const { api, contract, query, defaultSigner: sender } = await setup()

        // Arrange - Ensure initial supply is correct
        await expect(query.totalSupply()).to.have.bnToNumber(1000)

        // Act - Sender mint a token
        await contract.tx.mint(sender.address, 1)

        // Assert - Sender balance is now 1
        await expect(query.totalSupply()).to.have.bnToNumber(1001)

        await api.disconnect()
    })
})
