// SPDX-License-Identifier: Apache-2.0
// Copyright Authors of bpfd

use std::{fs::remove_file, path::Path};

use bpfd_api::{
    config::{self, Config},
    util::directories::STDIR_BYTECODE_IMAGE_CONTENT_STORE,
    v1::bpfd_server::BpfdServer,
};

use log::{debug, info};
use tokio::{
    net::UnixListener,
    runtime::Runtime,
    select,
    signal::unix::{signal, SignalKind},
    sync::mpsc,
    task::JoinHandle,
};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::Server;

use crate::{
    bpf::BpfManager,
    oci_utils::ImageManager,
    rpc::BpfdLoader,
    static_program::get_static_programs,
    storage::StorageManager,
    utils::{set_file_permissions, SOCK_MODE},
};

pub fn serve(
    runtime: Runtime,
    config: Config,
    static_program_path: &str,
    csi_support: bool,
) -> anyhow::Result<()> {
    let (tx, rx) = mpsc::channel(32);

    let loader = BpfdLoader::new(tx.clone());
    let service = BpfdServer::new(loader);

    let endpoints = config.grpc.endpoints.clone();
    let listeners_handle = runtime.spawn(async move {
        let mut listeners: Vec<_> = Vec::new();
        for endpoint in endpoints {
            match endpoint {
                config::Endpoint::Unix { path, enabled } => {
                    if !enabled {
                        info!("Skipping disabled endpoint on {path}");
                        continue;
                    }

                    match serve_unix(path.clone(), service.clone()).await {
                        Ok(handle) => listeners.push(handle),
                        Err(e) => eprintln!("Error = {e:?}"),
                    }
                }
            }
        }
        for listener in listeners {
            match listener.await {
                Ok(()) => {}
                Err(e) => eprintln!("Error = {e:?}"),
            }
        }
    });

    let allow_unsigned = config.signing.as_ref().map_or(true, |s| s.allow_unsigned);
    let (itx, irx) = mpsc::channel(32);

    let mut image_manager =
        ImageManager::new(STDIR_BYTECODE_IMAGE_CONTENT_STORE, allow_unsigned, irx)?;
    let image_manager_handle = runtime.spawn(async move {
        image_manager.run().await;
    });

    let mut bpf_manager = BpfManager::new(config, rx, itx);
    bpf_manager.rebuild_state()?;

    let static_programs = get_static_programs(static_program_path)?;

    // Load any static programs first
    if !static_programs.is_empty() {
        for prog in static_programs {
            let ret_prog = bpf_manager.add_program(prog)?;
            // Get the Kernel Info.
            let kernel_info = ret_prog
                .kernel_info()
                .expect("kernel info should be set for all loaded programs");
            info!("Loaded static program with program id {}", kernel_info.id)
        }
    };
    let mut handles = vec![listeners_handle, image_manager_handle];
    
    if csi_support {
        let storage_manager = StorageManager::new(tx);
        let storage_manager_handle = runtime.spawn(storage_manager.run());
        handles.push(storage_manager_handle);
    }

    loop {
        
            _ = shutdown_handler() => {
                info!("Signal received to stop command processing");
                return;
            }
            _ = bpf_manager.process_command() => {}
        }
    
}

pub(crate) async fn shutdown_handler() {
    let mut sigint = signal(SignalKind::interrupt()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    select! {
        _ = sigint.recv() => {debug!("Received SIGINT")},
        _ = sigterm.recv() => {debug!("Received SIGTERM")},
    }
}

async fn serve_unix(
    path: String,
    service: BpfdServer<BpfdLoader>,
) -> anyhow::Result<JoinHandle<()>> {
    // Listen on Unix socket
    if Path::new(&path).exists() {
        // Attempt to remove the socket, since bind fails if it exists
        remove_file(&path)?;
    }

    let uds = UnixListener::bind(&path)?;
    let uds_stream = UnixListenerStream::new(uds);
    // Always set the file permissions of our listening socket.
    set_file_permissions(&path.clone(), SOCK_MODE);

    let serve = Server::builder()
        .add_service(service)
        .serve_with_incoming_shutdown(uds_stream, shutdown_handler());

    Ok(tokio::spawn(async move {
        info!("Listening on {path}");
        if let Err(e) = serve.await {
            eprintln!("Error = {e:?}");
        }
        info!("Shutdown Unix Handler {}", path);
    }))
}
