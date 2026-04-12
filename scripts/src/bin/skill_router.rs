use clap::Parser;
use design_everything::{DesignState, SkillRouter, Stage, RouterConfig};
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "skill_router")]
#[command(about = "Intelligent skill routing for design-everything", long_about = None)]
struct Cli {
    #[arg(short, long)]
    stage: String,

    #[arg(short, long)]
    state: String,

    #[arg(short, long)]
    config: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let stage = Stage::from_str(&cli.stage)
        .map_err(|e| anyhow::anyhow!("Invalid stage: {}", e))?;

    let state = DesignState::from_json(&cli.state)?;

    let router = if let Some(config_path) = cli.config {
        let config = RouterConfig::from_file(std::path::Path::new(&config_path))?;
        SkillRouter::with_config(config)
    } else {
        SkillRouter::new()
    };

    match router.route(&stage, &state) {
        Some(trigger) => {
            println!("{}", trigger.to_json()?);
        }
        None => {
            println!("{{}}");
        }
    }

    Ok(())
}
