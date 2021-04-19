use crate::keywords::helpers;
use crate::{
    compilation::{context::CompilationContext, JSONSchema},
    error::{error, no_error, ErrorIterator, ValidationError},
    keywords::CompilationResult,
    validator::Validate,
};
use serde_json::{Map, Number, Value};
use std::f64::EPSILON;

struct ConstArrayValidator {
    value: Vec<Value>,
    path: Vec<String>,
}
impl ConstArrayValidator {
    #[inline]
    pub(crate) fn compile(value: &[Value], path: Vec<String>) -> CompilationResult {
        Ok(Box::new(ConstArrayValidator {
            value: value.to_vec(),
            path,
        }))
    }
}
impl Validate for ConstArrayValidator {
    #[inline]
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::constant_array(
                self.path.clone(),
                instance,
                &self.value,
            ))
        }
    }

    #[inline]
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Array(instance_value) = instance {
            helpers::equal_arrays(&self.value, instance_value)
        } else {
            false
        }
    }
}
impl ToString for ConstArrayValidator {
    fn to_string(&self) -> String {
        format!(
            "const: [{}]",
            self.value
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

struct ConstBooleanValidator {
    value: bool,
    path: Vec<String>,
}
impl ConstBooleanValidator {
    #[inline]
    pub(crate) fn compile(value: bool, path: Vec<String>) -> CompilationResult {
        Ok(Box::new(ConstBooleanValidator { value, path }))
    }
}
impl Validate for ConstBooleanValidator {
    #[inline]
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::constant_boolean(
                self.path.clone(),
                instance,
                self.value,
            ))
        }
    }

    #[inline]
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Bool(instance_value) = instance {
            &self.value == instance_value
        } else {
            false
        }
    }
}
impl ToString for ConstBooleanValidator {
    fn to_string(&self) -> String {
        format!("const: {}", self.value)
    }
}

struct ConstNullValidator {
    path: Vec<String>,
}
impl ConstNullValidator {
    #[inline]
    pub(crate) fn compile(path: Vec<String>) -> CompilationResult {
        Ok(Box::new(ConstNullValidator { path }))
    }
}
impl Validate for ConstNullValidator {
    #[inline]
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::constant_null(self.path.clone(), instance))
        }
    }

    #[inline]
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_null()
    }
}
impl ToString for ConstNullValidator {
    fn to_string(&self) -> String {
        format!("const: {}", Value::Null)
    }
}

struct ConstNumberValidator {
    // This is saved in order to ensure that the error message is not altered by precision loss
    original_value: Number,
    value: f64,
    path: Vec<String>,
}

impl ConstNumberValidator {
    #[inline]
    pub(crate) fn compile(original_value: &Number, path: Vec<String>) -> CompilationResult {
        Ok(Box::new(ConstNumberValidator {
            path,
            original_value: original_value.clone(),
            value: original_value
                .as_f64()
                .expect("A JSON number will always be representable as f64"),
        }))
    }
}

impl Validate for ConstNumberValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::constant_number(
                self.path.clone(),
                instance,
                &self.original_value,
            ))
        }
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Number(item) = instance {
            (self.value - item.as_f64().expect("Always representable as f64")).abs() < EPSILON
        } else {
            false
        }
    }
}

impl ToString for ConstNumberValidator {
    fn to_string(&self) -> String {
        format!("const: {}", self.original_value)
    }
}

pub(crate) struct ConstObjectValidator {
    value: Map<String, Value>,
    path: Vec<String>,
}

impl ConstObjectValidator {
    #[inline]
    pub(crate) fn compile(value: &Map<String, Value>, path: Vec<String>) -> CompilationResult {
        Ok(Box::new(ConstObjectValidator {
            path,
            value: value.clone(),
        }))
    }
}

impl Validate for ConstObjectValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::constant_object(
                self.path.clone(),
                instance,
                &self.value,
            ))
        }
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Object(item) = instance {
            helpers::equal_objects(&self.value, item)
        } else {
            false
        }
    }
}

impl ToString for ConstObjectValidator {
    fn to_string(&self) -> String {
        format!(
            "const: {{{}}}",
            self.value
                .iter()
                .map(|(key, value)| format!(r#""{}":{}"#, key, value))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

pub(crate) struct ConstStringValidator {
    value: String,
    path: Vec<String>,
}

impl ConstStringValidator {
    #[inline]
    pub(crate) fn compile(value: &str, path: Vec<String>) -> CompilationResult {
        Ok(Box::new(ConstStringValidator {
            path,
            value: value.to_string(),
        }))
    }
}

impl Validate for ConstStringValidator {
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::constant_string(
                self.path.clone(),
                instance,
                &self.value,
            ))
        }
    }

    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::String(item) = instance {
            &self.value == item
        } else {
            false
        }
    }
}

impl ToString for ConstStringValidator {
    fn to_string(&self) -> String {
        format!("const: {}", self.value)
    }
}

#[inline]
pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    context: &CompilationContext,
) -> Option<CompilationResult> {
    match schema {
        Value::Array(items) => Some(ConstArrayValidator::compile(
            items,
            context.curr_path.clone(),
        )),
        Value::Bool(item) => Some(ConstBooleanValidator::compile(
            *item,
            context.curr_path.clone(),
        )),
        Value::Null => Some(ConstNullValidator::compile(context.curr_path.clone())),
        Value::Number(item) => Some(ConstNumberValidator::compile(
            item,
            context.curr_path.clone(),
        )),
        Value::Object(map) => Some(ConstObjectValidator::compile(
            map,
            context.curr_path.clone(),
        )),
        Value::String(string) => Some(ConstStringValidator::compile(
            string,
            context.curr_path.clone(),
        )),
    }
}
