## AccessControl contract

Contract module that allows children to implement role-based access
control mechanisms.

Roles can be granted and revoked dynamically via the `grant_role` and
`revoke_role`. functions. Each role has an associated admin role, and only
accounts that have a role's admin role can call `grant_role` and `revoke_role`.

[See example](https://supercolony-net.github.io/openbrush-contracts/smart-contracts/access-control)