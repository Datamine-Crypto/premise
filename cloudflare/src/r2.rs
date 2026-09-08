use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use patterns_macros::because;
use std::io::{Read, Write};
use worker::{Bucket, Result};

fn fault<E: std::fmt::Display>(error: E) -> worker::Error {
    worker::Error::RustError(error.to_string())
}

pub fn gzipped(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(bytes).map_err(fault)?;
    encoder.finish().map_err(fault)
}
because!(gzipped, "bytes compressed the way an object store and a browser both read");

pub fn gunzipped(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    GzDecoder::new(bytes).read_to_end(&mut out).map_err(fault)?;
    Ok(out)
}
because!(gunzipped, "the bytes a compressed object held, the reverse of gzipped");

pub async fn put_gzip(bucket: &Bucket, key: &str, bytes: &[u8]) -> Result<()> {
    bucket.put(key, gzipped(bytes)?).execute().await?;
    Ok(())
}
because!(put_gzip, "bytes stored compressed under a key");

pub async fn get_gzip(bucket: &Bucket, key: &str) -> Result<Option<Vec<u8>>> {
    let object = match bucket.get(key).execute().await? {
        Some(object) => object,
        None => return Ok(None),
    };
    let body = match object.body() {
        Some(body) => body,
        None => return Ok(None),
    };
    Ok(Some(gunzipped(&body.bytes().await?)?))
}
because!(get_gzip, "the bytes stored compressed under a key, or nothing when no object or no body is there");

pub async fn keys_under(bucket: &Bucket, prefix: &str) -> Result<Vec<String>> {
    let mut out = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let mut listing = bucket.list().prefix(prefix);
        if let Some(c) = &cursor {
            listing = listing.cursor(c.clone());
        }
        let objects = listing.execute().await?;
        out.extend(objects.objects().iter().map(|object| object.key()));
        cursor = match objects.truncated() {
            true => objects.cursor(),
            false => None,
        };
        if cursor.is_none() {
            return Ok(out);
        }
    }
}
because!(keys_under, "every key under a prefix, read page after page until the listing ends");

pub async fn copy_missing(from: &Bucket, to: &Bucket, prefix: &str, cursor: Option<String>, limit: u32) -> Result<(usize, Option<String>)> {
    let mut listing = from.list().prefix(prefix).limit(limit);
    if let Some(c) = cursor {
        listing = listing.cursor(c);
    }
    let objects = listing.execute().await?;
    let mut copied = 0usize;
    for object in objects.objects() {
        let key = object.key();
        if to.head(key.clone()).await?.is_some() {
            continue;
        }
        if let Some(found) = from.get(key.clone()).execute().await? {
            if let Some(body) = found.body() {
                to.put(key, body.bytes().await?).execute().await?;
                copied += 1;
            }
        }
    }
    let next = match objects.truncated() {
        true => objects.cursor(),
        false => None,
    };
    Ok((copied, next))
}
because!(copy_missing, "one page of objects under a prefix copied from one bucket to another, skipping keys the target already holds, with the cursor of the next page");
