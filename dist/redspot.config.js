"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
require("@redspot/patract");
require("@redspot/chai");
require("@redspot/gas-reporter");
exports.default = {
    defaultNetwork: 'development',
    contract: {
        ink: {
            toolchain: 'nightly',
            sources: ['examples/**', 'mock/**']
        }
    },
    networks: {
        development: {
            endpoint: 'ws://127.0.0.1:9944',
            types: {},
            gasLimit: '400000000000',
            explorerUrl: 'https://polkadot.js.org/apps/#/explorer/query/?rpc=ws://127.0.0.1:9944/'
        },
        substrate: {
            endpoint: 'ws://127.0.0.1:9944',
            gasLimit: '400000000000',
            accounts: ['//Alice'],
            types: {}
        }
    },
    mocha: {
        timeout: 60000
    }
};
