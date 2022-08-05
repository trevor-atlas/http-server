use super::Method;

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<String>,
}
