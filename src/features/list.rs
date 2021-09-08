use super::Result as FeatureResult;
use options::CommonOptions;
use options::SubListOptions;

pub fn run(common_options: CommonOptions, options: SubListOptions) -> FeatureResult<()> {
    println!("Here goes list logic");
    Ok(())
}
