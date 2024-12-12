use crate::types::ProverType;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, sync::OnceLock};

#[derive(Debug, Serialize, Deserialize)]
pub struct CircuitConfig {
    pub hard_fork_name: String,
    pub params_path: String,
    pub assets_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoordinatorConfig {
    pub base_url: String,
    pub retry_count: u32,
    pub retry_wait_time_sec: u64,
    pub connection_timeout_sec: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct L2GethConfig {
    pub endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub prover_name: String,
    pub keystore_path: String,
    pub keystore_password: String,
    pub db_path: String,
    pub prover_type: ProverType,
    pub low_version_circuit: CircuitConfig,
    pub high_version_circuit: CircuitConfig,
    pub coordinator: CoordinatorConfig,
    pub l2geth: Option<L2GethConfig>,
}

impl Config {
    pub fn from_reader<R>(reader: R) -> Result<Self>
    where
        R: std::io::Read,
    {
        serde_json::from_reader(reader).map_err(|e| anyhow::anyhow!(e))
    }

    pub fn from_file(file_name: String) -> Result<Self> {
        let file = File::open(file_name)?;
        Config::from_reader(&file)
    }
}

static SCROLL_PROVER_ASSETS_DIR_ENV_NAME: &str = "SCROLL_PROVER_ASSETS_DIR";
static SCROLL_PROVER_ASSETS_DIRS: OnceLock<Vec<String>> = OnceLock::new();

#[derive(Debug)]
pub struct AssetsDirEnvConfig {}

impl AssetsDirEnvConfig {
    pub fn init() -> Result<()> {
        let value = std::env::var(SCROLL_PROVER_ASSETS_DIR_ENV_NAME)?;
        let dirs: Vec<&str> = value.split(',').collect();
        if dirs.len() != 2 {
            bail!("env variable SCROLL_PROVER_ASSETS_DIR value must be 2 parts seperated by comma.")
        }

        SCROLL_PROVER_ASSETS_DIRS.get_or_init(|| dirs.into_iter().map(|s| s.to_string()).collect());
        log::info!(
            "init SCROLL_PROVER_ASSETS_DIRS: {:?}",
            SCROLL_PROVER_ASSETS_DIRS
        );
        Ok(())
    }

    pub fn enable_first() {
        debug_assert!(
            SCROLL_PROVER_ASSETS_DIRS.get().is_some()
                && SCROLL_PROVER_ASSETS_DIRS.get().unwrap().len() >= 2
        );
        log::info!(
            "set env {SCROLL_PROVER_ASSETS_DIR_ENV_NAME} to {}",
            &SCROLL_PROVER_ASSETS_DIRS.get().unwrap()[0]
        );
        std::env::set_var(
            SCROLL_PROVER_ASSETS_DIR_ENV_NAME,
            &SCROLL_PROVER_ASSETS_DIRS.get().unwrap()[0],
        );
    }

    pub fn enable_second() {
        debug_assert!(
            SCROLL_PROVER_ASSETS_DIRS.get().is_some()
                && SCROLL_PROVER_ASSETS_DIRS.get().unwrap().len() >= 2
        );
        log::info!(
            "set env {SCROLL_PROVER_ASSETS_DIR_ENV_NAME} to {}",
            &SCROLL_PROVER_ASSETS_DIRS.get().unwrap()[1]
        );
        std::env::set_var(
            SCROLL_PROVER_ASSETS_DIR_ENV_NAME,
            &SCROLL_PROVER_ASSETS_DIRS.get().unwrap()[1],
        );
    }
}
