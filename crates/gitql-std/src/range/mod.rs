use gitql_core::signature::Function;
use gitql_core::signature::Signature;
use gitql_core::types::DataType;
use gitql_core::value::Value;

use std::collections::HashMap;

#[inline(always)]
pub fn register_std_range_functions(map: &mut HashMap<&'static str, Function>) {
    map.insert("int4range", int4range);
    map.insert("daterange", daterange);
    map.insert("tsrange", tsrange);
    map.insert("isempty", isempty);
}

#[inline(always)]
pub fn register_std_range_function_signatures(map: &mut HashMap<&'static str, Signature>) {
    map.insert(
        "int4range",
        Signature {
            parameters: vec![DataType::Integer, DataType::Integer],
            return_type: DataType::Range(Box::new(DataType::Integer)),
        },
    );
    map.insert(
        "daterange",
        Signature {
            parameters: vec![DataType::Date, DataType::Date],
            return_type: DataType::Range(Box::new(DataType::Date)),
        },
    );
    map.insert(
        "tsrange",
        Signature {
            parameters: vec![DataType::DateTime, DataType::DateTime],
            return_type: DataType::Range(Box::new(DataType::DateTime)),
        },
    );
    map.insert(
        "isempty",
        Signature {
            parameters: vec![DataType::Range(Box::new(DataType::Any))],
            return_type: DataType::Boolean,
        },
    );
}

pub fn int4range(inputs: &[Value]) -> Value {
    Value::Range(
        DataType::Integer,
        Box::new(inputs[0].clone()),
        Box::new(inputs[1].clone()),
    )
}

pub fn daterange(inputs: &[Value]) -> Value {
    Value::Range(
        DataType::Date,
        Box::new(inputs[0].clone()),
        Box::new(inputs[1].clone()),
    )
}

pub fn tsrange(inputs: &[Value]) -> Value {
    Value::Range(
        DataType::DateTime,
        Box::new(inputs[0].clone()),
        Box::new(inputs[1].clone()),
    )
}

pub fn isempty(inputs: &[Value]) -> Value {
    let range = inputs[0].as_range();
    Value::Boolean(range.0.equals(&range.1))
}
