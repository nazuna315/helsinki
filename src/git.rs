use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result, bail};

fn ensure_git_installed() -> Result<()> {
    Command::new("git")
        .arg("--version")
        .output()
        .context("git is not installed or not found in PATH")?;
    Ok(())
}

pub fn ensure_git_repo() -> Result<()> {
    ensure_git_installed()?;
    if !Path::new(".git").exists() {
        bail!("Not a git repository (no .git directory found)");
    }
    Ok(())
}

pub fn apply_profile(entries: &BTreeMap<String, String>) -> Result<()> {
    for (key, value) in entries {
        let status = Command::new("git")
            .args(["config", "--local", key, value])
            .status()
            .with_context(|| format!("Failed to run git config for {key}"))?;
        if !status.success() {
            bail!("git config --local {key} {value} failed");
        }
    }
    Ok(())
}

pub fn set_global(key: &str, value: &str) -> Result<()> {
    ensure_git_installed()?;
    let status = Command::new("git")
        .args(["config", "--global", key, value])
        .status()
        .with_context(|| format!("Failed to run git config --global {key}"))?;
    if !status.success() {
        bail!("git config --global {key} {value} failed");
    }
    Ok(())
}
