use super::Result as FeatureResult;
use options::CommonOptions;
use options::SubUninstallOptions;

pub fn run(common_options: CommonOptions, options: SubUninstallOptions) -> FeatureResult<()> {
    println!("Here goes uninstall logic");
    Ok(())
}
