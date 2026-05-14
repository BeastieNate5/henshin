use crate::{appcontext::AppContext, cli::ThemeCommands};
use anyhow::{Context, Result};

pub fn handle_comamnd(ctx: AppContext, cmd: ThemeCommands) -> Result<()> {
    match cmd {
        ThemeCommands::New { name, theme } => create_theme(ctx, name.as_str(), theme),
        ThemeCommands::List => list_themes(ctx),
        ThemeCommands::Delete { name } => delete_theme(ctx, name.as_str()),
        ThemeCommands::Current => current_theme(ctx),
        ThemeCommands::Load { name } => load_theme(ctx, name.as_str())
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

fn create_theme(ctx: AppContext, name: &str, from_theme: Option<String>) -> Result<()> {
    match from_theme {
        Some(theme) => ctx.create_theme_from(theme.as_str(), name)?,
        None => ctx.create_blank_theme(name)?
    };

    println!("[ \x1b[92mOK\x1b[0m ] '{name}' theme created");
    Ok(())
}

fn delete_theme(ctx: AppContext, name: &str) -> Result<()> {
    let current_theme = ctx.get_current_theme()?;

    if current_theme == name {
        println!(
            "[ WARN ] Cannot delete current theme, please switch to another theme to delete '{name}'"
        );
        return Ok(());
    }

    ctx.delete_theme(name)?;
    println!("[ \x1b[92mOK\x1b[0m ] '{name}' theme deleted");
    Ok(())
}

fn load_theme(ctx: AppContext, name: &str) -> Result<()> {
    ctx.switch_current(name)?;
    println!("[ \x1b[92mOK\x1b[0m ] Switched theme to '{name}'");
    ctx.create_misisng_theme_files()?;
    Ok(())
}
