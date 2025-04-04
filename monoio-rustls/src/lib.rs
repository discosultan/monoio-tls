#![allow(stable_features)]

mod client;
mod error;
mod server;
mod stream;

pub use client::{
    TlsConnector, TlsStream as ClientTlsStream, TlsStreamReadHalf as ClientTlsStreamReadHalf,
    TlsStreamWriteHalf as ClientTlsStreamWriteHalf,
};
pub use error::TlsError;
pub use server::{
    TlsAcceptor, TlsStream as ServerTlsStream, TlsStreamReadHalf as ServerTlsStreamReadHalf,
    TlsStreamWriteHalf as ServerTlsStreamWriteHalf,
};
pub use stream::Stream;

/// A wrapper around an underlying raw stream which implements the TLS protocol.
pub type TlsStream<IO> = stream::Stream<IO, rustls::Connection>;

impl<IO> From<ClientTlsStream<IO>> for TlsStream<IO> {
    fn from(value: ClientTlsStream<IO>) -> Self {
        value.map_conn(|c| c.into())
    }
}

impl<IO> From<ServerTlsStream<IO>> for TlsStream<IO> {
    fn from(value: ServerTlsStream<IO>) -> Self {
        value.map_conn(|c| c.into())
    }
}
