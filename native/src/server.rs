//! 本地 HTTP 服务：静态文件 + API。路由与 JSON 字段与 Python 版 server.py 一一对应。

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use serde_json::{json, Value};
use tiny_http::{Header, Response, Server};

use crate::autostart;
use crate::exporter;
use crate::hooks;
use crate::model::{now_text, range_bounds};
use crate::settings;
use crate::store::Reader;

pub struct ServerContext {
    pub reader: Reader,
    pub paused: Arc<AtomicBool>,
    pub started_at: String,
}

// 前端资源直接嵌入二进制，发布时只需一个 exe
const INDEX_HTML: &str = include_str!("../../web/index.html");
const STYLE_CSS: &str = include_str!("../../web/style.css");
const APP_JS: &str = include_str!("../../web/app.js");

pub fn spawn(ctx: Arc<ServerContext>, port: u16) -> Result<(), String> {
    let server = Server::http(("127.0.0.1", port)).map_err(|e| e.to_string())?;
    std::thread::Builder::new()
        .name("kc-http".into())
        .spawn(move || loop {
            match server.recv() {
                Ok(request) => handle_request(ctx.clone(), request),
                Err(_) => break,
            }
        })
        .expect("spawn http thread");
    Ok(())
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(v) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                out.push(v);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn query_params(query: &str) -> Vec<(String, String)> {
    query
        .split('&')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let (k, v) = match p.split_once('=') {
                Some((k, v)) => (k, v),
                None => (p, ""),
            };
            (percent_decode(k), percent_decode(v))
        })
        .collect()
}

fn param<'a>(params: &'a [(String, String)], key: &str) -> Option<&'a str> {
    params
        .iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.as_str())
}

fn common_headers() -> Vec<Header> {
    vec![
        Header::from_bytes(&b"Cache-Control"[..], &b"no-store"[..]).unwrap(),
        Header::from_bytes(&b"X-Content-Type-Options"[..], &b"nosniff"[..]).unwrap(),
    ]
}

fn respond(request: tiny_http::Request, status: u16, content_type: &str, body: Vec<u8>) {
    let extra = common_headers();
    let mut response = Response::from_data(body)
        .with_status_code(status)
        .with_header(Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap());
    for h in extra {
        response = response.with_header(h);
    }
    let _ = request.respond(response);
}

fn respond_json(request: tiny_http::Request, value: Value) {
    let body = serde_json::to_string(&value).unwrap_or_else(|_| "{}".into());
    respond(request, 200, "application/json", body.into_bytes());
}

fn respond_download(request: tiny_http::Request, filename: &str, content_type: &str, body: Vec<u8>) {
    let extra = common_headers();
    let mut response = Response::from_data(body)
        .with_header(
            Header::from_bytes(
                &b"Content-Type"[..],
                content_type.as_bytes(),
            )
            .unwrap(),
        )
        .with_header(
            Header::from_bytes(
                &b"Content-Disposition"[..],
                format!("attachment; filename=\"{filename}\"").as_bytes(),
            )
            .unwrap(),
        );
    for h in extra {
        response = response.with_header(h);
    }
    let _ = request.respond(response);
}

fn serve_static(request: tiny_http::Request, relative: &str) {
    let (ct, body) = match relative {
        "index.html" => ("text/html; charset=utf-8", INDEX_HTML.as_bytes().to_vec()),
        "style.css" => ("text/css; charset=utf-8", STYLE_CSS.as_bytes().to_vec()),
        "app.js" => ("text/javascript; charset=utf-8", APP_JS.as_bytes().to_vec()),
        _ => {
            respond(request, 404, "text/plain; charset=utf-8", b"Not Found".to_vec());
            return;
        }
    };
    respond(request, 200, ct, body)
}

fn summary_json(ctx: &ServerContext, start: Option<&str>, end: Option<&str>) -> Value {
    let reader = &ctx.reader;
    let (ls, le, ss, se) = range_bounds(start, end);
    let distance_px = reader.distance_range(&ss, &se);
    let date = if ls == le {
        ls.clone()
    } else {
        format!("{ls} 至 {le}")
    };
    let top_keys: Vec<Value> = reader
        .key_counts(&ss, &se, Some(15))
        .into_iter()
        .map(|(name, count)| json!({"key_name": name, "count": count}))
        .collect();
    let mouse: Vec<Value> = reader
        .mouse_counts(&ss, &se)
        .into_iter()
        .map(|(event_type, button, count)| {
            json!({"event_type": event_type, "button": button, "count": count})
        })
        .collect();
    json!({
        "start": ls,
        "end": le,
        "date": date,
        "keys": reader.key_count_range(&ss, &se),
        "clicks": reader.mouse_count_range(&ss, &se, "click"),
        "scrolls": reader.mouse_count_range(&ss, &se, "scroll"),
        "distance_px": (distance_px * 10.0).round() / 10.0,
        "distance_m": ((distance_px / 3779.527) * 1000.0).round() / 1000.0,
        "paused": ctx.paused.load(Ordering::SeqCst),
        "autostart_enabled": autostart::is_enabled(),
        "started_at": ctx.started_at,
        "last_updated": now_text(),
        "top_keys": top_keys,
        "mouse": mouse,
    })
}

fn heatmap_json(ctx: &ServerContext, start: Option<&str>, end: Option<&str>) -> Value {
    let reader = &ctx.reader;
    let (ls, le, ss, se) = range_bounds(start, end);
    let date = if ls == le {
        ls.clone()
    } else {
        format!("{ls} 至 {le}")
    };
    let keys: Vec<Value> = reader
        .key_counts(&ss, &se, None)
        .into_iter()
        .map(|(name, count)| json!({"key_name": name, "count": count}))
        .collect();
    json!({"start": ls, "end": le, "date": date, "keys": keys})
}

fn trend_json(ctx: &ServerContext, start: Option<&str>, end: Option<&str>) -> Value {
    let reader = &ctx.reader;
    let (ls, le, ss, se) = range_bounds(start, end);
    if ls == le && ls != "全部" {
        let mut hours_map: std::collections::HashMap<i32, (i64, i64, f64)> =
            (0..24).map(|h| (h, (0, 0, 0.0))).collect();
        for (hour, count) in reader.hourly_keys(&ss, &se) {
            if let Ok(h) = hour.parse::<i32>() {
                if let Some(slot) = hours_map.get_mut(&h) {
                    slot.0 = count;
                }
            }
        }
        for (hour, count) in reader.hourly_clicks(&ss, &se) {
            if let Ok(h) = hour.parse::<i32>() {
                if let Some(slot) = hours_map.get_mut(&h) {
                    slot.1 = count;
                }
            }
        }
        for (hour, dist) in reader.hourly_distance(&ss, &se) {
            if let Ok(h) = hour.parse::<i32>() {
                if let Some(slot) = hours_map.get_mut(&h) {
                    slot.2 = (dist * 10.0).round() / 10.0;
                }
            }
        }
        let hours: Vec<Value> = (0..24)
            .map(|h| {
                let (k, c, d) = hours_map[&h];
                json!({"hour": h, "keys": k, "clicks": c, "distance": d})
            })
            .collect();
        json!({"period": "hour", "start": ls, "end": le, "hours": hours})
    } else {
        let mut days_map: std::collections::BTreeMap<String, (i64, i64, f64)> = Default::default();
        for (day, count) in reader.daily_keys(&ss, &se) {
            days_map.entry(day).or_default().0 = count;
        }
        for (day, count) in reader.daily_clicks(&ss, &se) {
            days_map.entry(day).or_default().1 = count;
        }
        for (day, dist) in reader.daily_distance(&ss, &se) {
            days_map.entry(day).or_default().2 = (dist * 10.0).round() / 10.0;
        }
        let days: Vec<Value> = days_map
            .into_iter()
            .map(|(day, (k, c, d))| json!({"day": day, "keys": k, "clicks": c, "distance": d}))
            .collect();
        json!({"period": "day", "start": ls, "end": le, "days": days})
    }
}

fn handle_request(ctx: Arc<ServerContext>, mut request: tiny_http::Request) {
    let method = request.method().as_str().to_ascii_uppercase();
    let url = request.url().to_string();
    let (path, query) = match url.split_once('?') {
        Some((p, q)) => (p.to_string(), q.to_string()),
        None => (url.clone(), String::new()),
    };
    let params = query_params(&query);
    let q_start = param(&params, "start").map(|s| s.to_string());
    let q_end = param(&params, "end").map(|s| s.to_string());

    let mut body = String::new();
    if method == "POST" {
        let _ = request.as_reader().read_to_string(&mut body);
    }
    let body_json: Value = serde_json::from_str(&body).unwrap_or(Value::Null);

    match (method.as_str(), path.as_str()) {
        ("GET", "/") => serve_static(request, "index.html"),
        ("GET", p) if p.starts_with("/static/") => {
            serve_static(request, &p["/static/".len()..])
        }
        ("GET", "/api/summary") => respond_json(request, summary_json(&ctx, q_start.as_deref(), q_end.as_deref())),
        ("GET", "/api/heatmap") => respond_json(request, heatmap_json(&ctx, q_start.as_deref(), q_end.as_deref())),
        ("GET", "/api/trend") => respond_json(request, trend_json(&ctx, q_start.as_deref(), q_end.as_deref())),
        ("GET", "/api/export") => {
            let bytes = exporter::export_csv(&ctx.reader).into_bytes();
            respond_download(request, "keycounter_stats.csv", "text/csv; charset=utf-8", bytes)
        }
        ("GET", "/api/export/heatmap.png") => {
            let today = chrono::Local::now().format("%Y-%m-%d").to_string();
            let day = param(&params, "date").unwrap_or(&today);
            let day = valid_iso(day).unwrap_or(today);
            match exporter::heatmap_png(&ctx.reader, &day) {
                Some(bytes) => respond_download(
                    request,
                    &format!("keycounter_heatmap_{day}.png"),
                    "image/png",
                    bytes,
                ),
                None => respond(
                    request,
                    500,
                    "text/plain; charset=utf-8",
                    b"Heatmap image is unavailable".to_vec(),
                ),
            }
        }
        ("GET", "/api/settings") => respond_json(request, Value::Object(settings::get_all().into_iter().collect())),
        ("GET", "/api/status") => respond_json(
            request,
            json!({
                "paused": ctx.paused.load(Ordering::SeqCst),
                "autostart_enabled": autostart::is_enabled(),
            }),
        ),
        ("POST", "/api/pause") => {
            let paused = body_json.get("paused").and_then(|v| v.as_bool()).unwrap_or(false);
            ctx.paused.store(paused, Ordering::SeqCst);
            hooks::set_paused(paused);
            respond_json(request, json!({"paused": paused}))
        }
        ("POST", "/api/autostart") => {
            let enabled = body_json.get("enabled").and_then(|v| v.as_bool()).unwrap_or(false);
            let result = std::panic::catch_unwind(|| {
                if enabled {
                    autostart::enable();
                } else {
                    autostart::disable();
                }
            });
            match result {
                Ok(()) => respond_json(request, json!({"ok": true, "enabled": autostart::is_enabled()})),
                Err(_) => respond_json(request, json!({"ok": false, "error": "registry error"})),
            }
        }
        ("POST", "/api/settings") => {
            if let Some(action) = body_json.get("close_action").and_then(|v| v.as_str()) {
                if matches!(action, "ask" | "minimize" | "exit") {
                    settings::set("close_action", json!(action));
                }
            }
            respond_json(request, Value::Object(settings::get_all().into_iter().collect()))
        }
        _ => respond(request, 404, "text/plain; charset=utf-8", b"Not Found".to_vec()),
    }
}

fn valid_iso(s: &str) -> Option<String> {
    let b = s.as_bytes();
    if b.len() == 10 && b[4] == b'-' && b[7] == b'-' {
        Some(s.to_string())
    } else {
        None
    }
}
