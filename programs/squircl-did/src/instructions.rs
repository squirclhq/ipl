pub mod add_address;
pub mod create_did;
pub mod issue_credential;
pub mod remove_address;
pub mod revoke_credential;
pub mod update_credential;

pub use {
    add_address::*, create_did::*, issue_credential::*, remove_address::*, revoke_credential::*,
    update_credential::*,
};
