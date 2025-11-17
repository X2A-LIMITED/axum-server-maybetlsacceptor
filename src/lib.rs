//! axum-server-maybetlsacceptor is an [axum-server]-compatible enum that lets
//! you easily accepts connections with or without TLS. The main goals were:
//! - no duplication of routes, layers or other serving code to add the acceptor
//! - no dynamic dispatch
//! - ability to choose TLS backend (rustls, openssl)
//!
//! [pin-project] was chosen instead of [pin-project-lite] because it does not
//! play well with other attributes such as `cfg`.
//!
//! ## Features
//!
//! By default, no feature is enabled, meaning no TLS backend is available.
//!
//! - `rustls`: enable the rustls TLS backend
//! - `openssl`: enable the openssl TLS backend
//!
//! ## Compatibility
//!
//! Version `0.7.x` is compatible with [axum-server] version `0.7.y`.
//!
//! This crate's versioning will try to follow [axum-server]'s major versioning
//! (or minor while being unstable `0.x.y`).
//!
//! ## Usage example
//!
//! You can find examples in [repository].
//!
//! [repository]: https://github.com/X2A-LIMITED/axum-server-maybetlsacceptor
//! [axum-server]: https://crates.io/crates/axum-server
//! [pin-project]: https://github.com/taiki-e/pin-project
//! [pin-project-lite]: https://github.com/taiki-e/pin-project-lite

// Copyright 2025 X2A Holding SAS, SIREN 990970519, France
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#![cfg_attr(docsrs, feature(doc_cfg))]

use axum_server::accept::{Accept, DefaultAcceptor};
#[cfg(feature = "openssl")]
use axum_server::tls_openssl::OpenSSLAcceptor;
#[cfg(feature = "rustls")]
use axum_server::tls_rustls::RustlsAcceptor;
use pin_project::pin_project;
use tokio::io::{AsyncRead, AsyncWrite};

#[derive(Clone)]
pub enum MaybeTlsAcceptor {
    Default(DefaultAcceptor),
    #[cfg(feature = "rustls")]
    Rustls(RustlsAcceptor),
    #[cfg(feature = "openssl")]
    Openssl(OpenSSLAcceptor),
}

impl<I, S> Accept<I, S> for MaybeTlsAcceptor
where
    I: AsyncRead + AsyncWrite + Unpin,
{
    type Stream = MaybeTlsAcceptorStream<I, S>;
    type Service = S;
    type Future = MaybeTlsAcceptorFuture<I, S>;

    fn accept(&self, stream: I, service: S) -> Self::Future {
        match self {
            MaybeTlsAcceptor::Default(inner) => {
                MaybeTlsAcceptorFuture::Default(inner.accept(stream, service))
            }
            #[cfg(feature = "rustls")]
            MaybeTlsAcceptor::Rustls(inner) => {
                MaybeTlsAcceptorFuture::Rustls(inner.accept(stream, service))
            }
            #[cfg(feature = "openssl")]
            MaybeTlsAcceptor::Openssl(inner) => {
                MaybeTlsAcceptorFuture::Openssl(inner.accept(stream, service))
            }
        }
    }
}

impl Default for MaybeTlsAcceptor {
    fn default() -> Self {
        Self::Default(DefaultAcceptor::default())
    }
}

#[pin_project(project = MaybeTlsAcceptorFutureProj)]
pub enum MaybeTlsAcceptorFuture<I, S>
where
    I: AsyncRead + AsyncWrite + Unpin,
{
    Default(#[pin] <DefaultAcceptor as Accept<I, S>>::Future),
    #[cfg(feature = "rustls")]
    Rustls(#[pin] <RustlsAcceptor as Accept<I, S>>::Future),
    #[cfg(feature = "openssl")]
    Openssl(#[pin] <OpenSSLAcceptor as Accept<I, S>>::Future),
}

impl<I, S> Future for MaybeTlsAcceptorFuture<I, S>
where
    I: AsyncRead + AsyncWrite + Unpin,
{
    type Output = std::io::Result<(MaybeTlsAcceptorStream<I, S>, S)>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.project();

        match this {
            MaybeTlsAcceptorFutureProj::Default(future) => future
                .poll(cx)
                .map_ok(|(stream, service)| (MaybeTlsAcceptorStream::Default(stream), service)),
            #[cfg(feature = "rustls")]
            MaybeTlsAcceptorFutureProj::Rustls(future) => future
                .poll(cx)
                .map_ok(|(stream, service)| (MaybeTlsAcceptorStream::Rustls(stream), service)),
            #[cfg(feature = "openssl")]
            MaybeTlsAcceptorFutureProj::Openssl(future) => future
                .poll(cx)
                .map_ok(|(stream, service)| (MaybeTlsAcceptorStream::Openssl(stream), service)),
        }
    }
}

#[pin_project(project = MaybeTlsAcceptorStreamProj)]
pub enum MaybeTlsAcceptorStream<I, S>
where
    I: AsyncRead + AsyncWrite + Unpin,
{
    Default(#[pin] <DefaultAcceptor as Accept<I, S>>::Stream),
    #[cfg(feature = "rustls")]
    Rustls(#[pin] <RustlsAcceptor as Accept<I, S>>::Stream),
    #[cfg(feature = "openssl")]
    Openssl(#[pin] <OpenSSLAcceptor as Accept<I, S>>::Stream),
}

impl<I, S> AsyncRead for MaybeTlsAcceptorStream<I, S>
where
    I: AsyncRead + AsyncWrite + Unpin,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        let this = self.project();

        match this {
            MaybeTlsAcceptorStreamProj::Default(stream) => stream.poll_read(cx, buf),
            #[cfg(feature = "rustls")]
            MaybeTlsAcceptorStreamProj::Rustls(stream) => stream.poll_read(cx, buf),
            #[cfg(feature = "openssl")]
            MaybeTlsAcceptorStreamProj::Openssl(stream) => stream.poll_read(cx, buf),
        }
    }
}

impl<I, S> AsyncWrite for MaybeTlsAcceptorStream<I, S>
where
    I: AsyncRead + AsyncWrite + Unpin,
{
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        let this = self.project();

        match this {
            MaybeTlsAcceptorStreamProj::Default(stream) => stream.poll_write(cx, buf),
            #[cfg(feature = "rustls")]
            MaybeTlsAcceptorStreamProj::Rustls(stream) => stream.poll_write(cx, buf),
            #[cfg(feature = "openssl")]
            MaybeTlsAcceptorStreamProj::Openssl(stream) => stream.poll_write(cx, buf),
        }
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        let this = self.project();

        match this {
            MaybeTlsAcceptorStreamProj::Default(stream) => stream.poll_flush(cx),
            #[cfg(feature = "rustls")]
            MaybeTlsAcceptorStreamProj::Rustls(stream) => stream.poll_flush(cx),
            #[cfg(feature = "openssl")]
            MaybeTlsAcceptorStreamProj::Openssl(stream) => stream.poll_flush(cx),
        }
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        let this = self.project();

        match this {
            MaybeTlsAcceptorStreamProj::Default(stream) => stream.poll_shutdown(cx),
            #[cfg(feature = "rustls")]
            MaybeTlsAcceptorStreamProj::Rustls(stream) => stream.poll_shutdown(cx),
            #[cfg(feature = "openssl")]
            MaybeTlsAcceptorStreamProj::Openssl(stream) => stream.poll_shutdown(cx),
        }
    }
}
