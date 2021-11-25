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
const helpers_1 = require("../helpers");
describe('MY_PSP721', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('my_psp721', 'new');
        });
    }
    function setup_receiver() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('psp721_receiver', 'new');
        });
    }
    it('Assigns initial balance', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, defaultSigner: sender } = yield setup();
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(3);
    }));
    it('Transfer works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, defaultSigner: sender, accounts: [alice], query } = yield setup();
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(3);
        yield helpers_1.expect(query.balanceOf(alice.address)).to.have.output(0);
        yield contract.tx.transferFrom(sender.address, alice.address, helpers_1.bnArg(0));
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(2);
        yield helpers_1.expect(query.balanceOf(alice.address)).to.have.output(1);
    }));
    it('Transfer from works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, defaultSigner: sender, accounts: [alice], query } = yield setup();
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(3);
        yield helpers_1.expect(query.balanceOf(alice.address)).to.have.output(0);
        yield contract.tx.setApprovalForAll(alice.address, true);
        yield helpers_1.fromSigner(contract, alice.address).tx.transferFrom(sender.address, alice.address, helpers_1.bnArg(0));
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(2);
        yield helpers_1.expect(query.balanceOf(alice.address)).to.have.output(1);
    }));
    it('PSP 721 - safe transfer works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { tx, query, defaultSigner: sender, } = yield setup();
        const { contract } = yield setup_receiver();
        // Arrange - Sender mint a Token and Approve Receiver as spender of this token
        yield helpers_1.expect(query.ownerOf(helpers_1.bnArg(0))).to.have.output(sender.address);
        // Act - Alice transfers the token form sender to bob
        yield helpers_1.expect(contract.query.getCallCounter()).to.have.output(0);
        yield helpers_1.expect(tx.safeTransferFrom(sender.address, contract.address, helpers_1.bnArg(0), 'data')).to.eventually.be.fulfilled;
        yield helpers_1.expect(contract.query.getCallCounter()).to.have.output(1);
        // Assert - Bob is now owner of the token
        yield helpers_1.expect(query.ownerOf(helpers_1.bnArg(0))).to.have.output(contract.address.toString());
    }));
    it('PSP 721 - receiver can reject the transfer', () => __awaiter(void 0, void 0, void 0, function* () {
        const { tx, query, defaultSigner: sender } = yield setup();
        const { contract } = yield setup_receiver();
        // Arrange - Sender mint a token
        yield helpers_1.expect(query.ownerOf(helpers_1.bnArg(0))).to.have.output(sender.address);
        // Act - Receiver wants to reject the next transfer
        yield helpers_1.expect(contract.tx.revertNextTransfer()).to.eventually.be.fulfilled;
        // Assert - Sender cannot send token to receiver & Sender still own the token
        yield helpers_1.expect(tx.safeTransferFrom(sender.address, contract.address, helpers_1.bnArg(0), 'data')).to.eventually.be.rejected;
        yield helpers_1.expect(query.ownerOf(helpers_1.bnArg(0))).to.have.output(sender.address);
    }));
    it('Can not transfer non-existing token', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, accounts: [receiver], defaultSigner: sender, query } = yield setup();
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(3);
        yield helpers_1.expect(contract.tx.transferFrom(sender.address, receiver.address, helpers_1.bnArg(3))).to.eventually.be.rejected;
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(3);
    }));
    it('Can not transfer without allowance', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, accounts: [alice], defaultSigner: sender, query, } = yield setup();
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(3);
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.transferFrom(sender.address, alice.address, helpers_1.bnArg(0)))
            .to.eventually.be.rejected;
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(3);
    }));
});
