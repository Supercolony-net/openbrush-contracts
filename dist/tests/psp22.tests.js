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
const constants_1 = require("./constants");
const helpers_1 = require("./helpers");
describe('MY_PSP22', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('my_psp22', 'new', '1000', 'TOKEN', 'TKN', 2);
        });
    }
    function setup_receiver() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('psp22_receiver', 'new');
        });
    }
    it('Assigns initial balance', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, defaultSigner: sender } = yield setup();
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(1000);
    }));
    it('Transfer adds amount to destination account', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, accounts: [receiver] } = yield setup();
        yield helpers_1.expect(() => contract.tx.transfer(receiver.address, 7, [])).to.changeTokenBalance(contract, receiver, 7);
        yield helpers_1.expect(() => contract.tx.transfer(receiver.address, 7, [])).to.changeTokenBalances(contract, [contract.signer, receiver], [-7, 7]);
    }));
    it('Transfers funds successfully if destination account is a receiver and supports transfers', () => __awaiter(void 0, void 0, void 0, function* () {
        const { tx } = yield setup();
        const { contract } = yield setup_receiver();
        yield helpers_1.expect(tx.transfer(contract.address, 7, [])).to.eventually.be.fulfilled;
    }));
    it('Can not transfer above the amount', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, accounts: [receiver] } = yield setup();
        yield helpers_1.expect(contract.tx.transfer(receiver.address, 1007, [])).to.eventually.be.rejected;
    }));
    it('Can not transfer to hated account', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, tx, accounts: [hated_account] } = yield setup();
        // Check that we can transfer money while account is not hated
        yield helpers_1.expect(tx.transfer(hated_account.address, 10, [])).to.eventually.be.fulfilled;
        let result = yield query.balanceOf(hated_account.address);
        helpers_1.expect(result.output).to.equal(10);
        yield helpers_1.expect(query.getHatedAccount()).to.have.output(constants_1.consts.EMPTY_ADDRESS);
        // Hate account
        yield helpers_1.expect(tx.setHatedAccount(hated_account.address)).to.eventually.be.ok;
        yield helpers_1.expect(query.getHatedAccount()).to.have.output(hated_account.address);
        // Transfer must fail
        yield helpers_1.expect(tx.transfer(hated_account.address, 10, [])).to.eventually.be.rejected;
        // Amount of tokens must be the same
        result = yield query.balanceOf(hated_account.address);
        helpers_1.expect(result.output).to.equal(10);
    }));
});
