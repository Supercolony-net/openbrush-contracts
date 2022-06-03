import { RedspotUserConfig } from 'redspot/types'
import '@redspot/patract'
import '@redspot/chai'
import '@redspot/gas-reporter'
const types = {
  // ContractExecResult: 
  //   {
  //     _enum : {
  //       Ok: "Null",
  //       Err:"ContractExecResultErr"
  //   }
  // },
  ContractsPsp34Id: {
    _enum: {
      U8: 'u8',
      U16: 'u16',
      U32: 'u32',
      U64: 'u64',
      U128: 'u128',
      Bytes: 'Vec<u8>'
    }
  },
  OpenbrushContractsDiamondFacetCut: {
    hash: '[u8; 32]',
    selectors: 'Vec<[u8; 4]>'
  },
  ContractsErrorsPsp22Psp22Error : {
    _enum : {
      Custom : 'String',
      InsufficientBalance : 'Null',
      InsufficientAllowance : 'Null',
      ZeroRecipientAddress : 'Null',
      ZeroSenderAddress : 'Null',
      SafeTransferCheckFailed : 'String',
    }
  },
  ContractsErrorsPsp34Psp34Error : {
    _enum : {
      Custom : 'String',
      SelfApprove : 'Null',
      NotApproved : 'Null',
      TokenExists : 'Null',
      TokenNotExists : 'Null',
      SafeTransferCheckFailed : 'String',
    }
  },
  ContractsErrorsAccessControlAccessControlError : {
    _enum : {
      InvalidCaller : 'Null',
      MissingRole : 'Null',
      RoleRedundant : 'Null'
    }
  },
  ContractsErrorsPausablePausableError : {
    _enum : {
      Paused : 'Null',
      NotPaused :  'Null'
    }
  },
  
  LendingProjectLendingLendingError : {
    _enum : {
      PSP22Error : 'ContractsErrorsPsp22Psp22Error',
      PSP34Error : 'ContractsErrorsPsp34Psp34Error',
      AccessControlError : 'ContractsErrorsAccessControlAccessControlError',
      PausableError : 'ContractsErrorsPausablePausableError',

      InsufficientAllowanceToLend : 'Null',
      InsufficientBalanceToLend : 'Null',
      InsufficientAllowanceToRepay : 'Null',
      InsufficientBalanceToRepay : 'Null',
      InsufficientAllowanceForCollateral : 'Null',
      InsufficientCollateralBalance : 'Null',
      AmountNotSupported : 'Null',
      InsufficientBalanceInContract : 'Null',
      AssetNotSupported : 'Null',
      AssetSupported : 'Null',
      NotTheOwner : 'Null',
      LoanLiquidated : 'Null',
      CanNotBeLiquidated : 'Null',
      AssetsInTheContract : 'Null',
    }
  }

}

export default {
  defaultNetwork: 'development',
  contract: {
    ink: {
      toolchain: 'nightly',
      sources: ['example_project_structure/contracts/**', 'examples/**/', 'mock/**', `!examples/reentrancy_guard/Cargo.toml`]
    }
  },
  networks: {
    development: {
      endpoint: 'ws://127.0.0.1:9944',
      types,
      gasLimit: '400000000000',
      explorerUrl: 'https://polkadot.js.org/apps/#/explorer/query/?rpc=ws://127.0.0.1:9944/'
    },
    substrate: {
      endpoint: 'ws://127.0.0.1:9944',
      gasLimit: '400000000000',
      accounts: ['//Alice'],
      types
    }
  },
  mocha: {
    timeout: 60000,
    fullTrace: true
  }
} as RedspotUserConfig
