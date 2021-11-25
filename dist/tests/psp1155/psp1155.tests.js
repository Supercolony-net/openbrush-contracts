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
describe('MY_PSP1155', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('my_psp1155', 'new');
        });
    }
    function setup_receiver() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('psp1155_receiver', 'new');
        });
    }
    it('PSP 1155 - receiver can reject the transfer', () => __awaiter(void 0, void 0, void 0, function* () {
        const { tx, query, defaultSigner: sender } = yield setup();
        const { contract } = yield setup_receiver();
        // Arrange
        yield helpers_1.expect(query.balanceOfBatch([[contract.address, helpers_1.bnArg(0)], [sender.address, helpers_1.bnArg(0)]])).to.have.output([0, 1]);
        // Act - Receiver wants to reject the next transfer
        yield helpers_1.expect(contract.tx.revertNextTransfer()).to.eventually.be.fulfilled;
        // Assert - Sender cannot send token to receiver
        yield helpers_1.expect(tx.safeTransferFrom(sender.address, contract.address, helpers_1.bnArg(0), 1, 'data')).to.eventually.be.rejected;
        yield helpers_1.expect(query.balanceOfBatch([[contract.address, helpers_1.bnArg(0)], [sender.address, helpers_1.bnArg(0)]])).to.have.output([0, 1]);
    }));
    it('Balance of works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, defaultSigner: sender } = yield setup();
        yield helpers_1.expect(query.balanceOf(sender.address, helpers_1.bnArg(0))).to.have.output(1);
    }));
    it('Balance of batch works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, defaultSigner: sender } = yield setup();
        let token1 = helpers_1.bnArg(0);
        let token2 = helpers_1.bnArg(1);
        let token1Amount = 1;
        let token2Amount = 20;
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([token1Amount, token2Amount]);
    }));
    it('Set approval works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = yield setup();
        yield helpers_1.expect(query.isApprovedForAll(sender.address, alice.address))
            .to.have.output(false);
        yield contract.tx.setApprovalForAll(alice.address, true);
        yield helpers_1.expect(query.isApprovedForAll(sender.address, alice.address))
            .to.have.output(true);
        yield contract.tx.setApprovalForAll(alice.address, false);
        yield helpers_1.expect(query.isApprovedForAll(sender.address, alice.address))
            .to.have.output(false);
    }));
    it('Transfer from single works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = yield setup();
        let tokenId = helpers_1.bnArg(0);
        let tokenId2 = helpers_1.bnArg(1);
        let transferAmount = 1;
        let token2Amount = 10;
        yield helpers_1.fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true);
        yield contract.tx.safeTransferFrom(sender.address, alice.address, tokenId2, token2Amount, []);
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, tokenId], [sender.address, tokenId2], [alice.address, tokenId], [alice.address, tokenId2]]))
            .to.have.output([transferAmount, token2Amount, 0, token2Amount]);
        yield contract.tx.safeTransferFrom(sender.address, alice.address, tokenId, transferAmount, []);
        yield contract.tx.safeTransferFrom(alice.address, sender.address, tokenId2, transferAmount, []);
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, tokenId], [sender.address, tokenId2], [alice.address, tokenId], [alice.address, tokenId2]]))
            .to.have.output([0, token2Amount + 1, transferAmount, token2Amount - 1]);
    }));
    it('Transfer from batch works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = yield setup();
        let token1 = helpers_1.bnArg(0);
        let token2 = helpers_1.bnArg(1);
        let amount1 = 1;
        let amount2 = 20;
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2], [alice.address, token1], [alice.address, token2]]))
            .to.have.output([amount1, amount2, 0, 0]);
        yield helpers_1.fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true);
        yield helpers_1.expect(contract.tx.safeBatchTransferFrom(sender.address, alice.address, [[token1, amount1], [token2, amount2]], []))
            .to.eventually.be.fulfilled;
        yield helpers_1.expect(contract.tx.safeBatchTransferFrom(alice.address, sender.address, [[token1, amount1], [token2, amount2]], []))
            .to.eventually.be.fulfilled;
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2], [alice.address, token1], [alice.address, token2]]))
            .to.have.output([amount1, amount2, 0, 0]);
    }));
    it('Transfer from single insufficient balance should fail', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, defaultSigner: sender, query, accounts: [alice] } = yield setup();
        let tokenId = helpers_1.bnArg(0);
        let tokenAmount = 1;
        yield helpers_1.expect(query.balanceOf(sender.address, tokenId)).to.have.output(tokenAmount);
        yield helpers_1.fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true);
        yield helpers_1.expect(contract.tx.safeTransferFrom(sender.address, alice.address, tokenId, tokenAmount + 1, []))
            .to.eventually.be.rejected;
        yield helpers_1.expect(query.balanceOf(sender.address, tokenId)).to.have.output(tokenAmount);
    }));
    it('Transfer from single without allowance should fail', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, defaultSigner: sender, accounts: [alice], query } = yield setup();
        let tokenId = helpers_1.bnArg(0);
        let tokenAmount = 1;
        yield helpers_1.expect(query.balanceOf(sender.address, tokenId)).to.have.output(tokenAmount);
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.safeTransferFrom(sender.address, alice.address, tokenId, tokenAmount, []))
            .to.eventually.be.rejected;
        yield helpers_1.expect(query.balanceOf(sender.address, tokenId)).to.have.output(tokenAmount);
    }));
    it('Transfer from batch insufficient balance should fail', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, defaultSigner: sender, accounts: [alice], query } = yield setup();
        let token1 = helpers_1.bnArg(0);
        let token2 = helpers_1.bnArg(1);
        let amount1 = 1;
        let amount2 = 20;
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([amount1, amount2]);
        yield helpers_1.expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0]);
        yield contract.tx.setApprovalForAll(alice.address, true);
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address)
            .tx.safeBatchTransferFrom(sender.address, alice.address, [[token1, amount1 + 1], [token2, amount2]], [])).to.eventually.be.rejected;
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([amount1, amount2]);
        yield helpers_1.expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0]);
    }));
});
