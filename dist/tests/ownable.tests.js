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
describe('MY_OWNABLE', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('my_ownable', 'new');
        });
    }
    it('OWNABLE - owner is by default contract deployer', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, defaultSigner: sender } = yield setup();
        // Assert - Sender is by default the owner of the contract
        yield helpers_1.expect(query.owner()).to.have.output(sender.address);
    }));
    it('OWNABLE - only owner is allowed to mint', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = yield setup();
        // Arrange - Alice is not the owner hence minting should fail
        yield helpers_1.expect(query.owner()).to.have.output(sender.address);
        yield helpers_1.expect(contract.tx.mint(helpers_1.bnArg(0), 1)).to.eventually.be.fulfilled;
        // Act & Assert - Alice can mint a token
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.mint(helpers_1.bnArg(0), 100)).to.eventually.be.rejected;
    }));
    it('OWNABLE - transfer ownership works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, query, tx, defaultSigner: sender, accounts: [alice] } = yield setup();
        // Arrange - Alice is not the owner hence minting should fail
        yield helpers_1.expect(query.owner()).to.have.output(sender.address);
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.mint(helpers_1.bnArg(0), 100)).to.eventually.be.rejected;
        // Act - transfer ownership to Alice
        yield tx.transferOwnership(alice.address);
        yield helpers_1.expect(query.owner()).to.have.output(alice.address);
        // Assert - Alice can mint a token
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.mint(helpers_1.bnArg(0), 100)).to.eventually.be.fulfilled;
        yield helpers_1.expect(query.balanceOf(alice.address, helpers_1.bnArg(0))).to.have.output(100);
    }));
    it('OWNABLE - renounce ownership works', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, tx, defaultSigner: sender } = yield setup();
        // Arrange - Sender is the owner
        yield helpers_1.expect(query.owner()).to.have.output(sender.address);
        // Act - Sender renounce his role
        yield helpers_1.expect(tx.renounceOwnership()).to.eventually.be.fulfilled;
        // Assert - Zero account is now the owner
        yield helpers_1.expect(query.owner()).to.have.output(constants_1.consts.EMPTY_ADDRESS);
    }));
    it('OWNABLE - cannot renounce ownership if not owner', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, query, defaultSigner: sender, accounts: [alice] } = yield setup();
        // Arrange - Sender is the owner
        yield helpers_1.expect(query.owner()).to.have.output(sender.address);
        // Act - Alice try to call renounce his role
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.renounceOwnership()).to.eventually.be.rejected;
        // Assert - Sender is still the owner
        yield helpers_1.expect(query.owner()).to.have.output(sender.address);
    }));
});
