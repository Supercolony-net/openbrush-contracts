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
describe('MY_PSP1155_MINTABLE', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('my_psp1155_mintable', 'new');
        });
    }
    it('Mint works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, defaultSigner: sender, query, accounts: [alice] } = yield setup();
        let token1 = helpers_1.bnArg(1);
        let token2 = helpers_1.bnArg(2);
        let amount1 = 1;
        let amount2 = 10;
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([0, 0]);
        yield helpers_1.expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([0, 0]);
        yield contract.tx.mint(token1, amount1);
        yield contract.tx.mint(token2, amount2);
        yield contract.tx.mintTo(alice.address, token1, amount1);
        yield contract.tx.mintTo(alice.address, token2, amount2);
        yield helpers_1.expect(query.balanceOfBatch([[sender.address, token1], [sender.address, token2]]))
            .to.have.output([amount1, amount2]);
        yield helpers_1.expect(query.balanceOfBatch([[alice.address, token1], [alice.address, token2]]))
            .to.have.output([amount1, amount2]);
    }));
});
