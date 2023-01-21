//! This library provides elementary functionality for
//! distributed identities on top of [`identity_iota`]
use std::env;
use std::sync::Arc;

use identity_iota::account::{Account, AccountBuilder, IdentitySetup};
use identity_iota::account_storage::Stronghold;
use identity_iota::prelude::*;

pub mod error;
pub mod interface;

use error::Result;

/// Default storage path
static STORAGE_PATH: &str = "./key_manager.hodl";

/// Get access to the account storage
async fn storage() -> Result<Stronghold> {
    let password = env::var("STRONGHOLD_PASSWORD")?;
    let path = env::var("STRONGHOLD_PATH").unwrap_or_else(|_| STORAGE_PATH.into());
    Ok(Stronghold::new(&path, password, None).await?)
}

/// Build a [`Subject`]
pub struct SubjectBuilder {
    keypair: KeyPair,
    account_builder: AccountBuilder,
}

impl SubjectBuilder {
    /// Create a new builder
    pub fn new() -> Result<Self> {
        let keypair = KeyPair::new(KeyType::Ed25519)?;
        let account_builder = Account::<Arc<Client>>::builder();
        Ok(Self {
            keypair,
            account_builder,
        })
    }

    /// Build the [`Subject`] by creating the associated DID
    pub async fn build(self) -> Result<Subject> {
        let identity_setup = IdentitySetup::new().private_key(self.keypair.private().clone());
        let storage = storage().await?;
        let account = self
            .account_builder
            .storage(storage)
            .create_identity(identity_setup)
            .await?;
        Ok(Subject {
            keypair: self.keypair,
            account,
        })
    }
}

/// A subject with a decentralized identifier (DID)
pub struct Subject {
    keypair: KeyPair,
    account: Account,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_subject() {
        let subject = SubjectBuilder::new().unwrap().build().await.unwrap();
        println!(
            "{}: {:#?}",
            subject.account.did(),
            subject.account.document()
        );
    }
}
