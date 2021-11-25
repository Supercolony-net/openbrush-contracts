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
describe('REENTRANCY_GUARD', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('my_flipper_guard', 'new');
        });
    }
    function setup_flip_on_me() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('flip_on_me', 'new');
        });
    }
    it('One flip works correct', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, query, defaultSigner: sender } = yield setup();
        // Arrange - Ensure flip value is false
        yield helpers_1.expect(query.getValue()).to.have.output(false);
        // Act - Flip
        yield helpers_1.expect(contract.tx.flip()).to.eventually.be.fulfilled;
        // Assert - Flip value must be true after flip
        yield helpers_1.expect(query.getValue()).to.have.output(true);
    }));
    it('Two flips work correct', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, query, defaultSigner: sender } = yield setup();
        // Arrange - Ensure flip value is false
        yield helpers_1.expect(query.getValue()).to.have.output(false);
        // Act - Flip
        yield helpers_1.expect(contract.tx.flip()).to.eventually.be.fulfilled;
        yield helpers_1.expect(contract.tx.flip()).to.eventually.be.fulfilled;
        // Assert - After two flips value must be false again
        yield helpers_1.expect(query.getValue()).to.have.output(false);
    }));
    it('Call flip on me must fails', () => __awaiter(void 0, void 0, void 0, function* () {
        const { tx, query, defaultSigner: sender } = yield setup();
        const { contract } = yield setup_flip_on_me();
        // Arrange - Ensure flip value is false
        yield helpers_1.expect(query.getValue()).to.have.output(false);
        // Assert
        yield helpers_1.expect(tx.callFlipOnMe(contract.address)).to.eventually.be.rejected;
        // Assert - Value still must be false, because flip failed
        yield helpers_1.expect(query.getValue()).to.have.output(false);
    }));
});
