use std::collections::HashMap;
use std::convert::{From, TryFrom};

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value<'buf>> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        
    s.split('&')
        .for_each(|substr| {
        let mut key = substr;
        let mut value = "";
        if let Some(i) = substr.find('=') {
            key = &substr[..i];
            value = &substr[i+1..]
        }
        data
            .entry(key)
            .and_modify(|existing: &mut Value| match existing {
                Value::Single(prev_value) => {
                    *existing = Value::Multiple(vec![prev_value, value]);
                },
                Value::Multiple(vec) => vec.push(value)
            })
            .or_insert(Value::Single(value));
    });

        QueryString {data}
    }
}
