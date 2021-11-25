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
const helpers_1 = require("../../helpers");
describe('MY_PSP1155_BURNABLE', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('my_psp1155_burnable', 'new');
        });
    }
    it('Burn works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = yield setup();
        let tokenId = helpers_1.bnArg(0);
        let tokenId2 = helpers_1.bnArg(1);
        let mintAmount = 1;
        let mintAmount2 = 20;
        yield contract.tx.safeTransferFrom(sender.address, alice.address, tokenId, mintAmount, []);
        yield helpers_1.expect(query.balanceOf(alice.address, tokenId)).to.have.output(mintAmount);
        yield helpers_1.expect(query.balanceOf(sender.address, tokenId2)).to.have.output(mintAmount2);
        yield helpers_1.fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true);
        yield contract.tx.burn(tokenId2, mintAmount2);
        yield contract.tx.burnFrom(alice.address, tokenId, mintAmount);
        yield helpers_1.expect(query.balanceOf(sender.address, tokenId)).to.have.output(0);
        yield helpers_1.expect(query.balanceOf(alice.address, tokenId2)).to.have.output(0);
    }));
    it('Burn batch works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = yield setup();
        let token1 = helpers_1.bnArg(0);
        let token2 = helpers_1.bnArg(1);
        let amount1 = 1;
        let amount2 = 10;
        yield contract.tx.safeTransferFrom(sender.address, alice.address, token2, amount2, []);
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([amount1, amount2]);
        yield helpers_1.expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, amount2]);
        yield contract.tx.burnBatch([[token1, amount1], [token2, amount2]], []);
        yield helpers_1.fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true);
        yield contract.tx.burnBatchFrom(alice.address, [[token1, 0], [token2, amount2]], []);
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([0, 0]);
        yield helpers_1.expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0]);
    }));
    it('Burn from without allowance should fail', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, accounts: [alice], query, defaultSigner: sender } = yield setup();
        let token1 = helpers_1.bnArg(0);
        let token2 = helpers_1.bnArg(1);
        let amount1 = 1;
        let amount2 = 20;
        yield contract.tx.safeTransferFrom(sender.address, alice.address, token1, amount1, []);
        yield contract.tx.safeTransferFrom(sender.address, alice.address, token2, amount2, []);
        yield helpers_1.expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([amount1, amount2]);
        yield helpers_1.expect(contract.tx.burnBatchFrom(alice.address, [[token1, amount1], [token2, amount2]], []))
            .to.eventually.be.rejected;
        yield helpers_1.expect(contract.tx.burnFrom(alice.address, token1, amount1, []))
            .to.eventually.be.rejected;
        yield helpers_1.expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([amount1, amount2]);
    }));
    it('Burn inssuficient balance should fail', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, defaultSigner: sender, query, accounts: [alice] } = yield setup();
        let token1 = helpers_1.bnArg(0);
        let token2 = helpers_1.bnArg(1);
        let amount1 = 1;
        let amount2 = 20;
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([amount1, amount2]);
        yield helpers_1.expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0]);
        yield helpers_1.expect(contract.tx.burnBatch([[token1, amount1 + 1], [token2, amount2]], []))
            .to.eventually.be.rejected;
        yield helpers_1.expect(contract.tx.burn(token1, amount1 + 1, []))
            .to.eventually.be.rejected;
        yield helpers_1.fromSigner(contract, alice.address).tx.setApprovalForAll(sender.address, true);
        yield helpers_1.expect(contract.tx.burnBatchFrom(alice.address, [[token1, amount1 + 1], [token2, amount2]], []))
            .to.eventually.be.rejected;
        yield helpers_1.expect(contract.tx.burnFrom(alice.address, token1, amount1 + 1, []))
            .to.eventually.be.rejected;
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([amount1, amount2]);
        yield helpers_1.expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0]);
    }));
});
