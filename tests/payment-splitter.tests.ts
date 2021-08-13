import {bnArg, expect, fromSigner, setupContract} from './helpers'
import {network, patract} from 'redspot'
const { api } = network;

const {getRandomSigner} = patract

describe('MY_PAYMENT_SPLITTER', () => {
    async function setup() {
        const ian = await getRandomSigner()
        const kayne = await getRandomSigner()
        let contract = await setupContract('my_payment_splitter', 'new', [kayne.address, ian.address], [40, 60]);

        return { contract, kayne, ian }
    }

    it('PAYMENT SPLITTER - init values', async () => {
        // Arrange - Create a contract
        const {
            contract,
            kayne,
            ian,
        } = await setup()

        // Assert - Init with shares
        await expect(contract.query.shares(kayne.address)).to.have.output(40)
        await expect(contract.query.shares(ian.address)).to.have.output(60)
        await expect(contract.query.totalShares()).to.have.output(100)
        await expect(contract.query.payee(0)).to.have.output(kayne.address)
        await expect(contract.query.payee(1)).to.have.output(ian.address)
    })

    it('PAYMENT SPLITTER - release a native token', async () => {
        // Arrange - Create a contract
        const {
            contract,
            kayne,
            ian,
        } = await setup()

        // Act - Send native token and release them
        await expect(contract.contract.query.totalReleased()).to.have.output(0);
        await expect(fromSigner(contract.contract, contract.alice.address).tx.receive({ value: 20000000 })).to.eventually.be.fulfilled
        await expect(contract.contract.tx.release(kayne.address)).to.eventually.be.fulfilled
        await expect(contract.contract.tx.release(ian.address)).to.eventually.be.fulfilled

        // Assert - Ian must got more tokens than kayne
        // @ts-ignore
        let totalReleased = Number.parseInt((await contract.contract.query.totalReleased()).output);
        // @ts-ignore
        let kayneReleased = Number.parseInt((await contract.contract.query.released(kayne.address)).output);
        // @ts-ignore
        let ianReleased = Number.parseInt((await contract.contract.query.released(ian.address)).output);
        expect(ianReleased > kayneReleased).to.true;
        expect(ianReleased + kayneReleased).to.equal(totalReleased);
    })
})
