use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

use crate::core::error::RutesError;

#[derive(Parser)]
#[grammar = "core/pipelines/pipeline.pest"]
struct PipelineParser;

/// The method is used for quick validation of pipeline's syntax
pub fn check_syntax(script: &str) -> Result<(), RutesError> {
    let _ = PipelineParser::parse(Rule::pipeline, script)
        .map_err(|_e| {
            println!("{:?}", _e);
            return RutesError::PipelineSyntaxError
        })?;
    Ok(())
}

pub fn get_script_parameters(script: &str) -> Result<Vec<HashMap<String,String>>, RutesError> {
    let output = PipelineParser::parse(Rule::pipeline, script)
        .map_err(|_e| RutesError::PipelineSyntaxError)?;

    let mut parameters: Vec<HashMap<String, String>> = vec![];

    // We extract the parameters using the grammar tags
    for parameter in output.find_tagged("parameter") {
        let mut param: HashMap<String, String> = HashMap::new();
        for pair in parameter.into_inner().find_tagged("pair") {
            let inner_pair = pair.into_inner();

            let key = String::from(
                inner_pair
                    .find_first_tagged("identifier")
                    .ok_or(RutesError::PipelineSyntaxError)?
                    .as_str(),
            );
            let value = String::from(
                inner_pair
                    .find_first_tagged("value")
                    .ok_or(RutesError::PipelineSyntaxError)?
                    .as_str(),
            );
            param.insert(key, value);
        }
        parameters.push(param);
    }
    Ok(parameters)
}
