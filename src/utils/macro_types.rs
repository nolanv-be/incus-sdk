#[macro_export]
macro_rules! inner_to_bool_method {
    ($method:ident, $field:expr) => {
        pub fn $method(&self) -> Result<bool, Error> {
            self.inner()
                .get($field)
                .ok_or_else(|| FieldError::Missing)?
                .as_bool()
                .ok_or_else(|| FieldError::Invalid.into())
        }
    };
}

#[macro_export]
macro_rules! inner_to_u64_method {
    ($method:ident, $field:expr) => {
        pub fn $method(&self) -> Result<u64, Error> {
            self.inner()
                .get($field)
                .ok_or_else(|| FieldError::Missing)?
                .as_u64()
                .ok_or_else(|| FieldError::Invalid.into())
        }
    };
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
macro_rules! inner_to_map_str_str_method {
    ($method:ident, $field:expr) => {
        pub fn $method(&self) -> Result<HashMap<String, String>, Error> {
            serde_json::from_value(
                self.inner()
                    .get($field)
                    .ok_or_else(|| FieldError::Missing)?
                    .clone(),
            )
            .map_err(|_| FieldError::Invalid.into())
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
