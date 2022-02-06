import {RedspotUserConfig} from '@supercolony/redspot/types'
import '@supercolony/redspot-patract'
import '@supercolony/redspot-chai'
import '@supercolony/redspot-gas-reporter'

export default {
    defaultNetwork: 'development',
    contract: {
        ink: {
            toolchain: 'nightly',
            sources: ['example_project_structure/contracts/**', 'examples/**', 'mock/**']
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
} as RedspotUserConfig