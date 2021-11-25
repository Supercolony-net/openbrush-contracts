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
describe('MY_PAUSABLE', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('my_pausable', 'new');
        });
    }
    it('Success flip when not paused', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract } = yield setup();
        yield helpers_1.expect(contract.tx.flip()).to.eventually.be.fulfilled;
    }));
    it('Success pause when not paused', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract } = yield setup();
        yield helpers_1.expect(contract.tx.pause()).to.eventually.be.fulfilled;
    }));
    it('Failed double pause', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract } = yield setup();
        yield helpers_1.expect(contract.tx.pause()).to.eventually.be.fulfilled;
        yield helpers_1.expect(contract.tx.pause()).to.eventually.be.rejected;
    }));
    it('Success pause and unpause', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract } = yield setup();
        yield helpers_1.expect(contract.tx.pause()).to.eventually.be.fulfilled;
        yield helpers_1.expect(contract.tx.unpause()).to.eventually.be.fulfilled;
    }));
    it('Failed unpause', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract } = yield setup();
        yield helpers_1.expect(contract.tx.unpause()).to.eventually.be.rejected;
    }));
    it('Failed flip when paused', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract } = yield setup();
        yield helpers_1.expect(contract.tx.pause()).to.eventually.be.fulfilled;
        yield helpers_1.expect(contract.tx.flip()).to.eventually.be.rejected;
    }));
});
