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
describe('MY_PSP721_METADATA', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('my_psp721_metadata', 'new', 'Non Fungible Token', 'NFT');
        });
    }
    it('Metadata works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query } = yield setup();
        yield helpers_1.expect(query.name()).to.have.output('Non Fungible Token');
        yield helpers_1.expect(query.symbol()).to.have.output('NFT');
    }));
});
