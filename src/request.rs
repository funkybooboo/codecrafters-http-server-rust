use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub params: HashMap<String, String>,
}

impl Request {
    pub fn parse<R: BufRead>(reader: &mut R) -> io::Result<Self> {
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;
        let request_line = request_line.trim_end();
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();
        let http_version = parts.next().unwrap_or("").to_string();

        let headers = Self::parse_headers(reader)?;
        let body = Self::parse_body(reader, &headers)?;

        Ok(Request {
            method,
            path,
            http_version,
            headers,
            body,
            params: HashMap::new(),
        })
    }

    fn parse_headers<R: BufRead>(reader: &mut R) -> io::Result<HashMap<String, String>> {
        let mut headers = HashMap::new();
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 || line.trim().is_empty() {
                break;
            }
            if let Some((key, value)) = line.split_once(':') {
                headers.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        Ok(headers)
    }

    fn parse_body<R: BufRead>(reader: &mut R, headers: &HashMap<String, String>) -> io::Result<String> {
        if let Some(cl_val) = headers.get("Content-Length") {
            if let Ok(content_length) = cl_val.parse::<usize>() {
                let mut body_buffer = vec![0; content_length];
                reader.read_exact(&mut body_buffer)?;
                return String::from_utf8(body_buffer).map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidData, format!("UTF-8 error: {}", e))
                });
            }
        }
        Ok(String::new())
    }
}
