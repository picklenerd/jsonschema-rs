use crate::{
    compilation::{context::CompilationContext, JSONSchema},
    error::{error, no_error, CompilationError, ErrorIterator, ValidationError},
    keywords::CompilationResult,
    primitive_type::{PrimitiveType, PrimitiveTypesBitMap},
    validator::Validate,
};
use serde_json::{Map, Number, Value};
use std::convert::TryFrom;

pub(crate) struct MultipleTypesValidator {
    types: PrimitiveTypesBitMap,
    path: Vec<String>,
}

impl MultipleTypesValidator {
    #[inline]
    pub(crate) fn compile(items: &[Value], path: Vec<String>) -> CompilationResult {
        let mut types = PrimitiveTypesBitMap::new();
        for item in items {
            match item {
                Value::String(string) => {
                    if let Ok(primitive_type) = PrimitiveType::try_from(string.as_str()) {
                        types |= primitive_type;
                    } else {
                        return Err(CompilationError::SchemaError);
                    }
                }
                _ => return Err(CompilationError::SchemaError),
            }
        }
        Ok(Box::new(MultipleTypesValidator { types, path }))
    }
}

impl Validate for MultipleTypesValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        match instance {
            Value::Array(_) => self.types.contains_type(PrimitiveType::Array),
            Value::Bool(_) => self.types.contains_type(PrimitiveType::Boolean),
            Value::Null => self.types.contains_type(PrimitiveType::Null),
            Value::Number(num) => {
                self.types.contains_type(PrimitiveType::Number)
                    || (self.types.contains_type(PrimitiveType::Integer) && is_integer(num))
            }
            Value::Object(_) => self.types.contains_type(PrimitiveType::Object),
            Value::String(_) => self.types.contains_type(PrimitiveType::String),
        }
    }
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::multiple_type_error(
                self.path.clone(),
                instance,
                self.types,
            ))
        }
    }
}

impl ToString for MultipleTypesValidator {
    fn to_string(&self) -> String {
        format!(
            "type: [{}]",
            self.types
                .into_iter()
                .map(|type_| format!("{}", type_))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

pub(crate) struct NullTypeValidator {
    path: Vec<String>,
}

impl NullTypeValidator {
    #[inline]
    pub(crate) fn compile(path: Vec<String>) -> CompilationResult {
        Ok(Box::new(NullTypeValidator { path }))
    }
}

impl Validate for NullTypeValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_null()
    }
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                self.path.clone(),
                instance,
                PrimitiveType::Null,
            ))
        }
    }
}

impl ToString for NullTypeValidator {
    fn to_string(&self) -> String {
        "type: null".to_string()
    }
}

pub(crate) struct BooleanTypeValidator {
    path: Vec<String>,
}

impl BooleanTypeValidator {
    #[inline]
    pub(crate) fn compile(path: Vec<String>) -> CompilationResult {
        Ok(Box::new(BooleanTypeValidator { path }))
    }
}

impl Validate for BooleanTypeValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_boolean()
    }
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                self.path.clone(),
                instance,
                PrimitiveType::Boolean,
            ))
        }
    }
}

impl ToString for BooleanTypeValidator {
    fn to_string(&self) -> String {
        "type: boolean".to_string()
    }
}

pub(crate) struct StringTypeValidator {
    path: Vec<String>,
}

impl StringTypeValidator {
    #[inline]
    pub(crate) fn compile(path: Vec<String>) -> CompilationResult {
        Ok(Box::new(StringTypeValidator { path }))
    }
}

impl Validate for StringTypeValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_string()
    }

    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                self.path.clone(),
                instance,
                PrimitiveType::String,
            ))
        }
    }
}
impl ToString for StringTypeValidator {
    fn to_string(&self) -> String {
        "type: string".to_string()
    }
}

pub(crate) struct ArrayTypeValidator {
    path: Vec<String>,
}

impl ArrayTypeValidator {
    #[inline]
    pub(crate) fn compile(path: Vec<String>) -> CompilationResult {
        Ok(Box::new(ArrayTypeValidator { path }))
    }
}

impl Validate for ArrayTypeValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_array()
    }

    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                self.path.clone(),
                instance,
                PrimitiveType::Array,
            ))
        }
    }
}

impl ToString for ArrayTypeValidator {
    fn to_string(&self) -> String {
        "type: array".to_string()
    }
}

pub(crate) struct ObjectTypeValidator {
    path: Vec<String>,
}

impl ObjectTypeValidator {
    #[inline]
    pub(crate) fn compile(path: Vec<String>) -> CompilationResult {
        Ok(Box::new(ObjectTypeValidator { path }))
    }
}

impl Validate for ObjectTypeValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_object()
    }
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                self.path.clone(),
                instance,
                PrimitiveType::Object,
            ))
        }
    }
}

impl ToString for ObjectTypeValidator {
    fn to_string(&self) -> String {
        "type: object".to_string()
    }
}

pub(crate) struct NumberTypeValidator {
    path: Vec<String>,
}

impl NumberTypeValidator {
    #[inline]
    pub(crate) fn compile(path: Vec<String>) -> CompilationResult {
        Ok(Box::new(NumberTypeValidator { path }))
    }
}

impl Validate for NumberTypeValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        instance.is_number()
    }
    fn validate<'a>(&self, config: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(config, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                self.path.clone(),
                instance,
                PrimitiveType::Number,
            ))
        }
    }
}
impl ToString for NumberTypeValidator {
    fn to_string(&self) -> String {
        "type: number".to_string()
    }
}
pub(crate) struct IntegerTypeValidator {
    path: Vec<String>,
}

impl IntegerTypeValidator {
    #[inline]
    pub(crate) fn compile(path: Vec<String>) -> CompilationResult {
        Ok(Box::new(IntegerTypeValidator { path }))
    }
}

impl Validate for IntegerTypeValidator {
    fn is_valid(&self, _: &JSONSchema, instance: &Value) -> bool {
        if let Value::Number(num) = instance {
            is_integer(num)
        } else {
            false
        }
    }
    fn validate<'a>(&self, schema: &'a JSONSchema, instance: &'a Value) -> ErrorIterator<'a> {
        if self.is_valid(schema, instance) {
            no_error()
        } else {
            error(ValidationError::single_type_error(
                self.path.clone(),
                instance,
                PrimitiveType::Integer,
            ))
        }
    }
}

impl ToString for IntegerTypeValidator {
    fn to_string(&self) -> String {
        "type: integer".to_string()
    }
}

fn is_integer(num: &Number) -> bool {
    num.is_u64() || num.is_i64() || num.as_f64().expect("Always valid").fract() == 0.
}

#[inline]
pub(crate) fn compile(
    _: &Map<String, Value>,
    schema: &Value,
    context: &CompilationContext,
) -> Option<CompilationResult> {
    let path = context.curr_path.clone();
    match schema {
        Value::String(item) => compile_single_type(item.as_str(), path),
        Value::Array(items) => {
            if items.len() == 1 {
                if let Some(Value::String(item)) = items.iter().next() {
                    compile_single_type(item.as_str(), path)
                } else {
                    Some(Err(CompilationError::SchemaError))
                }
            } else {
                Some(MultipleTypesValidator::compile(items, path))
            }
        }
        _ => Some(Err(CompilationError::SchemaError)),
    }
}

fn compile_single_type(item: &str, path: Vec<String>) -> Option<CompilationResult> {
    match PrimitiveType::try_from(item) {
        Ok(PrimitiveType::Array) => Some(ArrayTypeValidator::compile(path)),
        Ok(PrimitiveType::Boolean) => Some(BooleanTypeValidator::compile(path)),
        Ok(PrimitiveType::Integer) => Some(IntegerTypeValidator::compile(path)),
        Ok(PrimitiveType::Null) => Some(NullTypeValidator::compile(path)),
        Ok(PrimitiveType::Number) => Some(NumberTypeValidator::compile(path)),
        Ok(PrimitiveType::Object) => Some(ObjectTypeValidator::compile(path)),
        Ok(PrimitiveType::String) => Some(StringTypeValidator::compile(path)),
        Err(()) => Some(Err(CompilationError::SchemaError)),
    }
}
