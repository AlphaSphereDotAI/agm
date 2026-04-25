use anyhow::Result;

use crate::cli::parser::{Cli, Commands, McpAction, SkillAction};
use crate::config::base::{Base, CONFIG_FILE, init_config, load_config};

/// Executes the parsed CLI command.
pub fn run(cli: Cli, version: &str) -> Result<()> {
    match cli.command {
        Some(Commands::Init) => {
            init_config(CONFIG_FILE, &Base::new(version.to_string()))?;
            println!("Created {CONFIG_FILE}.");
        }
        Some(Commands::Skill {
            action: SkillAction::List,
        }) => list_entries(
            load_config(CONFIG_FILE)?.skills,
            "skills",
            "No managed skills found.",
        ),
        Some(Commands::Mcp {
            action: McpAction::List,
        }) => list_entries(
            load_config(CONFIG_FILE)?.mcps,
            "MCPs",
            "No managed MCPs found.",
        ),
        None => {
            println!("No command provided. Use --help to see available commands.");
        }
    }

    Ok(())
}

fn list_entries(entries: Vec<String>, _label: &str, empty_message: &str) {
    if entries.is_empty() {
        println!("{empty_message}");
        return;
    }

    for entry in entries {
        println!("{entry}");
    }
}
