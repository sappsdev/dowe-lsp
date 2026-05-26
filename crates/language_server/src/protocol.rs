use serde_json::{Value, json};
use std::io::{self, BufRead, BufReader, Read, Write};

pub struct Message {
    pub id: Option<Value>,
    pub method: Option<String>,
    pub params: Value,
}

pub fn read_message(reader: &mut BufReader<io::StdinLock<'_>>) -> io::Result<Option<Message>> {
    let mut content_length = None;
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            return Ok(None);
        }
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            break;
        }
        if let Some(value) = trimmed.strip_prefix("Content-Length:") {
            content_length =
                Some(value.trim().parse::<usize>().map_err(|error| {
                    io::Error::new(io::ErrorKind::InvalidData, error.to_string())
                })?);
        }
    }
    let Some(length) = content_length else {
        return Ok(None);
    };
    let mut body = vec![0u8; length];
    reader.read_exact(&mut body)?;
    let value: Value = serde_json::from_slice(&body)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
    Ok(Some(Message {
        id: value.get("id").cloned(),
        method: value
            .get("method")
            .and_then(Value::as_str)
            .map(str::to_string),
        params: value.get("params").cloned().unwrap_or(Value::Null),
    }))
}

pub fn write_response(id: Value, result: Value) -> io::Result<()> {
    write_value(json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": result
    }))
}

pub fn write_error(id: Value, code: i64, message: impl AsRef<str>) -> io::Result<()> {
    write_value(json!({
        "jsonrpc": "2.0",
        "id": id,
        "error": {
            "code": code,
            "message": message.as_ref()
        }
    }))
}

pub fn write_notification(method: &str, params: Value) -> io::Result<()> {
    write_value(json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params
    }))
}

fn write_value(value: Value) -> io::Result<()> {
    let body = serde_json::to_vec(&value)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error.to_string()))?;
    let mut stdout = io::stdout().lock();
    write!(stdout, "Content-Length: {}\r\n\r\n", body.len())?;
    stdout.write_all(&body)?;
    stdout.flush()
}
