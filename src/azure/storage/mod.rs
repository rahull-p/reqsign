//! Azure Storage Signer

mod signer;

pub use signer::Signer as AzureStorageSigner;

mod config;

pub use config::Config as AzureStorageConfig;

mod credential;

pub use credential::Credential as AzureStorageCredential;

mod imds_credential;

mod workload_identity_credential;

mod loader;

pub use loader::Loader as AzureStorageLoader;

mod client_secret_credential;
mod sas;
