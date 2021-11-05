use super::Result;
use crate::config::Globals;
use crate::{
    proto::ExperimentRunBase,
    simulation::package::{creator::PackageCreators, output::packages::OutputPackagesSimConfig},
};

#[derive(Clone)]
pub struct Config {
    pub output_config: OutputPackagesSimConfig,
}

impl Config {
    pub fn new_sim(
        exp_config: &super::ExperimentConfig<ExperimentRunBase>,
        globals: &Globals,
        package_creators: &PackageCreators,
    ) -> Result<Config> {
        let output_config = package_creators.get_output_persistence_config(exp_config, globals)?;
        Ok(Config { output_config })
    }
}
