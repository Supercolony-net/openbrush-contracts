#[macro_export]
macro_rules! address_of {
    ($account:ident) => {
        ink::primitives::AccountId::from(ink_e2e::$account::<PolkadotConfig>().account_id().0)
    };
}

#[macro_export]
macro_rules! balance_of {
    ($client:ident, $address:ident, $account:ident) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.balance_of(address_of!($account)));
            $client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await.return_value()
        }
    }
}

#[macro_export]
macro_rules! owner_of {
    ($client:ident, $address:ident, $id:expr) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.owner_of($id));
            $client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await.return_value()
        }
    }
}

#[macro_export]
macro_rules! balance_of_37 {
    ($client:ident, $address:ident, $account:ident, $token:expr) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.balance_of(address_of!($account), $token));
            $client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await.return_value()
        }
    }
}

#[macro_export]
macro_rules! has_role {
    ($client:ident, $address:ident, $role:expr, $account:ident) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.has_role($role, address_of!($account)));
            $client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await.return_value()
        }
    }
}

#[macro_export]
macro_rules! grant_role {
    ($client:ident, $address:ident, $role:expr, $account:ident) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.grant_role($role, address_of!($account)));
            $client.call(&ink_e2e::alice(), _msg, 0, None).await.expect("grant_role failed").return_value()
        }
    }
}

#[macro_export]
macro_rules! revoke_role {
    ($client:ident, $address:ident, $role:expr, $account:ident) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.revoke_role($role, address_of!($account)));
            $client.call(&ink_e2e::alice(), _msg, 0, None).await.expect("revoke_role failed").return_value()
        }
    }
}

#[macro_export]
macro_rules! mint_dry_run {
    ($client:ident, $address:ident, $account:ident, $id:expr) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.mint(address_of!($account), $id));
            $client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await.return_value()
        }
    };
    ($client:ident, $address:ident, $signer:ident, $account:ident, $id:expr) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.mint(address_of!($account), $id));
            $client.call_dry_run(&ink_e2e::$signer(), &_msg, 0, None).await.return_value()
        }
    };
}

#[macro_export]
macro_rules! mint {
    ($client:ident, $address:ident, $account:ident, $id:expr) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.mint(address_of!($account), $id));
            $client.call(&ink_e2e::alice(), _msg, 0, None).await.expect("mint failed").return_value()
        }
    };
    ($client:ident, $address:ident, $signer:ident, $account:ident, $id:expr) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.mint(address_of!($account), $id));
            $client.call(&ink_e2e::$signer(), _msg, 0, None).await.expect("mint failed").return_value()
        }
    };
}

#[macro_export]
macro_rules! get_role_member_count {
    ($client:ident, $address:ident, $role:expr) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.get_role_member_count($role));
            $client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await.return_value()
        }
    }
}

#[macro_export]
macro_rules! get_role_member {
    ($client:ident, $address:ident, $role:expr, $index:expr) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.get_role_member($role, $index));
            $client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await.return_value()
        }
    }
}

#[macro_export]
macro_rules! method_call {
    ($client:ident, $address:ident, $method:ident) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.$method());
            $client.call(&ink_e2e::alice(), _msg, 0, None).await.expect("method_call failed").return_value()
        }
    };
    ($client:ident, $address:ident, $signer:ident, $method:ident) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.$method());
            $client.call(&ink_e2e::$signer(), _msg, 0, None).await.expect("method_call failed").return_value()
        }
    };
}

#[macro_export]
macro_rules! method_call_dry_run {
    ($client:ident, $address:ident, $method:ident) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.$method());
            $client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await.return_value()
        }
    };
    ($client:ident, $address:ident, $signer:ident, $method:ident) => {
        {
            let _msg = build_message::<ContractRef>($address.clone())
                .call(|contract| contract.$method());
            $client.call_dry_run(&ink_e2e::$signer(), &_msg, 0, None).await.return_value()
        }
    };
}