mod config;
mod git;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use dialoguer::{Select, theme::ColorfulTheme};

#[derive(Parser)]
#[command(name = "helsinki", about = "Git profile manager for multi-account workflows")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Set or get a profile value (key uses git config format: user.name, user.email, etc.)
    Config {
        /// Profile name
        profile: String,
        /// Git config key (e.g. user.name, user.email, user.signingkey)
        key: String,
        /// Value to set (omit to get current value)
        value: Option<String>,
    },
    /// Apply a profile to the current repository's local git config
    Set {
        /// Profile name (omit for interactive selection)
        profile: Option<String>,
    },
    /// List all registered profiles
    List,
    /// Remove a profile
    Remove {
        /// Profile name to remove
        profile: String,
    },
    /// Set git global config to require local user config (user.useConfigOnly = true)
    Global,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        None => {
            // Show help when no subcommand is given
            use clap::CommandFactory;
            Cli::command().print_help()?;
            println!();
            Ok(())
        }
        Some(Commands::Config { profile, key, value }) => cmd_config(&profile, &key, value),
        Some(Commands::Set { profile }) => cmd_set(profile),
        Some(Commands::List) => cmd_list(),
        Some(Commands::Remove { profile }) => cmd_remove(&profile),
        Some(Commands::Global) => cmd_global(),
    }
}

fn cmd_config(profile: &str, key: &str, value: Option<String>) -> Result<()> {
    let mut profiles = config::load()?;

    match value {
        Some(val) => {
            profiles
                .entry(profile.to_string())
                .or_default()
                .insert(key.to_string(), val);
            config::save(&profiles)?;
        }
        None => {
            let Some(entries) = profiles.get(profile) else {
                bail!("Profile '{profile}' not found");
            };
            let Some(val) = entries.get(key) else {
                bail!("Key '{key}' not found in profile '{profile}'");
            };
            println!("{val}");
        }
    }
    Ok(())
}

fn cmd_set(profile: Option<String>) -> Result<()> {
    let profiles = config::load()?;

    if profiles.is_empty() {
        bail!("No profiles registered. Use 'helsinki config <profile> <key> <value>' to create one.");
    }

    let profile_name = match profile {
        Some(name) => name,
        None => {
            let names: Vec<&String> = profiles.keys().collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a profile")
                .items(&names)
                .default(0)
                .interact()?;

            names[selection].clone()
        }
    };

    let Some(entries) = profiles.get(&profile_name) else {
        bail!("Profile '{profile_name}' not found");
    };

    if entries.is_empty() {
        bail!("Profile '{profile_name}' has no keys configured");
    }

    git::apply_profile(entries)?;
    println!("Applied profile '{profile_name}' to local git config.");
    Ok(())
}

fn cmd_list() -> Result<()> {
    let profiles = config::load()?;

    if profiles.is_empty() {
        println!("No profiles registered.");
        return Ok(());
    }

    for (name, entries) in &profiles {
        println!("[{name}]");
        for (key, value) in entries {
            println!("  {key} = {value}");
        }
    }
    Ok(())
}

fn cmd_remove(profile: &str) -> Result<()> {
    let mut profiles = config::load()?;

    if profiles.remove(profile).is_none() {
        bail!("Profile '{profile}' not found");
    }

    config::save(&profiles)?;
    println!("Removed profile '{profile}'.");
    Ok(())
}

fn cmd_global() -> Result<()> {
    git::set_global("user.useConfigOnly", "true")?;
    println!("Set git global config: user.useConfigOnly = true");
    Ok(())
}
