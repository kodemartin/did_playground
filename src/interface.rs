//! Manage abstractions and logic for the interface between
//! decentralized identities
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::{Extension, Json};
use axum::routing::post;
use axum::Router;
use identity_iota::client::Resolver;
use identity_iota::crypto::{Ed25519, Sign, Verify};
use identity_iota::iota_core::IotaDID;
use reqwest::IntoUrl;
use serde::{Deserialize, Serialize};
use std::net::TcpListener;

use crate::error::{DidPlaygroundError, Result};
use crate::Subject;

/// Web interface of a subject
#[derive(Debug)]
pub struct SubjectInterface {
    subject: Arc<Subject>,
    resolver: Arc<Resolver>,
    port: u16,
}

impl From<(Subject, Resolver)> for SubjectInterface {
    fn from((subject, resolver): (Subject, Resolver)) -> Self {
        let subject = Arc::new(subject);
        let resolver = Arc::new(resolver);
        Self {
            subject,
            resolver,
            port: 0,
        }
    }
}

impl SubjectInterface {
    /// The service endpoint for authentication
    pub const AUTH_ENDPOINT: &str = "/auth";

    /// Get the Url of the service that manages authentication.
    ///
    /// This is the "local" alternative to any service added
    /// to the DID document itself. See more in the [docs][adding-a-service].
    ///
    /// [adding-a-service]: https://wiki.iota.org/identity.rs/concepts/decentralized_identifiers/update/#adding-a-service
    pub fn auth_url(&self) -> reqwest::Url {
        reqwest::Url::try_from(format!("http://localhost:{}", self.port).as_str())
            .expect("this should be a valid base url")
            .join(Self::AUTH_ENDPOINT)
            .expect("this should be a valid endpoint")
    }

    /// Get the tcp port of the interface
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Send a signed hello message to another subject interface
    ///
    /// Upon successful authentication, the remote subject sends
    /// a message to let this subject proceed with mutual authentication
    pub async fn handshake(&self, remote_url: impl IntoUrl) -> Result<()> {
        let request = AuthRequest::try_from(&*self.subject)?;
        let response: Option<AuthRequest> = reqwest::Client::new()
            .post(remote_url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;
        if let Some(remote_msg) = response {
            remote_msg.verify(&self.resolver).await
        } else {
            Err(DidPlaygroundError::Hello)
        }
    }

    /// Start the web interface of the inner subject
    pub async fn up(&mut self) -> Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))?;
        let addr = listener.local_addr()?;
        let port = match addr {
            SocketAddr::V4(addr) => addr.port(),
            SocketAddr::V6(addr) => addr.port(),
        };
        self.port = port;

        let service = Router::new()
            .route(Self::AUTH_ENDPOINT, post(auth))
            .layer(Extension(Arc::clone(&self.subject)))
            .layer(Extension(Arc::clone(&self.resolver)));
        // TODO: Store the JoinHandle to enable aborting
        tokio::spawn(async move {
            axum::Server::from_tcp(listener)?
                .serve(service.into_make_service())
                .await
        });

        tracing::info!(
            "Subject with id {} listening for other subjects at {}",
            self.subject.account.did(),
            addr
        );

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthRequest {
    did: IotaDID,
    data: Vec<u8>,
    sig: Vec<u8>,
}

impl TryFrom<&Subject> for AuthRequest {
    type Error = DidPlaygroundError;

    fn try_from(subject: &Subject) -> Result<Self> {
        let data = b"hello".to_vec();
        let sig = Ed25519::sign(&data, subject.keypair.private())?.into();
        Ok(Self {
            did: subject.account.did().clone(),
            data,
            sig,
        })
    }
}

impl AuthRequest {
    /// Verify that the signature has been signed by the subject associated
    /// with the `did`.
    pub async fn verify(&self, resolver: &Resolver) -> Result<()> {
        let remote_document = resolver.resolve(&self.did).await.map(|d| d.document)?;
        let remote_pubkey = remote_document
            .default_signing_method()
            .map(|m| m.data().try_decode())??;
        Ok(Ed25519::verify(&self.data, &self.sig, &remote_pubkey)?)
    }
}

/// Handle authentication requests from remote subject interfaces
async fn auth(
    Extension(subject): Extension<Arc<Subject>>,
    Extension(resolver): Extension<Arc<Resolver>>,
    Json(msg): Json<AuthRequest>,
) -> Json<Option<AuthRequest>> {
    tracing::debug!("Received auth request: {:?}", msg);
    Json(if msg.verify(&resolver).await.is_ok() {
        AuthRequest::try_from(&*subject).ok()
    } else {
        None
    })
}
