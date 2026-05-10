use crate::{appcontext::AppContext, cli::ThemeCommands};
use anyhow::{Context, Result};

pub fn handle_comamnd(ctx: AppContext, cmd: ThemeCommands) -> Result<()> {
    match cmd {
        ThemeCommands::New { name } => create_theme(ctx, name.as_str()),
        ThemeCommands::List => list_themes(ctx),
        ThemeCommands::Delete { name } => delete_theme(ctx, name.as_str()),
        ThemeCommands::Current => current_theme(ctx),
        _ => unreachable!()
    }
}

fn list_themes(ctx: AppContext) -> Result<()> {
    let themes = ctx.list_themes()?;

    for theme in themes {
        println!("{theme}")
    }

    Ok(())
}

fn current_theme(ctx: AppContext) -> Result<()> {
    let theme = ctx.get_current_theme()?;
    println!("{theme}");
    Ok(())
}

fn create_theme(ctx: AppContext, name: &str) -> Result<()> {
    ctx.create_blank_theme(name)?;
    println!("[ \x1b[92mOK\x1b[0m ] '{name}' theme created");
    Ok(())
}

fn delete_theme(ctx: AppContext, name: &str) -> Result<()> {
    ctx.delete_theme(name)?;
    println!("[ \x1b[91mOK\x1b[0m ] '{name}' theme deleted");
    Ok(())
}
