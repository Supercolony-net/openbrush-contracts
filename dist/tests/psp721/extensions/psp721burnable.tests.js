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
describe('MY_PSP721_BURNABLE', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('my_psp721_burnable', 'new');
        });
    }
    it('Burn works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, defaultSigner: sender, query } = yield setup();
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(3);
        yield contract.tx.burn(helpers_1.bnArg(0));
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(2);
    }));
    it('Burn from works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, defaultSigner: sender, accounts: [alice], query } = yield setup();
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(3);
        yield contract.tx.setApprovalForAll(alice.address, true);
        yield helpers_1.fromSigner(contract, alice.address).tx.burnFrom(sender.address, helpers_1.bnArg(0));
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(2);
    }));
    it('Burn from without allowance should fail', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, defaultSigner: sender, accounts: [alice], query } = yield setup();
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(3);
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.burnFrom(sender.address, helpers_1.bnArg(0)))
            .to.eventually.be.rejected;
        yield helpers_1.expect(query.balanceOf(sender.address)).to.have.output(3);
    }));
});
