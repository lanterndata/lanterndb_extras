use std::str::FromStr;

use crate::{openai_runtime::OpenAiRuntime, cohere_runtime::CohereRuntime, ort_runtime::OrtRuntime, runtime::EmbeddingRuntime};

fn default_logger(text: &str) {
    println!("{}", text);
}

#[derive(Debug, PartialEq, Clone)]
pub enum Runtime {
    Ort,
    OpenAi,
    Cohere,
}

pub type LoggerFn = fn(&str);
impl FromStr for Runtime {
    type Err = anyhow::Error;
    fn from_str(input: &str) -> Result<Runtime, anyhow::Error> {
        match input {
            "ort" => Ok(Runtime::Ort),
            "openai" => Ok(Runtime::OpenAi),
            "cohere" => Ok(Runtime::Cohere),
            _ => anyhow::bail!("Invalid runtime {input}"),
        }
    }
}

impl ToString for Runtime {
    fn to_string(&self) -> String {
        match self {
            Runtime::Ort => "ort".to_owned(),
            Runtime::OpenAi => "openai".to_owned(),
            Runtime::Cohere => "cohere".to_owned(),
        }
    }
}

pub fn get_runtime<'a>(
    runtime: &Runtime,
    logger: Option<&'a LoggerFn>,
    params: &'a str,
) -> Result<Box<dyn EmbeddingRuntime + 'a>, anyhow::Error> {
    Ok(match runtime {
        Runtime::Ort => Box::new(OrtRuntime::new(
            logger.unwrap_or(&(default_logger as LoggerFn)),
            params,
        )?),
        Runtime::OpenAi => Box::new(OpenAiRuntime::new(
            logger.unwrap_or(&(default_logger as LoggerFn)),
            params,
        )?),
        Runtime::Cohere => Box::new(CohereRuntime::new(
            logger.unwrap_or(&(default_logger as LoggerFn)),
            params,
        )?),
    })
}

pub fn get_available_runtimes() -> Vec<String> {
    todo!();
}