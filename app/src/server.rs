use crate::{AppResult, BuiltService, fail};
use std::{
    future::{Future, IntoFuture},
    net::SocketAddr,
    time::Duration,
};
use tokio::sync::oneshot;

pub(crate) async fn serve(
    service: BuiltService,
    address: SocketAddr,
    shutdown_timeout: Duration,
) -> AppResult<()> {
    serve_with_shutdown(service, address, shutdown_timeout, shutdown_signal()).await
}

async fn serve_with_shutdown<S>(
    service: BuiltService,
    address: SocketAddr,
    shutdown_timeout: Duration,
    shutdown: S,
) -> AppResult<()>
where
    S: Future<Output = AppResult<()>> + Send,
{
    let result = async {
        let listener = tokio::net::TcpListener::bind(address).await?;
        log::info!("server listening; address={address}");
        let (shutdown_tx, shutdown_rx) = oneshot::channel();
        let server = axum::serve(listener, service.router.clone())
            .with_graceful_shutdown(async {
                let _ = shutdown_rx.await;
            })
            .into_future();
        let mut server = Box::pin(server);

        tokio::select! {
            result = server.as_mut() => match result {
                Ok(()) => Err(fail("HTTP server stopped before a shutdown signal")),
                Err(error) => Err(Box::new(error) as Box<dyn std::error::Error + Send + Sync>),
            },
            signal = shutdown => match signal {
                Ok(()) => {
                    let _ = shutdown_tx.send(());
                    match tokio::time::timeout(shutdown_timeout, server).await {
                        Ok(Ok(())) => Ok(()),
                        Ok(Err(error)) => Err(Box::new(error) as Box<dyn std::error::Error + Send + Sync>),
                        Err(_) => Err(fail("HTTP shutdown timed out")),
                    }
                }
                Err(error) => Err(error),
            },
        }
    }
    .await;
    service.close().await;
    result
}

#[cfg(unix)]
async fn shutdown_signal() -> AppResult<()> {
    let mut terminate = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?;
    tokio::select! {
        result = tokio::signal::ctrl_c() => result?,
        _ = terminate.recv() => {},
    }
    Ok(())
}

#[cfg(not(unix))]
async fn shutdown_signal() -> AppResult<()> {
    tokio::signal::ctrl_c().await?;
    Ok(())
}
