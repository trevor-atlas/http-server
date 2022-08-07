use std::fmt::{Display, Formatter, Result as FmtResult};
use std::collections::HashMap;
use std::convert::From;

#[derive(Debug)]
pub struct Headers<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> Headers<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value<'buf>> {
        self.data.get(key)
    }
}

// Host: developer.mozilla.org\r\n
// User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.9; rv:50.0) Gecko/20100101 Firefox/50.0\r\n
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8\r\n
// Accept-Language: en-US,en;q=0.5\r\n
// Accept-Encoding: gzip, deflate, br\r\n
// Referer: https://developer.mozilla.org/testpage.html\r\n
// Connection: keep-alive\r\n
// Upgrade-Insecure-Requests: 1\r\n
// If-Modified-Since: Mon, 18 Jul 2016 02:36:04 GMT\r\n
// If-None-Match: "c561c68d0ba92bbeb8b0fff2a9199f722e3a621a"\r\n
// Cache-Control: max-age=0\r\n\r\n

impl<'buf> From<&'buf str> for Headers<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        
    for (_, str) in s.split("\r\n").enumerate() {
        if str.is_empty() {
            break;
        }
        let mut key = str;
        let mut value = "";
        if let Some(i) = str.find(':') {
            key = &str[..i];
            // skip ": "
            value = &str[i+2..];
        }
        data.entry(key).and_modify(|existing: &mut Value| match existing {
            Value::Single(prev_value) => {
                *existing = Value::Multiple(vec![prev_value, value]);
            },
            Value::Multiple(vec) => vec.push(value)
        })
        .or_insert(Value::Single(value));
    };

        Headers {data}
    }
}

impl<'buf> Display for Headers<'buf> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        // status codes go up to ~5xx so can't use u8
        write!(f, "{:?}", self.data)
    }
}

impl<'buf> Display for Value<'buf> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let value = match self {
            Value::Single(value) => value.to_string(),
            Value::Multiple(values) => values.concat()
        };
        write!(f, "{}", value)
    }
}
