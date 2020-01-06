#[derive(Debug, Clap)]
#[clap(version = "0.1", author = "CCDirectLink Contributors")]
pub struct BaseOptions {
    #[clap(flatten)]
    pub common_options: CommonOptions,
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Debug, Clap)]
pub struct CommonOptions {
    #[clap(long = "game", default_value = ".", help = "Sets the game folder used for operations")]
    pub game: String,
}

#[derive(Debug, Clap)]
pub enum Subcommand {
    #[clap(name = "list", about = "List all the mods that the tool knows about")]
    List(SubListOptions),
    #[clap(name = "outdated", about = "Show the names and versions of outdated mods")]
    Outdated(SubOutdatedOptions),
    #[clap(name = "install", about = "Install one or more mods")]
    Install(SubInstallOptions),
    #[clap(name = "uninstall", visible_alias = "remove", about = "Uninstall one or more mods")]
    Uninstall(SubUninstallOptions),
    #[clap(name = "update", visible_alias = "upgrade", about = "Update one or more mods")]
    Update(SubUpdateOptions),
}

#[derive(Debug, Clap)]
pub struct SubListOptions {}

#[derive(Debug, Clap)]
pub struct SubOutdatedOptions {}

#[derive(Debug, Clap)]
pub struct SubInstallOptions {
    #[clap(help = "The mods to install", required = true)]
    pub mods: Vec<String>,
}

#[derive(Debug, Clap)]
pub struct SubUninstallOptions {
    #[clap(help = "The mods to uninstall", required = true)]
    pub mods: Vec<String>,
}

#[derive(Debug, Clap)]
pub struct SubUpdateOptions {
    #[clap(help = "The mods to update")]
    pub mods: Vec<String>,
}
