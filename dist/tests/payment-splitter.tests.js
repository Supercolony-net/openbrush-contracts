"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const helpers_1 = require("./helpers");
const redspot_1 = require("redspot");
const { api } = redspot_1.network;
const { getRandomSigner } = redspot_1.patract;
describe('MY_PAYMENT_SPLITTER', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            const ian = yield getRandomSigner();
            const kayne = yield getRandomSigner();
            let contract = yield helpers_1.setupContract('my_payment_splitter', 'new', [kayne.address, ian.address], [40, 60]);
            return { contract, kayne, ian };
        });
    }
    it('PAYMENT SPLITTER - init values', () => __awaiter(void 0, void 0, void 0, function* () {
        // Arrange - Create a contract
        const { contract, kayne, ian } = yield setup();
        // Assert - Init with shares
        yield helpers_1.expect(contract.query.shares(kayne.address)).to.have.output(40);
        yield helpers_1.expect(contract.query.shares(ian.address)).to.have.output(60);
        yield helpers_1.expect(contract.query.totalShares()).to.have.output(100);
        yield helpers_1.expect(contract.query.payee(0)).to.have.output(kayne.address);
        yield helpers_1.expect(contract.query.payee(1)).to.have.output(ian.address);
    }));
    it('PAYMENT SPLITTER - release a native token', () => __awaiter(void 0, void 0, void 0, function* () {
        // Arrange - Create a contract
        const { contract, kayne, ian } = yield setup();
        // Act - Send native token and release them
        yield helpers_1.expect(contract.contract.query.totalReleased()).to.have.output(0);
        yield helpers_1.expect(helpers_1.fromSigner(contract.contract, contract.alice.address).tx.receive({ value: 20000000 })).to.eventually.be.fulfilled;
        yield helpers_1.expect(contract.contract.tx.release(kayne.address)).to.eventually.be.fulfilled;
        yield helpers_1.expect(contract.contract.tx.release(ian.address)).to.eventually.be.fulfilled;
        // Assert - Ian must hold more tokens than kayne
        // @ts-ignore
        let totalReleased = Number.parseInt((yield contract.contract.query.totalReleased()).output);
        // @ts-ignore
        let kayneReleased = Number.parseInt((yield contract.contract.query.released(kayne.address)).output);
        // @ts-ignore
        let ianReleased = Number.parseInt((yield contract.contract.query.released(ian.address)).output);
        helpers_1.expect(ianReleased > kayneReleased).to.true;
        helpers_1.expect(ianReleased + kayneReleased).to.equal(totalReleased);
    }));
});
