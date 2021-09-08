use super::Result as FeatureResult;
use options::CommonOptions;
use options::SubUpdateOptions;

pub fn run(common_options: CommonOptions, options: SubUpdateOptions) -> FeatureResult<()> {
    println!("Here goes update logic");
    Ok(())
}
