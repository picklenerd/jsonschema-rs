use crate::{
    compilation::{context::CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
    keywords::CompilationResult,
    validator::Validate,
};
use serde_json::{Map, Value};
use std::f64::EPSILON;

pub(crate) struct MultipleOfFloatValidator {
    multiple_of: f64,
    path: Vec<String>,
}

impl MultipleOfFloatValidator {
    #[inline]
    pub(crate) fn compile(multiple_of: f64, path: Vec<String>) -> CompilationResult {
        Ok(Box::new(MultipleOfFloatValidator { multiple_of, path }))
    }
}

impl Validate for MultipleOfFloatValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Number(item) = instance {
            let item = item.as_f64().expect("Always valid");
            let remainder = (item / self.multiple_of) % 1.;
            if !(remainder < EPSILON && remainder < (1. - EPSILON)) {
                return false;
            }
        }
        true
    }

    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Number(item) = instance {
            let item = item.as_f64().expect("Always valid");
            let remainder = (item / self.multiple_of) % 1.;
            if !(remainder < EPSILON && remainder < (1. - EPSILON)) {
                return error(ValidationError::multiple_of(
                    self.path.clone(),
                    instance,
                    self.multiple_of,
                ));
            }
        }
        no_error()
    }
}

impl ToString for MultipleOfFloatValidator {
    fn to_string(&self) -> String {
        format!("multipleOf: {}", self.multiple_of)
    }
}

pub(crate) struct MultipleOfIntegerValidator {
    multiple_of: f64,
    path: Vec<String>,
}

impl MultipleOfIntegerValidator {
    #[inline]
    pub(crate) fn compile(multiple_of: f64, path: Vec<String>) -> CompilationResult {
        Ok(Box::new(MultipleOfIntegerValidator { multiple_of, path }))
    }
}

impl Validate for MultipleOfIntegerValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Number(item) = instance {
            let item = item.as_f64().expect("Always valid");
            let is_multiple = if item.fract() == 0. {
                (item % self.multiple_of) == 0.
            } else {
                let remainder = (item / self.multiple_of) % 1.;
                remainder < EPSILON && remainder < (1. - EPSILON)
            };
            if !is_multiple {
                return false;
            }
        }
        true
    }

    fn validate<'a>(&self, _: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if let Value::Number(item) = instance {
            let item = item.as_f64().expect("Always valid");
            let is_multiple = if item.fract() == 0. {
                (item % self.multiple_of) == 0.
            } else {
                let remainder = (item / self.multiple_of) % 1.;
                remainder < EPSILON && remainder < (1. - EPSILON)
            };
            if !is_multiple {
                return error(ValidationError::multiple_of(
                    self.path.clone(),
                    instance,
                    self.multiple_of,
                ));
            }
        }
        no_error()
    }
}

impl ToString for MultipleOfIntegerValidator {
    fn to_string(&self) -> String {
        format!("multipleOf: {}", self.multiple_of)
    }
}
#[inline]
pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    context: &CompilationContext,
) -> Option<CompilationResult> {
    let path = context.curr_path.clone();
    if let Value::Number(multiple_of) = schema {
        let multiple_of = multiple_of.as_f64().expect("Always valid");
        if multiple_of.fract() == 0. {
            Some(MultipleOfIntegerValidator::compile(multiple_of, path))
        } else {
            Some(MultipleOfFloatValidator::compile(multiple_of, path))
        }
    } else {
        Some(Err(CompilationError::SchemaError))
    }
}
