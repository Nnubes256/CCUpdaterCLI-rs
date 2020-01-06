#[derive(Clap)]
#[clap(version = "0.1", author = "CCDirectLink Contributors")]
pub struct BaseOptions {
    #[clap(flatten)]
    pub common_options: CommonOptions,
    #[clap(subcommand)]
    pub subcommand: Subcommand,
}

#[derive(Clap)]
pub struct CommonOptions {
    #[clap(long = "game", default_value = ".", help = "Sets the game folder used for operations")]
    pub game: String,
}

#[derive(Clap)]
#[clap(version = "0.1", author = "CCDirectLink Contributors")]
pub enum Subcommand {
    #[clap(name = "list", about = "List all the mods that the tool knows about")]
    List(SubListOptions),
    #[clap(name = "outdated", about = "Show the names and versions of outdated mods")]
    Outdated(SubOutdatedOptions),
    #[clap(name = "install", about = "Install one or more mods")]
    Install(SubInstallOptions),
    #[clap(name = "uninstall", about = "Uninstall one or more mods")]
    Uninstall(SubUninstallOptions),
    #[clap(name = "update", about = "Update one or more mods")]
    Update(SubUpdateOptions),
}

#[derive(Clap)]
#[clap(name = "list", version = "0.1", author = "CCDirectLink Contributors")]
pub struct SubListOptions {}

#[derive(Clap)]
#[clap(name = "outdated", version = "0.1", author = "CCDirectLink Contributors")]
pub struct SubOutdatedOptions {}

#[derive(Clap)]
#[clap(name = "install", version = "0.1", author = "CCDirectLink Contributors")]
pub struct SubInstallOptions {
    #[clap(help = "The mods to uninstall")]
    pub mods: Vec<String>,
}

#[derive(Clap)]
#[clap(name = "uninstall", version = "0.1", author = "CCDirectLink Contributors")]
pub struct SubUninstallOptions {
    #[clap(help = "The mods to uninstall")]
    pub mods: Vec<String>,
}

#[derive(Clap)]
#[clap(name = "update", version = "0.1", author = "CCDirectLink Contributors")]
pub struct SubUpdateOptions {
    #[clap(help = "The mods to uninstall", min_values = 0)]
    pub mods: Vec<String>,
}
