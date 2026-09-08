use patterns_macros::{because, source};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use worker::{Headers, Request, Response, Result};

pub struct WebSecurity;
source!(
    WebSecurity,
    "the response headers browsers honour for a site's security: the content security policy, framing, type sniffing, referrer, cross-origin isolation, feature permissions and transport policies"
);

pub struct HttpSemantics;
source!(
    HttpSemantics,
    "the HTTP status codes, the conditional request headers and the cache directives, as the protocol defines them"
);

pub const SECURITY_HEADERS: &[(&str, &str)] = &[
    (
        "content-security-policy",
        "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self' 'unsafe-inline'; img-src 'self' data:; connect-src 'self'; font-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'self'",
    ),
    ("x-frame-options", "SAMEORIGIN"),
    ("x-content-type-options", "nosniff"),
    ("referrer-policy", "no-referrer"),
    ("cross-origin-opener-policy", "same-origin"),
    ("cross-origin-resource-policy", "same-origin"),
    ("permissions-policy", "camera=(), microphone=(), geolocation=()"),
];
because!(
    SECURITY_HEADERS,
    WebSecurity,
    "the headers every answer carries: a policy that lets the page load only its own scripts, styles, images and wasm, no framing by others, no type sniffing, no referrer, an isolated origin and no device access"
);

pub struct CloudflareHeaders;
source!(
    CloudflareHeaders,
    "the request headers Cloudflare adds in front of a Worker, of which the connecting address is the one a rate limit keys on"
);

const TRANSPORT_HEADER: &str = "strict-transport-security";
because!(TRANSPORT_HEADER, WebSecurity, "the header that tells a browser to reach this host only over a secure transport");

const FETCH_SITE_HEADER: &str = "sec-fetch-site";
because!(FETCH_SITE_HEADER, WebSecurity, "the header a browser sets to say which site a request comes from, which a same-site API reads to refuse other sites");

const CROSS_SITE: &str = "cross-site";
because!(CROSS_SITE, WebSecurity, "the fetch-site value that means another site started the request");

const CONNECTING_ADDRESS_HEADER: &str = "cf-connecting-ip";
because!(CONNECTING_ADDRESS_HEADER, CloudflareHeaders, "the header carrying the address the request came from, the key one visitor's requests share");

pub const FORBIDDEN_ANSWER: (&str, u16) = ("{\"error\":\"Forbidden\"}", 403);
because!(FORBIDDEN_ANSWER, HttpSemantics, "the body and status of an answer to a request another site started, which this API never serves");

pub const TOO_MANY_REQUESTS: u16 = 429;
because!(TOO_MANY_REQUESTS, HttpSemantics, "the status of a request refused because its visitor asked too often");

pub const JSON: &str = "application/json";
because!(JSON, HttpSemantics, "the media type every page answer and every refusal carries");

pub const NOT_FOUND_ANSWER: (&str, u16) = ("{\"error\":\"NotFound\"}", 404);
because!(NOT_FOUND_ANSWER, HttpSemantics, "the body and status of an answer to a path that names nothing, JSON so every caller parses one shape");

pub const FAILED_ANSWER: (&str, u16) = ("{\"error\":\"Failed\"}", 500);
because!(FAILED_ANSWER, HttpSemantics, "the body and status of an answer to a request that failed inside, with the detail kept in the log where a caller cannot read it");

const TAG_SEPARATOR: char = ',';
because!(TAG_SEPARATOR, HttpSemantics, "the mark between the several tags a browser may present in one conditional header");

const METHOD_NOT_ALLOWED: u16 = 405;
because!(METHOD_NOT_ALLOWED, HttpSemantics, "the status of a method a route does not take");

const UNAUTHORIZED: u16 = 401;
because!(UNAUTHORIZED, HttpSemantics, "the status of a request without the secret a route demands");

const NOT_MODIFIED: u16 = 304;
because!(NOT_MODIFIED, HttpSemantics, "the status that tells a browser the version it holds is still the version");

const BEARER: &str = "Bearer ";
because!(BEARER, HttpSemantics, "the scheme prefix before a secret in an authorization header");

static PAGE_BODIES: OnceLock<Mutex<(HashMap<String, String>, Vec<String>)>> = OnceLock::new();
because!(
    PAGE_BODIES,
    HttpSemantics,
    "the answers one isolate remembers by key beside the order they arrived in, so a repeated request is served from memory and the oldest is forgotten first"
);

pub fn secured(response: Response, transport_seconds: u64) -> Response {
    let headers = response.headers().clone();
    for (name, value) in SECURITY_HEADERS {
        let _ = headers.set(name, value);
    }
    let _ = headers.set(TRANSPORT_HEADER, &format!("max-age={transport_seconds}; includeSubDomains"));
    response.with_headers(headers)
}
because!(secured, "an answer with every security header set and a transport promise for the seconds given, applied to every answer the same way");

pub fn json_text(text: String, status: u16) -> Result<Response> {
    let headers = Headers::new();
    headers.set("content-type", JSON)?;
    Ok(Response::ok(text)?.with_status(status).with_headers(headers))
}
because!(json_text, "a JSON body under a status, the one way every answer is built");

pub fn answer(kind: (&str, u16)) -> Result<Response> {
    json_text(String::from(kind.0), kind.1)
}
because!(answer, "one of the fixed answers, a path that names nothing or a request that failed inside, built from its body and status pair");

pub const COMPARED_BYTES: usize = 256;
because!(
    COMPARED_BYTES,
    WebSecurity,
    "the bytes every secret comparison walks whatever it is given, a fixed count far above any secret a deployment sets, so neither the answer nor the work spent depends on either length, and a secret longer than the walk is refused on length rather than read short"
);

fn same(presented: &str, expected: &str) -> bool {
    let a = presented.as_bytes();
    let b = expected.as_bytes();
    let mut differ = a.len() ^ b.len();
    differ |= usize::from(a.len() > COMPARED_BYTES);
    differ |= usize::from(b.len() > COMPARED_BYTES);
    for i in 0..COMPARED_BYTES {
        let x = a.get(i).copied().unwrap_or_default();
        let y = b.get(i).copied().unwrap_or_default();
        differ |= (x ^ y) as usize;
    }
    differ == 0
}

pub fn bearer_refusal(request: &Request, expected: &str) -> Result<Option<Response>> {
    if request.method() != worker::Method::Post {
        let headers = Headers::new();
        headers.set("allow", "POST")?;
        return Ok(Some(Response::error("method not allowed", METHOD_NOT_ALLOWED)?.with_headers(headers)));
    }
    let header = request.headers().get("authorization")?.unwrap_or_default();
    let presented = header.strip_prefix(BEARER).unwrap_or_default();
    if expected.is_empty() || presented.is_empty() || !same(presented, expected) {
        return Ok(Some(Response::error("unauthorized", UNAUTHORIZED)?));
    }
    Ok(None)
}
because!(
    bearer_refusal,
    "the refusal of an admin request, or nothing when it may proceed: it must be a post carrying the secret, compared over a fixed count of bytes so that neither where the two differ nor how long either one is changes the time an answer takes"
);

fn bodies() -> &'static Mutex<(HashMap<String, String>, Vec<String>)> {
    PAGE_BODIES.get_or_init(|| Mutex::new((HashMap::new(), Vec::new())))
}

pub fn remembered(key: &str) -> Option<String> {
    bodies().lock().ok().and_then(|held| held.0.get(key).cloned())
}
because!(remembered, "the answer this isolate remembers under a key, or nothing");

pub fn remember(key: &str, body: &str, max_entries: usize) {
    if let Ok(mut held) = bodies().lock() {
        if held.0.insert(String::from(key), String::from(body)).is_none() {
            held.1.push(String::from(key));
        }
        while held.1.len() > max_entries {
            let oldest = held.1.remove(0);
            held.0.remove(&oldest);
        }
    }
}
because!(remember, "an answer kept under its key, the oldest forgotten once more than the allowed entries are held");

fn cache_control(browser_seconds: u64, edge_seconds: u64) -> String {
    format!("public, max-age={browser_seconds}, s-maxage={edge_seconds}")
}

fn versioned(headers: &Headers, version: &str, browser_seconds: u64, edge_seconds: u64, state: &str) -> Result<()> {
    headers.set("cache-control", &cache_control(browser_seconds, edge_seconds))?;
    headers.set("etag", &format!("\"{version}\""))?;
    headers.set("x-data-version", version)?;
    headers.set("x-cache", state)
}

pub fn stamped(body: &str, version: &str, state: &str, browser_seconds: u64, edge_seconds: u64) -> Result<Response> {
    let headers = Headers::new();
    headers.set("content-type", JSON)?;
    versioned(&headers, version, browser_seconds, edge_seconds, state)?;
    Ok(Response::ok(body)?.with_headers(headers))
}
because!(stamped, "a body under its version as tag, the ages a browser and the edge may keep it, and where it was found");

pub fn not_modified(version: &str, browser_seconds: u64, edge_seconds: u64) -> Result<Response> {
    let headers = Headers::new();
    versioned(&headers, version, browser_seconds, edge_seconds, "browser")?;
    Ok(Response::empty()?.with_status(NOT_MODIFIED).with_headers(headers))
}
because!(not_modified, "the empty answer that tells a browser its copy is still the version");

pub fn held_forever(response: Response, seconds: u64) -> Response {
    let headers = response.headers().clone();
    let _ = headers.set("cache-control", &format!("public, max-age={seconds}, immutable"));
    response.with_headers(headers)
}
because!(held_forever, "an answer a browser may keep for the seconds given and never ask about again, which only a file whose name carries a digest of its content may claim");

pub fn etag_matches(request: &Request, version: &str) -> bool {
    let current = format!("\"{version}\"");
    request
        .headers()
        .get("if-none-match")
        .ok()
        .flatten()
        .map(|text| text.split(TAG_SEPARATOR).any(|entry| entry.trim() == current))
        .unwrap_or_default()
}
because!(etag_matches, "whether the browser already holds this version, read from the tags it presents");

pub fn refusal(status: u16, body: &str, retry_after: Option<u64>) -> Result<Response> {
    let headers = Headers::new();
    headers.set("content-type", JSON)?;
    headers.set("cache-control", "no-store")?;
    if let Some(seconds) = retry_after {
        headers.set("retry-after", &seconds.to_string())?;
    }
    Ok(Response::ok(body)?.with_status(status).with_headers(headers))
}
because!(refusal, "an answer that refuses, never cached, with the seconds to wait when the refusal is a rate limit");

pub fn cross_site(request: &Request) -> bool {
    request.headers().get(FETCH_SITE_HEADER).ok().flatten().as_deref() == Some(CROSS_SITE)
}
because!(cross_site, "whether a browser says another site started this request, the one signal a same-site API refuses on");

pub fn client_key(request: &Request, fallback: &str) -> String {
    request.headers().get(CONNECTING_ADDRESS_HEADER).ok().flatten().unwrap_or_else(|| String::from(fallback))
}
because!(client_key, "the address a request came from as the key its visitor's requests share, or a fallback when the platform did not say");

pub fn query_pairs(request: &Request) -> Vec<(String, String)> {
    request
        .url()
        .map(|url| url.query_pairs().map(|(k, v)| (k.into_owned(), v.into_owned())).collect())
        .unwrap_or_default()
}
because!(query_pairs, "every query parameter of a request as a pair, in the order given");

pub fn query(request: &Request, name: &str) -> Option<String> {
    query_pairs(request).into_iter().find(|(k, _)| k == name).map(|(_, v)| v)
}
because!(query, "one query parameter of a request by name, or nothing");
