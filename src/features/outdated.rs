use super::Result as FeatureResult;
use options::CommonOptions;
use options::SubOutdatedOptions;

pub fn run(common_options: CommonOptions, options: SubOutdatedOptions) -> FeatureResult<()> {
    println!("Here goes outdated logic");
    Ok(())
}
