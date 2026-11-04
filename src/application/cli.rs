use super::{call, Options, KEYS};
use anyhow::{Context, Result};
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::Path;

pub fn command() -> Command {
    Command::new("ui")
        .about("Inspect or operate a native or WASM application's semantic UI")
        .subcommand_required(true)
        .subcommand(Command::new("capabilities"))
        .subcommand(Command::new("snapshot"))
        .subcommand(
            Command::new("query")
                .arg(Arg::new("id").long("id"))
                .arg(Arg::new("role").long("role"))
                .arg(Arg::new("name").long("name")),
        )
        .subcommand(Command::new("activate").arg(Arg::new("id").required(true)))
        .subcommand(
            Command::new("fill")
                .arg(Arg::new("id").required(true))
                .arg(Arg::new("value").required(true).allow_hyphen_values(true)),
        )
        .subcommand(
            Command::new("set-value")
                .arg(Arg::new("id").required(true))
                .arg(
                    Arg::new("value")
                        .required(true)
                        .allow_hyphen_values(true)
                        .value_parser(finite_value),
                ),
        )
        .subcommand(
            Command::new("key")
                .arg(Arg::new("key").required(true).value_parser(KEYS))
                .arg(Arg::new("extend").long("extend").action(ArgAction::SetTrue))
                .arg(
                    Arg::new("by-word")
                        .long("by-word")
                        .action(ArgAction::SetTrue),
                ),
        )
}

fn finite_value(value: &str) -> std::result::Result<f64, String> {
    let value = value.parse::<f64>().map_err(|e| e.to_string())?;
    if value.is_finite() {
        Ok(value)
    } else {
        Err("value must be finite".into())
    }
}
pub async fn execute(root: &Path, options: &Options, args: &ArgMatches) -> Result<()> {
    use jet_bridge_protocol::Command as Ui;
    let (name, args) = args.subcommand().context("UI subcommand required")?;
    let string = |field| args.get_one::<String>(field).cloned();
    let id = || string("id").expect("ID is required");
    let command = match name {
        "capabilities" => Ui::Capabilities,
        "snapshot" => Ui::Snapshot,
        "query" => Ui::Query {
            id: string("id"),
            role: string("role"),
            name: string("name"),
        },
        "activate" => Ui::Activate { id: id() },
        "fill" => Ui::Fill {
            id: id(),
            value: string("value").expect("value is required"),
        },
        "set-value" => Ui::SetValue {
            id: id(),
            value: *args.get_one::<f64>("value").expect("value is required"),
        },
        "key" => Ui::Key {
            key: serde_json::from_value(string("key").expect("key is required").into())?,
            extend: args.get_flag("extend"),
            by_word: args.get_flag("by-word"),
        },
        _ => unreachable!("Clap validates the UI command"),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&call(root, options, command).await?)?
    );
    Ok(())
}
