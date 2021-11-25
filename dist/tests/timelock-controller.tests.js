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
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const helpers_1 = require("./helpers");
const redspot_1 = require("redspot");
const bn_js_1 = __importDefault(require("bn.js"));
const { getRandomSigner } = redspot_1.patract;
const { api, getSigners } = redspot_1.network;
function getMessageAbi(contract, ident) {
    return contract.contract.abi.messages.find((message) => message.identifier === ident);
}
describe('MY_TIMELOCK_CONTROLLER', () => {
    function setup() {
        return __awaiter(this, void 0, void 0, function* () {
            const one = new bn_js_1.default(10).pow(new bn_js_1.default(api.registry.chainDecimals[0]));
            const signers = yield getSigners();
            const bob = yield getRandomSigner(signers[1], one.muln(10000));
            let contract = yield helpers_1.setupContract('my_timelock_controller', 'new', 0, [bob.address], [bob.address]);
            return { contract, bob };
        });
    }
    it('TIMELOCK CONTROLLER - can schedule', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, bob } = yield setup();
        // Arrange - Prepare data for schedule
        let transaction = {
            callee: contract.contract.address,
            selector: [0, 0, 0, 0],
            input: [],
            transferred_value: 0,
            gas_limit: 0
        };
        let salt = helpers_1.bnArg(0);
        // Act - Bob scheduled the transaction
        let id = (yield contract.query.hashOperation(transaction, undefined, salt)).output;
        yield helpers_1.expect(contract.query.isOperationPending(id)).to.have.output(false);
        yield helpers_1.expect(helpers_1.fromSigner(contract.contract, bob.address).tx.schedule(transaction, undefined, salt, 0)).to.eventually.be.fulfilled;
        // Assert - Operation must be scheduled, it should be in Pending state and in Ready state(because min delay is zero)
        yield helpers_1.expect(contract.query.isOperationPending(id)).to.have.output(true);
        yield helpers_1.expect(contract.query.isOperationReady(id)).to.have.output(true);
        yield helpers_1.expect(contract.query.isOperationDone(id)).to.have.output(false);
    }));
    it('TIMELOCK CONTROLLER - schedule and execute without input data `TimelockController,get_min_delay`', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, bob } = yield setup();
        // Arrange - Prepare data for execute `get_min_delay`
        const message = getMessageAbi(contract, 'TimelockController,get_min_delay');
        let transaction = {
            callee: contract.contract.address,
            selector: message.selector,
            input: [],
            transferred_value: 0,
            gas_limit: 0
        };
        let salt = helpers_1.bnArg(0);
        // Act - Bob scheduled the transaction
        let id = (yield contract.query.hashOperation(transaction, undefined, salt)).output;
        yield helpers_1.expect(helpers_1.fromSigner(contract.contract, bob.address).tx.schedule(transaction, undefined, salt, 0)).to.eventually.be.fulfilled;
        // Assert - Transaction must be updated and now the state is Done
        yield helpers_1.expect(contract.query.isOperationDone(id)).to.have.output(false);
        yield helpers_1.expect(helpers_1.fromSigner(contract.contract, bob.address).tx.execute(transaction, undefined, salt)).to.eventually.be.fulfilled;
        yield helpers_1.expect(contract.query.isOperationDone(id)).to.have.output(true);
    }));
    it('TIMELOCK CONTROLLER - schedule and execute by passing value into `TimelockController,update_delay`, and update', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, bob } = yield setup();
        // Arrange - Prepare data for execute `update_delay` with a new `min_delay`
        const message = getMessageAbi(contract, 'TimelockController,update_delay');
        const new_min_delay = 15;
        let dataWithSelector = message.toU8a([new_min_delay]);
        // --------
        // Remove selector id
        let data = new Uint8Array(dataWithSelector.length - 4);
        let dataLength = dataWithSelector[0];
        dataLength -= 4 * 4;
        data.set([dataLength]);
        data.set(dataWithSelector.slice(5), 1);
        // --------
        let transaction = {
            callee: contract.contract.address,
            selector: message.selector,
            input: data,
            transferred_value: 0,
            gas_limit: 0
        };
        let salt = helpers_1.bnArg(0);
        // Act - Bob scheduled the transaction
        yield helpers_1.expect(helpers_1.fromSigner(contract.contract, bob.address).tx.schedule(transaction, undefined, salt, 0)).to.eventually.be.fulfilled;
        // Assert - Min delay must be updated via `execute` method
        yield helpers_1.expect(contract.query.getMinDelay()).to.have.output(0);
        yield helpers_1.expect(helpers_1.fromSigner(contract.contract, bob.address).tx.execute(transaction, undefined, salt)).to.eventually.be.fulfilled;
        yield helpers_1.expect(contract.query.getMinDelay()).to.have.output(new_min_delay);
    }));
    it('TIMELOCK CONTROLLER - fails schedule because signer is not proposal', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract } = yield setup();
        // Arrange - Prepare data for schedule
        let transaction = {
            callee: contract.contract.address,
            selector: [0, 0, 0, 0],
            input: [],
            transferred_value: 0,
            gas_limit: 0
        };
        let salt = helpers_1.bnArg(0);
        // Assert - Alice can't schedule the transaction
        yield helpers_1.expect(helpers_1.fromSigner(contract.contract, contract.alice.address).tx.schedule(transaction, undefined, salt, 0)).to.eventually.be.rejected;
    }));
    it('TIMELOCK CONTROLLER - fails execute because signer is not executor', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, bob } = yield setup();
        // Arrange - Prepare data for schedule
        let transaction = {
            callee: contract.contract.address,
            selector: [0, 0, 0, 0],
            input: [],
            transferred_value: 0,
            gas_limit: 0
        };
        let salt = helpers_1.bnArg(0);
        // Act - Bob scheduled the transaction
        yield helpers_1.expect(helpers_1.fromSigner(contract.contract, bob.address).tx.schedule(transaction, undefined, salt, 0)).to.eventually.be.fulfilled;
        // Assert - Alice can't execute the transaction
        yield helpers_1.expect(helpers_1.fromSigner(contract.contract, contract.alice.address).tx.execute(transaction, undefined, salt)).to.eventually.be.rejected;
    }));
    it('TIMELOCK CONTROLLER - fails update_delay', () => __awaiter(void 0, void 0, void 0, function* () {
        const { contract, bob } = yield setup();
        // Assert - Bob is not contract itself, then it must fails
        yield helpers_1.expect(helpers_1.fromSigner(contract.contract, bob.address).tx.updateDelay(15)).to.eventually.be.rejected;
    }));
});
