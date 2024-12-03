//! Contract module that allows children to implement role-based access control
//! mechanisms. This is a lightweight version that doesn't allow enumerating
//! role members except through off-chain means by accessing the contract event
//! logs.
//!
//! Roles are referred to by their `bytes32` identifier. These should be exposed
//! in the external API and be unique. The best way to achieve this is by using
//! `pub const` hash digests:
//!
//! ```no_run
//! // Output of `keccak256("MY_ROLE")`.
//! pub const MY_ROLE: [u8; 32] = [151, 200, 119, 228, 14, 219, 65, 113,
//!                                15, 11, 175, 88, 140, 135, 142, 225,
//!                                90, 4, 73, 155, 6, 174, 140, 152, 207,
//!                                72, 136, 117, 217, 26, 114, 19];
//! ```
//!
//! Roles can be used to represent a set of permissions. To restrict access to a
//! function call, use [`AccessControl::has_role`]:
//!
//! ```rust,ignore
//! pub fn foo() {
//!   assert!(self.has_role(MY_ROLE.into(), msg::sender()));
//!   // ...
//! }
//! ```
//!
//! Roles can be granted and revoked dynamically via the `grant_role` and
//! `revoke_role` functions. Each role has an associated admin role, and only
//! accounts that have a `role`'s `admin_role` can call `grant_role` and
//! `revoke_role`.
//!
//! By default, the admin role for all roles is `DEFAULT_ADMIN_ROLE`, which
//! means that only accounts with this role will be able to grant or revoke
//! other roles. More complex role relationships can be created by using
//! `_set_role_admin`.
//!
//! WARNING: The `DEFAULT_ADMIN_ROLE` is also its own admin: it has permission
//! to grant and revoke this role. Extra precautions should be taken to secure
//! accounts that have been granted it. We recommend using
//! `AccessControlDefaultAdminRules` to enforce additional security measures for
//! this role.
use alloy_primitives::{Address, B256};
use alloy_sol_types::sol;
use stylus_sdk::{
    evm, msg,
    stylus_proc::{public, sol_storage, SolidityError},
};

sol! {
    /// Emitted when `new_admin_role` is set as `role`'s admin role, replacing
    /// `previous_admin_role`.
    ///
    /// `DEFAULT_ADMIN_ROLE` is the starting admin for all roles, despite
    /// `RoleAdminChanged` not being emitted signaling this.
    #[allow(missing_docs)]
    event RoleAdminChanged(bytes32 indexed role, bytes32 indexed previous_admin_role, bytes32 indexed new_admin_role);
    /// Emitted when `account` is granted `role`.
    ///
    /// `sender` is the account that originated the contract call. This account
    /// bears the admin role (for the granted role).
    /// Expected in cases where the role was granted using the internal
    /// [`AccessControl::grant_role`].
    #[allow(missing_docs)]
    event RoleGranted(bytes32 indexed role, address indexed account, address indexed sender);
    /// Emitted when `account` is revoked `role`.
    ///
    /// `sender` is the account that originated the contract call:
    ///   - if using `revoke_role`, it is the admin role bearer.
    ///   - if using `renounce_role`, it is the role bearer (i.e. `account`).
    #[allow(missing_docs)]
    event RoleRevoked(bytes32 indexed role, address indexed account, address indexed sender);
}

sol! {
    /// The `account` is missing a role.
    ///
    /// * `account` - Account that was found to not be authorized.
    /// * `needed_role` - The missing role.
    #[derive(Debug)]
    #[allow(missing_docs)]
    error AccessControlUnauthorizedAccount(address account, bytes32 needed_role);
    /// The caller of a function is not the expected one.
    ///
    /// NOTE: Don't confuse with [`AccessControlUnauthorizedAccount`].
    #[derive(Debug)]
    #[allow(missing_docs)]
    error AccessControlBadConfirmation();
}

/// An error that occurred in the implementation of an [`AccessControl`]
/// contract.
#[derive(SolidityError, Debug)]
pub enum Error {
    /// The caller account is missing a role.
    UnauthorizedAccount(AccessControlUnauthorizedAccount),
    /// The caller of a afunction is not the expected one.
    BadConfirmation(AccessControlBadConfirmation),
}

sol_storage! {
    /// Information about a specific role.
    pub struct RoleData {
        /// Whether an account is member of a certain role.
        mapping(address => bool) has_role;
        /// The admin role for this role.
        bytes32 admin_role;
    }

    /// State of an `AccessControl` contract.
    pub struct AccessControl {
        /// Role identifier -> Role information.
        mapping(bytes32 => RoleData) _roles;
    }
}

pub const DEFAULT_ADMIN_ROLE: [u8; 32] = [0; 32];
#[public]
impl AccessControl {
   
}

impl AccessControl {
  
}
