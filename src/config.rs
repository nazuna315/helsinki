use std::collections::BTreeMap;
use std::path::PathBuf;
use std::{env, fs};

use anyhow::{Context, Result};
use toml::Table;

pub fn config_path() -> Result<PathBuf> {
    let config_dir = if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        PathBuf::from(xdg)
    } else {
        dirs::home_dir()
            .context("Could not determine home directory")?
            .join(".config")
    };
    Ok(config_dir.join("helsinki").join("helsinki.toml"))
}

pub fn load() -> Result<BTreeMap<String, BTreeMap<String, String>>> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(BTreeMap::new());
    }
    let content =
        fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
    let root: Table = content
        .parse()
        .with_context(|| format!("Failed to parse {}", path.display()))?;

    let mut profiles = BTreeMap::new();
    for (profile_name, value) in &root {
        if let Some(table) = value.as_table() {
            profiles.insert(profile_name.clone(), flatten_table(table, ""));
        }
    }
    Ok(profiles)
}

pub fn save(profiles: &BTreeMap<String, BTreeMap<String, String>>) -> Result<()> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }

    let mut content = String::new();
    for (i, (profile_name, entries)) in profiles.iter().enumerate() {
        if i > 0 {
            content.push('\n');
        }
        content.push_str(&format!("[{profile_name}]\n"));
        for (key, value) in entries {
            let escaped = toml::Value::String(value.clone()).to_string();
            content.push_str(&format!("{key} = {escaped}\n"));
        }
    }
    fs::write(&path, content).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

/// Converts a nested TOML table into a flat map with dot-separated keys.
///
/// For example, `[user] name = "John"` becomes `"user.name" => "John"`.
/// This maps directly to git config key format (`user.name`, `user.email`, etc.).
/// The `prefix` parameter accumulates parent keys during recursion; pass an empty
/// string for the initial call.
pub(crate) fn flatten_table(table: &Table, prefix: &str) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();

    for (key, value) in table {
        let full_key = if prefix.is_empty() {
            key.clone()
        } else {
            format!("{prefix}.{key}")
        };

        match value {
            toml::Value::Table(nested) => {
                result.extend(flatten_table(nested, &full_key));
            }
            toml::Value::String(s) => {
                result.insert(full_key, s.clone());
            }
            other => {
                result.insert(full_key, other.to_string());
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_single_level() {
        let toml: Table = toml::from_str(r#"name = "John""#).unwrap();
        let result = flatten_table(&toml, "");
        assert_eq!(result.get("name").unwrap(), "John");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn flatten_nested() {
        let toml: Table = toml::from_str(
            r#"
            [user]
            name = "John Doe"
            email = "john@example.com"
            "#,
        )
        .unwrap();
        let result = flatten_table(&toml, "");
        assert_eq!(result.get("user.name").unwrap(), "John Doe");
        assert_eq!(result.get("user.email").unwrap(), "john@example.com");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn flatten_deeply_nested() {
        let toml: Table = toml::from_str(
            r#"
            [a]
            [a.b]
            c = "deep"
            "#,
        )
        .unwrap();
        let result = flatten_table(&toml, "");
        assert_eq!(result.get("a.b.c").unwrap(), "deep");
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn flatten_with_prefix() {
        let toml: Table = toml::from_str(r#"name = "John""#).unwrap();
        let result = flatten_table(&toml, "user");
        assert_eq!(result.get("user.name").unwrap(), "John");
    }

    #[test]
    fn flatten_empty_table() {
        let toml = Table::new();
        let result = flatten_table(&toml, "");
        assert!(result.is_empty());
    }

    #[test]
    fn flatten_non_string_value() {
        let toml: Table = toml::from_str(r#"count = 42"#).unwrap();
        let result = flatten_table(&toml, "");
        assert_eq!(result.get("count").unwrap(), "42");
    }

}
