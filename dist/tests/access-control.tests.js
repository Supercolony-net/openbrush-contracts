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
describe('MY_ACCESS_CONTROL', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            return helpers_1.setupContract('my_access_control', 'new');
        });
    }
    it('ACCESS CONTROL - only minter role is allowed to mint', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, query, tx, accounts: [alice] } = yield setup();
        // Arrange - Alice doesn't have Minter role hence minting should fail
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(false);
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.mint(helpers_1.bnArg(1))).to.eventually.be.rejected;
        // Act - Grant Alice the minter role
        yield tx.grantRole(constants_1.Roles.Minter, alice.address);
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(true);
        // Assert - Alice can mint a token
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.mint(helpers_1.bnArg(0))).to.eventually.be.fulfilled;
        yield helpers_1.expect(query.ownerOf(helpers_1.bnArg(0))).to.have.output(alice.address);
    }));
    it('ACCESS CONTROL - should grant initial roles to default signer', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, defaultSigner: sender } = yield setup();
        // Assert - After sender has deployed a contract instance, Sender should has default roles
        yield helpers_1.expect(query.hasRole(constants_1.Roles.DefaultAdminRole, sender.address)).to.have.output(true);
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, sender.address)).to.have.output(true);
    }));
    it('ACCESS CONTROL - should not grant initial roles for random role', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, accounts: [alice] } = yield setup();
        // Assert - After sender has deployed a contract instance, Alice should not has any role
        yield helpers_1.expect(query.hasRole(constants_1.Roles.DefaultAdminRole, alice.address)).to.have.output(false);
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(false);
    }));
    it('ACCESS CONTROL - should grant role', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, tx, accounts: [alice] } = yield setup();
        // Arrange - Check that Alice has not a minter role
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(false);
        // Act - Grant Alice the minter Role
        yield tx.grantRole(constants_1.Roles.Minter, alice.address);
        // Assert - Alice has minter role
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(true);
    }));
    it('ACCESS CONTROL - should not change old roles after grant role', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, tx, defaultSigner, accounts: [alice] } = yield setup();
        // Arrange - Alice don't have minter role, sender has default roles
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(false);
        yield helpers_1.expect(query.hasRole(constants_1.Roles.DefaultAdminRole, defaultSigner.address)).to.have.output(true);
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, defaultSigner.address)).to.have.output(true);
        // Act - Grant Alice the minter role
        yield tx.grantRole(constants_1.Roles.Minter, alice.address);
        // Assert - Alice has minter role, and sender still have gra
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(true);
        yield helpers_1.expect(query.hasRole(constants_1.Roles.DefaultAdminRole, defaultSigner.address)).to.have.output(true);
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, defaultSigner.address)).to.have.output(true);
    }));
    it('ACCESS CONTROL - should revoke role', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, tx, accounts: [alice] } = yield setup();
        // Arrange - Grant Alice minter role
        yield tx.grantRole(constants_1.Roles.Minter, alice.address);
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(true);
        // Act - Revoke Alice minter role
        yield tx.revokeRole(constants_1.Roles.Minter, alice.address);
        // Assert - Alice don't have minter role
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(false);
    }));
    it('ACCESS CONTROL - should renounce role', () => __awaiter(void 0, void 0, void 0, function* () {
        const { query, contract, accounts: [alice] } = yield setup();
        // Arrange - Grant Alice minter role
        yield contract.tx.grantRole(constants_1.Roles.Minter, alice.address);
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(true);
        // Act - Alice renounce his minter role
        yield helpers_1.fromSigner(contract, alice.address).tx.renounceRole(constants_1.Roles.Minter, alice.address);
        // Assert - Alice don't have minter role
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(false);
    }));
    it('ACCESS CONTROL - should reject when grant/revoke not by admin role', () => __awaiter(void 0, void 0, void 0, function* () {
        const { tx, contract, accounts: [alice, bob] } = yield setup();
        // Assert - Only sender has admin role
        yield tx.grantRole(constants_1.Roles.Minter, bob.address);
        // Act & Assert - Alice & Bob can't grant or revoke roles
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.grantRole(constants_1.Roles.Minter, alice.address)).to.eventually.be.rejected;
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.revokeRole(constants_1.Roles.Minter, bob.address)).to.eventually.be.rejected;
    }));
    it('ACCESS CONTROL - should reject when renounce not self role', () => __awaiter(void 0, void 0, void 0, function* () {
        const { tx, query, defaultSigner, contract, accounts: [alice] } = yield setup();
        // Arrange - Grant Alice minter role
        yield tx.grantRole(constants_1.Roles.Minter, alice.address);
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(true);
        // Act & Assert - Sender calling renounce for Alice should fail
        yield helpers_1.expect(helpers_1.fromSigner(contract, defaultSigner.address).tx.renounceRole(constants_1.Roles.Minter, alice.address)).to.eventually.be.rejected;
    }));
    it('ACCESS CONTROL - should reject burn if no minter role', () => __awaiter(void 0, void 0, void 0, function* () {
        const { tx, query, contract, accounts: [alice] } = yield setup();
        // Assert - Grant Alice minter role & mint a token
        yield tx.grantRole(constants_1.Roles.Minter, alice.address);
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(true);
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.mint(helpers_1.bnArg(0))).to.eventually.be.fulfilled;
        yield helpers_1.expect(query.ownerOf(helpers_1.bnArg(0))).to.have.output(alice.address);
        // Act - revoke Alice minter role
        yield tx.revokeRole(constants_1.Roles.Minter, alice.address);
        yield helpers_1.expect(query.hasRole(constants_1.Roles.Minter, alice.address)).to.have.output(false);
        // Assert - Alice cannot burn token
        yield helpers_1.expect(helpers_1.fromSigner(contract, alice.address).tx.burn(alice.address, helpers_1.bnArg(0), 1)).to.eventually.be.rejected;
    }));
});
