use crate::json::{columns_of, field_of, js_of};
use patterns::{Field, Fielded};
use patterns_macros::{because, source};
use serde_json::Value;
use wasm_bindgen::JsValue;
use worker::{D1Database, D1PreparedStatement, Result};

pub struct SqliteWords;
source!(
    SqliteWords,
    "the words SQLite reserves, of which a record's field may carry one as its name and must be stored under another"
);

pub const RESERVED_COLUMNS: &[&str] = &["index", "transaction"];
because!(
    RESERVED_COLUMNS,
    SqliteWords,
    "the field names a record in this library's projects has carried that SQLite reserves, each stored with a trailing mark and read back without it"
);

const MARK: char = '_';
because!(MARK, SqliteWords, "the mark appended to a reserved word to make it a column name, one a field name never ends with");

pub fn column_name(field: &str) -> String {
    match RESERVED_COLUMNS.contains(&field) {
        true => format!("{field}{MARK}"),
        false => String::from(field),
    }
}
because!(column_name, "the column a field is stored under: the field's own name unless SQLite reserves it, then the name with the mark appended");

pub fn field_name(column: &str) -> String {
    column.strip_suffix(MARK).map(String::from).unwrap_or_else(|| String::from(column))
}
because!(field_name, "the field a column carries, the reverse of column_name");

pub async fn create_schema(db: &D1Database, statements: &[&str]) -> Result<()> {
    for statement in statements {
        db.exec(statement).await?;
    }
    Ok(())
}
because!(create_schema, "every statement of a schema run in order, each written so a second run changes nothing");

pub fn statement(db: &D1Database, sql: &str, params: &[Field]) -> Result<D1PreparedStatement> {
    let values: Vec<JsValue> = params.iter().map(js_of).collect();
    db.prepare(sql).bind(&values)
}
because!(statement, "one prepared statement with every parameter bound as the value its field becomes");

pub fn upsert_statements<T: Fielded>(db: &D1Database, table: &str, fixed: &[(&str, Field)], rows: &[T]) -> Result<Vec<D1PreparedStatement>> {
    let mut out = Vec::new();
    for row in rows {
        let mut columns: Vec<(String, Field)> = fixed.iter().map(|(k, v)| (String::from(*k), v.clone())).collect();
        columns.extend(columns_of(row));
        let names: Vec<String> = columns.iter().map(|(name, _)| column_name(name)).collect();
        let marks: Vec<String> = columns.iter().map(|_| String::from(PLACEHOLDER)).collect();
        let values: Vec<Field> = columns.into_iter().map(|(_, field)| field).collect();
        let sql = format!("insert or replace into {table} ({}) values ({})", names.join(SEPARATOR), marks.join(SEPARATOR));
        out.push(statement(db, &sql, &values)?);
    }
    Ok(out)
}
because!(upsert_statements, "an insert or replace of every row, each row's columns from its record beside the fixed columns every row shares, so a store writes a record without naming its fields");

const PLACEHOLDER: &str = "?";
because!(PLACEHOLDER, SqliteWords, "the mark SQLite reads as the next bound parameter");

const SEPARATOR: &str = ", ";
because!(SEPARATOR, SqliteWords, "the text between two names or two marks in a statement's lists");

pub async fn run_all(db: &D1Database, statements: Vec<D1PreparedStatement>, batch_size: usize) -> Result<()> {
    let mut pending = statements;
    while !pending.is_empty() {
        let rest = pending.split_off(pending.len().min(batch_size));
        db.batch(pending).await?;
        pending = rest;
    }
    Ok(())
}
because!(run_all, "every statement run in batches of the size the platform allows, in order, so a long write is as few round trips as it can be");

pub async fn upsert<T: Fielded>(db: &D1Database, table: &str, fixed: &[(&str, Field)], rows: &[T], batch_size: usize) -> Result<()> {
    run_all(db, upsert_statements(db, table, fixed, rows)?, batch_size).await
}
because!(upsert, "every row written in batches, the two steps of a store's write in one call");

pub fn parsed<T: Fielded>(raw: &[Value]) -> Vec<T> {
    raw.iter()
        .filter_map(|row| {
            let table = match field_of(row) {
                Field::Table(rows) => rows.into_iter().map(|(k, v)| (field_name(&k), v)).collect(),
                other => return T::refielded(&other),
            };
            T::refielded(&Field::Table(table))
        })
        .collect()
}
because!(parsed, "result rows as records, each column read back through the field name it was stored under, with rows that do not fit the record left out");

pub async fn rows<T: Fielded>(db: &D1Database, sql: &str, params: &[Field]) -> Result<Vec<T>> {
    let result = statement(db, sql, params)?.all().await?;
    let raw: Vec<Value> = result.results()?;
    Ok(parsed(&raw))
}
because!(rows, "every row of a query as the record type");

pub async fn one<T: Fielded>(db: &D1Database, sql: &str, params: &[Field]) -> Result<Option<T>> {
    let mut found = rows::<T>(db, sql, params).await?;
    found.truncate(1);
    Ok(found.pop())
}
because!(one, "the first row of a query as the record type, or nothing");

pub async fn scalars<T: Fielded>(db: &D1Database, sql: &str, params: &[Field]) -> Result<Vec<T>> {
    let result = statement(db, sql, params)?.all().await?;
    let raw: Vec<Value> = result.results()?;
    Ok(raw
        .iter()
        .filter_map(|row| match row {
            Value::Object(map) => map.values().next().and_then(|v| T::refielded(&field_of(v))),
            other => T::refielded(&field_of(other)),
        })
        .collect())
}
because!(scalars, "the first column of every row as a plain value, the shape of a count or a maximum");

pub async fn scalar<T: Fielded>(db: &D1Database, sql: &str, params: &[Field]) -> Result<Option<T>> {
    Ok(scalars::<T>(db, sql, params).await?.into_iter().next())
}
because!(scalar, "the first column of the first row as a plain value, or nothing");

pub async fn execute(db: &D1Database, sql: &str, params: &[Field]) -> Result<()> {
    statement(db, sql, params)?.run().await?;
    Ok(())
}
because!(execute, "one statement run for its effect, its rows discarded");

pub fn text(value: &str) -> Field {
    Field::Text(String::from(value))
}
because!(text, "a text as the field a statement binds");

pub fn whole(value: u64) -> Field {
    Field::Whole(value as i64)
}
because!(whole, "a count as the field a statement binds");
