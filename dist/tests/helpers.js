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
exports.bnArg = exports.fromSigner = exports.setupContract = exports.expect = void 0;
const bn_js_1 = __importDefault(require("bn.js"));
const redspot_1 = require("redspot");
const { getContractFactory, getRandomSigner } = redspot_1.patract;
const { api, getSigners } = redspot_1.network;
var chai_1 = require("./setup/chai");
Object.defineProperty(exports, "expect", { enumerable: true, get: function () { return chai_1.expect; } });
const patchContractMethods = (contract) => {
    patchMethods(contract.query);
    patchMethods(contract.tx);
    let original_tx = contract.tx;
    for (const prop in contract.tx) {
        contract.tx[prop] = function (...args) {
            return __awaiter(this, void 0, void 0, function* () {
                return new Promise(((resolve, reject) => {
                    contract.query[prop](...args).then((result => {
                        console.log(result.output);
                        if (result.output) {
                            reject(result.output);
                        }
                        else {
                            resolve(contract.tx[prop](...args));
                        }
                    })).catch((reason => reject(reason)));
                }));
            });
        };
    }
    // // @ts-ignore
    // contract.tx = new Proxy(original_tx, handler);
    return contract;
};
// It removes prefix from the function and adds only name of method like a function
// PSP22::token_name
// query["PSP22,tokenName"]
// query.tokenName()
const patchMethods = (object) => {
    for (const prop in object) {
        if (prop.includes(',')) {
            const selectors = prop.split(',');
            const method = selectors[selectors.length - 1];
            object[method] = object[prop];
        }
    }
};
const setupContract = (name, constructor, ...args) => __awaiter(void 0, void 0, void 0, function* () {
    const one = new bn_js_1.default(10).pow(new bn_js_1.default(api.registry.chainDecimals[0]));
    const signers = yield getSigners();
    const defaultSigner = yield getRandomSigner(signers[0], one.muln(10000));
    const alice = yield getRandomSigner(signers[1], one.muln(10000));
    const contractFactory = yield getContractFactory(name, defaultSigner);
    const contract = yield contractFactory.deploy(constructor, ...args);
    const abi = redspot_1.artifacts.readArtifact(name);
    patchContractMethods(contract);
    return {
        defaultSigner,
        alice,
        accounts: [alice, yield getRandomSigner(), yield getRandomSigner()],
        contractFactory,
        contract,
        abi,
        one,
        query: contract.query,
        tx: contract.tx
    };
});
exports.setupContract = setupContract;
const fromSigner = (contract, address) => {
    return patchContractMethods(contract.connect(address));
};
exports.fromSigner = fromSigner;
const bnArg = (value, length = 32) => new bn_js_1.default(value, undefined, 'le').toArray('le', length);
exports.bnArg = bnArg;
