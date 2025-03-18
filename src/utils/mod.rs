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

#[macro_export]
macro_rules! inner_to_str_method {
    ($method:ident, $field:expr) => {
        pub fn $method(&self) -> Result<String, Error> {
            self.inner()
                .get($field)
                .ok_or_else(|| FieldError::Missing)?
                .as_str()
                .ok_or_else(|| FieldError::Invalid.into())
                .map(|s| s.into())
        }
    };
}

#[macro_export]
macro_rules! inner_to_vec_str_method {
    ($method:ident, $field:expr) => {
        pub fn $method(&self) -> Result<Vec<String>, Error> {
            self.inner()
                .get($field)
                .ok_or_else(|| FieldError::Missing)?
                .as_array()
                .ok_or_else(|| FieldError::Invalid)?
                .into_iter()
                .map(|api| {
                    api.as_str()
                        .ok_or_else(|| FieldError::Invalid.into())
                        .map(|s| s.to_string())
                })
                .collect()
        }
    };
}

#[macro_export]
macro_rules! inner_to_struct_method {
    ($method:ident, $field:expr, $output:ident) => {
        pub fn $method(&self) -> Result<$output, Error> {
            self.inner()
                .get($field)
                .ok_or_else(|| FieldError::Missing)?
                .as_str()
                .ok_or_else(|| FieldError::Invalid)?
                .try_into()
        }
    };
}

#[macro_export]
macro_rules! inner_to_vec_struct_method {
    ($method:ident, $field:expr, $output:ident) => {
        pub fn $method(&self) -> Result<Vec<$output>, Error> {
            self.inner()
                .get($field)
                .ok_or_else(|| FieldError::Missing)?
                .as_array()
                .ok_or_else(|| FieldError::Invalid)?
                .into_iter()
                .map(|method| {
                    method
                        .as_str()
                        .ok_or_else(|| FieldError::Invalid)
                        .map(|s| s.try_into())
                })
                .flatten()
                .collect::<Result<Vec<$output>, Error>>()
        }
    };
}
