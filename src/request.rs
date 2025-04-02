use std::io::BufRead;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub http_version: String,
    pub headers: Vec<String>,
    pub body: String,
}

impl Request {
    /// Parses an HTTP request from the given buffered reader.
    pub fn parse<R: BufRead>(reader: &mut R) -> std::io::Result<Self> {
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;
        let request_line = request_line.trim_end();
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap_or("").to_string();
        let path = parts.next().unwrap_or("").to_string();
        let http_version = parts.next().unwrap_or("").to_string();

        let headers = Self::parse_headers(reader)?;
        let body = String::new(); // For now, we're not handling the body.

        Ok(Request {
            method,
            path,
            http_version,
            headers,
            body,
        })
    }

    /// Parses HTTP headers until an empty line is encountered.
    fn parse_headers<R: BufRead>(reader: &mut R) -> std::io::Result<Vec<String>> {
        let mut headers = Vec::new();
        loop {
            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 || line.trim().is_empty() {
                break;
            }
            headers.push(line.trim_end().to_string());
        }
        Ok(headers)
    }
}
