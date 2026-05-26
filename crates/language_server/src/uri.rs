use std::path::{Path, PathBuf};

pub fn path_to_uri(path: &Path) -> String {
    let mut value = path.to_string_lossy().replace('\\', "/");
    if !value.starts_with('/') {
        value = format!("/{value}");
    }
    format!("file://{}", encode_path(&value))
}

pub fn uri_to_path(uri: &str) -> Option<PathBuf> {
    let value = uri.strip_prefix("file://")?;
    Some(PathBuf::from(decode_path(value)))
}

fn encode_path(value: &str) -> String {
    let mut output = String::new();
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'.' | b'-' | b'_' | b'~' => {
                output.push(byte as char)
            }
            _ => output.push_str(&format!("%{byte:02X}")),
        }
    }
    output
}

fn decode_path(value: &str) -> String {
    let mut output = Vec::new();
    let bytes = value.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && index + 2 < bytes.len()
            && let Ok(hex) = u8::from_str_radix(&value[index + 1..index + 3], 16)
        {
            output.push(hex);
            index += 3;
        } else {
            output.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8_lossy(&output).to_string()
}

#[cfg(test)]
mod tests {
    use super::{path_to_uri, uri_to_path};
    use std::path::Path;

    #[test]
    fn round_trips_file_uri_with_spaces() {
        let path = Path::new("/tmp/dowe project/src/views.dowe");
        let uri = path_to_uri(path);

        assert_eq!(uri, "file:///tmp/dowe%20project/src/views.dowe");
        assert_eq!(uri_to_path(&uri).expect("path"), path);
    }
}
