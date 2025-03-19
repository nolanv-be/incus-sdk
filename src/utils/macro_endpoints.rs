#[macro_export]
macro_rules! build_query {
    ($($param:ident),*) => {{
        let mut queries = Vec::new();
        $(
            if let Some(val) = $param {
                queries.push(format!("{}={val}", stringify!($param)));
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
