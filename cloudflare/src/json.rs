use patterns::{camel_of, none, unit, Field, Fielded};
use patterns_macros::{because, source};
use serde_json::{Map, Number, Value};
use wasm_bindgen::JsValue;

pub struct WireShapes;
source!(
    WireShapes,
    "the shapes a JSON wire and a SQLite binding accept: null, booleans, numbers, strings, arrays and objects on the wire, and null, numbers and strings as bound values"
);

const SCALAR_COLUMN: &str = "value";
because!(SCALAR_COLUMN, WireShapes, "the column name a record that is one scalar is stored under, since a store needs a name for every column");

pub fn json_of(field: &Field) -> Value {
    match field {
        Field::Nothing => Value::Null,
        Field::Flag(flag) => Value::Bool(*flag),
        Field::Whole(value) => Value::Number(Number::from(*value)),
        Field::Real(value) => Number::from_f64(*value).map(Value::Number).unwrap_or(Value::Null),
        Field::Text(text) => Value::String(text.clone()),
        Field::List(items) => Value::Array(items.iter().map(json_of).collect()),
        Field::Table(rows) => {
            let mut out = Map::new();
            for (key, value) in rows {
                if !matches!(value, Field::Nothing) {
                    out.insert(camel_of(key), json_of(value));
                }
            }
            Value::Object(out)
        }
    }
}
because!(json_of, "a field as JSON with every table key in the wire's naming and every absent value left out, so an optional part of a record is simply not there");

pub fn field_of(value: &Value) -> Field {
    match value {
        Value::Null => Field::Nothing,
        Value::Bool(flag) => Field::Flag(*flag),
        Value::Number(number) => match number.as_i64() {
            Some(whole) => Field::Whole(whole),
            None => Field::Real(number.as_f64().unwrap_or(none())),
        },
        Value::String(text) => Field::Text(text.clone()),
        Value::Array(items) => Field::List(items.iter().map(field_of).collect()),
        Value::Object(rows) => Field::Table(rows.iter().map(|(k, v)| (k.clone(), field_of(v))).collect()),
    }
}
because!(field_of, "JSON as a field with its keys kept as given, the reverse of json_of for a wire or a store that already writes a program's names");

pub fn json_text_of<T: Fielded>(value: &T) -> String {
    json_of(&value.field()).to_string()
}
because!(json_text_of, "a record as one JSON text, the body a wire sends");

pub fn restored_json<T: Fielded>(value: &Value) -> Option<T> {
    T::refielded(&field_of(value))
}
because!(restored_json, "a record read back from JSON, or nothing when the JSON does not fit the record");

pub fn js_of(field: &Field) -> JsValue {
    match field {
        Field::Nothing => JsValue::NULL,
        Field::Flag(flag) => JsValue::from_f64(match flag {
            true => unit(),
            false => none(),
        }),
        Field::Whole(value) => JsValue::from_f64(*value as f64),
        Field::Real(value) => JsValue::from_f64(*value),
        Field::Text(text) => JsValue::from_str(text),
        Field::List(_) | Field::Table(_) => JsValue::from_str(&json_of(field).to_string()),
    }
}
because!(js_of, "a field as the value a statement binds: a flag as one or zero, a number as a number, a text as itself and a nested shape as its JSON text");

pub fn columns_of<T: Fielded>(value: &T) -> Vec<(String, Field)> {
    match value.field() {
        Field::Table(rows) => rows,
        other => vec![(String::from(SCALAR_COLUMN), other)],
    }
}
because!(columns_of, "the named columns of a record in declaration order, a lone scalar under the one column name a store gives it");
