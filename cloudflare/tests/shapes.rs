use premise_cloudflare::{column_name, columns_of, field_name, field_of, json_of, json_text_of, parsed, restored_json};
use patterns::{Field, Fielded, Record};
use serde_json::json;

#[derive(Record, Clone, Debug, PartialEq)]
struct Row {
    day_timestamp_unix_sec: u64,
    label: String,
    price_usd: Option<f64>,
    index: u64,
}

#[test]
fn a_field_becomes_wire_json_with_camel_keys_and_no_absent_values() {
    let row = Row { day_timestamp_unix_sec: 86_400, label: String::from("FLUX"), price_usd: None, index: 3 };
    assert_eq!(json_of(&row.field()), json!({ "dayTimestampUnixSec": 86_400, "label": "FLUX", "index": 3 }));
    assert_eq!(json_text_of(&row), "{\"dayTimestampUnixSec\":86400,\"index\":3,\"label\":\"FLUX\"}");
}

#[test]
fn json_reads_back_as_a_field_with_its_keys_as_given() {
    let value = json!({ "day": 1, "usd": 2.5, "name": "x", "flag": true, "gone": null, "list": [1, 2] });
    let field = field_of(&value);
    match &field {
        Field::Table(rows) => {
            assert_eq!(rows.iter().find(|(k, _)| k == "usd").map(|(_, v)| v.clone()), Some(Field::Real(2.5)));
            assert_eq!(rows.iter().find(|(k, _)| k == "gone").map(|(_, v)| v.clone()), Some(Field::Nothing));
            assert_eq!(rows.iter().find(|(k, _)| k == "list").map(|(_, v)| v.clone()), Some(Field::List(vec![Field::Whole(1), Field::Whole(2)])));
        }
        other => panic!("an object is a table, not {:?}", other),
    }
    assert_eq!(json_of(&field_of(&json!(7))), json!(7));
}

#[test]
fn a_stored_row_comes_back_through_its_column_names() {
    assert_eq!(column_name("index"), "index_");
    assert_eq!(column_name("day"), "day");
    assert_eq!(field_name("index_"), "index");
    assert_eq!(field_name("transaction_"), "transaction");
    let raw = vec![json!({ "day_timestamp_unix_sec": 86_400, "label": "FLUX", "price_usd": null, "index_": 3 })];
    let rows: Vec<Row> = parsed(&raw);
    assert_eq!(rows, vec![Row { day_timestamp_unix_sec: 86_400, label: String::from("FLUX"), price_usd: None, index: 3 }]);
    assert_eq!(restored_json::<u64>(&json!(9)), Some(9));
    assert_eq!(columns_of(&5u64), vec![(String::from("value"), Field::Whole(5))]);
}
