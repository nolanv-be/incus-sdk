#[macro_export]
macro_rules! build_query {
    ($($var:ident),*) => {{
        let mut queries = Vec::new();
        $(
            if let Some(val) = $var {
                queries.push(format!("{}={val}", stringify!($var)));
            }
        )*
        let query_string = if !queries.is_empty() {
            format!("?{}", queries.join("&"))
        } else {
            "".into()
        };
        query_string
    }};
}
