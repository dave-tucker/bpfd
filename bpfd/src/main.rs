// SPDX-License-Identifier: Apache-2.0
// Copyright Authors of bpfd

use std::{
    env,
    fs::{create_dir_all, read_to_string, File},
    io::{BufRead, BufReader},
    str::FromStr,
};

mod bpf;
mod command;
mod dispatcher_config;
mod errors;
mod multiprog;
mod netlink;
mod oci_utils;
mod rpc;
mod serve;
mod static_program;
mod storage;
mod utils;

use anyhow::{bail, Context};
use bpfd_api::{config::Config, util::directories::*};
use clap::Parser;
use log::{debug, error, info, warn};
use nix::{
    libc::RLIM_INFINITY,
    sys::resource::{setrlimit, Resource},
    unistd::{getuid, User},
};
use systemd_journal_logger::{connected_to_journal, JournalLog};
use tokio::runtime::Builder;
use utils::{create_bpffs, set_dir_permissions};

use crate::serve::serve;
const BPFD_ENV_LOG_LEVEL: &str = "RUST_LOG";

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    csi_support: bool,
}

fn main() -> anyhow::Result<()> {
    let runtime = Builder::new_multi_thread()
        .enable_all()
        .on_thread_start(|| {
            drop_linux_capabilities();
        })
        .build()
        .expect("Unable to create tokio runtime");
    let args = Args::parse();
    if connected_to_journal() {
        // If bpfd is running as a service, log to journald.
        JournalLog::default()
            .with_extra_fields(vec![("VERSION", env!("CARGO_PKG_VERSION"))])
            .install()
            .unwrap();
        manage_journal_log_level();
        log::info!("Log using journald");
    } else {
        // Otherwise fall back to logging to standard error.
        env_logger::init();
        log::info!("Log using env_logger");
    }

    has_cap(caps::CapSet::Effective, caps::Capability::CAP_BPF);
    has_cap(caps::CapSet::Effective, caps::Capability::CAP_SYS_ADMIN);

    setrlimit(Resource::RLIMIT_MEMLOCK, RLIM_INFINITY, RLIM_INFINITY).unwrap();

    // Create directories associated with bpfd
    create_dir_all(RTDIR).context("unable to create runtime directory")?;
    create_dir_all(RTDIR_FS).context("unable to create mountpoint")?;
    create_dir_all(RTDIR_TC_INGRESS_DISPATCHER).context("unable to create dispatcher directory")?;
    create_dir_all(RTDIR_TC_EGRESS_DISPATCHER).context("unable to create dispatcher directory")?;
    create_dir_all(RTDIR_XDP_DISPATCHER).context("unable to create dispatcher directory")?;
    create_dir_all(RTDIR_PROGRAMS).context("unable to create programs directory")?;

    if !is_bpffs_mounted()? {
        create_bpffs(RTDIR_FS)?;
    }
    create_dir_all(RTDIR_FS_XDP).context("unable to create xdp distpacher dir")?;
    create_dir_all(RTDIR_FS_TC_INGRESS).context("unable to create tc ingress dispatcher dir")?;
    create_dir_all(RTDIR_FS_TC_EGRESS).context("unable to create tc egress dispatcher dir")?;
    create_dir_all(RTDIR_FS_MAPS).context("unable to create maps directory")?;
    create_dir_all(RTDIR_BPFD_CSI).context("unable to create CSI directory")?;
    create_dir_all(RTDIR_BPFD_CSI_FS).context("unable to create socket directory")?;

    create_dir_all(CFGDIR_STATIC_PROGRAMS).context("unable to create static programs directory")?;

    create_dir_all(STDIR_BYTECODE_IMAGE_CONTENT_STORE)
        .context("unable to create bytecode image store directory")?;

    set_dir_permissions(CFGDIR, CFGDIR_MODE);
    set_dir_permissions(RTDIR, RTDIR_MODE);
    set_dir_permissions(STDIR, STDIR_MODE);

    let config = if let Ok(c) = read_to_string(CFGPATH_BPFD_CONFIG) {
        c.parse().unwrap_or_else(|_| {
            warn!("Unable to parse config file, using defaults");
            Config::default()
        })
    } else {
        warn!("Unable to read config file, using defaults");
        Config::default()
    };

    serve(runtime, config, CFGDIR_STATIC_PROGRAMS, args.csi_support)?;
    Ok(())
}

fn manage_journal_log_level() {
    // env_logger uses the environment variable RUST_LOG to set the log
    // level. Parse RUST_LOG to set the log level for journald.
    log::set_max_level(log::LevelFilter::Error);
    if env::var(BPFD_ENV_LOG_LEVEL).is_ok() {
        let rust_log = log::LevelFilter::from_str(&env::var(BPFD_ENV_LOG_LEVEL).unwrap());
        match rust_log {
            Ok(value) => log::set_max_level(value),
            Err(e) => log::error!("Invalid Log Level: {}", e),
        }
    }
}

fn drop_all_cap(cap: caps::CapSet) {
    match caps::clear(None, cap) {
        Ok(()) => debug!("CAPS:  {:?} Cleared", cap),
        Err(e) => error!("CAPS:  Clear {:?} Error  {}", cap, e),
    }
}

fn has_cap(cset: caps::CapSet, cap: caps::Capability) {
    info!("Has {}: {}", cap, caps::has_cap(None, cset, cap).unwrap());
}

fn drop_linux_capabilities() {
    let uid = getuid();
    let res = match User::from_uid(uid) {
        Ok(res) => res.unwrap(),
        Err(e) => {
            warn!("Unable to map user id {} to a name. err: {}", uid, e);
            debug!(
                "Running as user id {}, skip dropping all capabilities for spawned threads",
                uid
            );
            return;
        }
    };

    if res.name == "bpfd" {
        debug!(
            "Running as user {}, dropping all capabilities for spawned threads",
            res.name
        );
        drop_all_cap(caps::CapSet::Ambient);
        drop_all_cap(caps::CapSet::Bounding);
        drop_all_cap(caps::CapSet::Effective);
        drop_all_cap(caps::CapSet::Inheritable);
        drop_all_cap(caps::CapSet::Permitted);
    } else {
        debug!(
            "Running as user {}, skip dropping all capabilities for spawned threads",
            res.name
        );
    }
}

fn is_bpffs_mounted() -> Result<bool, anyhow::Error> {
    let file = File::open("/proc/mounts").context("Failed to open /proc/mounts")?;
    let reader = BufReader::new(file);
    for l in reader.lines() {
        match l {
            Ok(line) => {
                let parts: Vec<&str> = line.split(' ').collect();
                if parts.len() != 6 {
                    bail!("expected 6 parts in proc mount")
                }
                if parts[0] == "none" && parts[1].contains("bpfd") && parts[2] == "bpf" {
                    return Ok(true);
                }
            }
            Err(e) => bail!("problem reading lines {}", e),
        }
    }
    Ok(false)
}
