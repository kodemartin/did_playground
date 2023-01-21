//! This is an example of using the `did_playground` library.
//!
//! It creates two decentralized identifiers (DID)
//! associated with a subject and publishes them to the IOTA
//! distributed ledger.
//!
//! The subjects then communicated through HTTP to perform
//! mutual authentication.
use did_playground::error::Result;
use did_playground::interface::SubjectInterface;
use did_playground::SubjectBuilder;
use identity_iota::client::Resolver;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn use_tracing_subscriber() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() -> Result<()> {
    use_tracing_subscriber();
    env_logger::init();

    tracing::info!("Creating subjects with published DIDs");
    let bob = tokio::spawn(SubjectBuilder::new()?.build());
    let alice = tokio::spawn(SubjectBuilder::new()?.build());
    let bob = bob.await.expect("there shouldn't be any join error")?;
    let alice = alice.await.expect("there shouldn't be any join error")?;

    tracing::info!("Creating web interfaces");
    let mut bob = SubjectInterface::from((bob, Resolver::new().await?));
    let alice = SubjectInterface::from((alice, Resolver::new().await?));

    tracing::info!("Starting web interface for bob");
    bob.up().await?;
    tracing::info!("Starting handshake between alice and bob");
    alice
        .handshake(format!("http://localhost:{}", bob.port()))
        .await?;
    tracing::info!("Alice and bob have been mutually authenticated!");

    Ok(())
}
