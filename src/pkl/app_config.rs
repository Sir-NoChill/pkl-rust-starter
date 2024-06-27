// Test
use pkl_bind::evaluator::{evaluator_manager::{self, EvaluatorManager}, module_source::file_source};
use pkl_bind::evaluator::decoder::Pkl;
use pkl_derive::Pkl;

#[derive(Debug, Pkl)]
pub struct AppConfig {
    /// The hostname of this application
    pub host: String,

    /// The port to listen on
    pub port: u16,
}

pub fn load_from_path(path: String) -> Result<AppConfig, &'static str> {
    let mut manager: EvaluatorManager = Default::default();
    let evaluator_id = manager.new_evaluator(None).expect("Failed to create an evaluator"); // create a default evaluator

    let file_source = file_source(path.into());

    let evaluate_module: AppConfig = manager.evaluate_module(file_source.uri().to_string(), evaluator_id)?;
    let config: AppConfig = evaluate_module;

    return Ok(config);
}
