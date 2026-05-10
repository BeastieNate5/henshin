use crate::{appcontext::AppContext, cli::ThemeCommands};
use anyhow::{Context, Result};

pub fn handle_comamnd(ctx: AppContext, cmd: ThemeCommands) -> Result<()> {
    match cmd {
        ThemeCommands::Create { name } => create_theme(ctx, name.as_str()),
        ThemeCommands::List => list_themes(ctx),
        _ => unreachable!()
    }
}

fn create_theme(ctx: AppContext, name: &str) -> Result<()> {
    Ok(())
}

fn list_themes(ctx: AppContext) -> Result<()> {
    let themes = ctx.list_themes()?;

    for theme in themes {
        println!("{theme}")
    }

    Ok(())
}
