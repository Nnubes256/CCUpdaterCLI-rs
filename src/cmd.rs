#[derive(Clap, Clone)]
#[clap(version = "0.1", author = "CCDirectLink Contributors")]
pub(crate) struct CLIBaseOptions {
    #[clap(long = "game", default_value = ".", help = "Sets the game folder used for operations")]
    pub(crate) game: String,
    #[clap(subcommand)]
    pub(crate) subcommand: CLISubcommand,
}

#[derive(Clap, Clone)]
#[clap(version = "0.1", author = "CCDirectLink Contributors")]
pub(crate) enum CLISubcommand {
    #[clap(name = "list", about = "List all the mods that the tool knows about")]
    List(CLISubListOptions),
    #[clap(name = "outdated", about = "Show the names and versions of outdated mods")]
    Outdated(CLISubOutdatedOptions),
    #[clap(name = "install", about = "Install one or more mods")]
    Install(CLISubInstallOptions),
    #[clap(name = "uninstall", about = "Uninstall one or more mods")]
    Uninstall(CLISubUninstallOptions),
    #[clap(name = "update", about = "Update one or more mods")]
    Update(CLISubUpdateOptions),
}

#[derive(Clap, Clone)]
#[clap(name = "list", version = "0.1", author = "CCDirectLink Contributors")]
pub(crate) struct CLISubListOptions {}

#[derive(Clap, Clone)]
#[clap(name = "outdated", version = "0.1", author = "CCDirectLink Contributors")]
pub(crate) struct CLISubOutdatedOptions {}

#[derive(Clap, Clone)]
#[clap(name = "install", version = "0.1", author = "CCDirectLink Contributors")]
pub(crate) struct CLISubInstallOptions {
    #[clap(help = "The mods to uninstall")]
    pub(crate) mods: Vec<String>,
}

#[derive(Clap, Clone)]
#[clap(name = "uninstall", version = "0.1", author = "CCDirectLink Contributors")]
pub(crate) struct CLISubUninstallOptions {
    #[clap(help = "The mods to uninstall")]
    pub(crate) mods: Vec<String>,
}

#[derive(Clap, Clone)]
#[clap(name = "update", version = "0.1", author = "CCDirectLink Contributors")]
pub(crate) struct CLISubUpdateOptions {
    #[clap(help = "The mods to uninstall", min_values = 0)]
    pub(crate) mods: Vec<String>,
}
