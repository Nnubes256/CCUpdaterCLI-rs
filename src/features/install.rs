use super::Result as FeatureResult;
use options::CommonOptions;
use options::SubInstallOptions;

pub fn run(common_options: CommonOptions, options: SubInstallOptions) -> FeatureResult<()> {
    println!("Here goes install logic");
    Ok(())
}
