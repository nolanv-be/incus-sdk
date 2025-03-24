#[macro_export]
macro_rules! inner_to_bool_method {
    ($name_rust:ident, $setter:ident, $name_json:expr) => {
        pub fn $name_rust(&self) -> Result<bool, Error> {
            self.inner()
                .get($name_json)
                .ok_or_else(|| FieldError::Missing)?
                .as_bool()
                .ok_or_else(|| FieldError::Invalid.into())
        }

        with_method!($setter, $name_rust, bool, $name_json);
    };
}

#[macro_export]
macro_rules! inner_to_u64_method {
    ($name_rust:ident, $setter:ident, $name_json:expr) => {
        pub fn $name_rust(&self) -> Result<u64, Error> {
            self.inner()
                .get($name_json)
                .ok_or_else(|| FieldError::Missing)?
                .as_u64()
                .ok_or_else(|| FieldError::Invalid.into())
        }

        with_method!($setter, $name_rust, u64, $name_json);
    };
}

#[macro_export]
macro_rules! inner_to_str_method {
    ($name_rust:ident, $setter:ident, $name_json:expr) => {
        pub fn $name_rust(&self) -> Result<String, Error> {
            self.0
                .get($name_json)
                .ok_or_else(|| FieldError::Missing)?
                .as_str()
                .ok_or_else(|| FieldError::Invalid.into())
                .map(|s| s.into())
        }

        with_method!($setter, $name_rust, String, $name_json);
    };
}

#[macro_export]
macro_rules! inner_to_vec_str_method {
    ($name_rust:ident, $setter:ident, $name_json:expr) => {
        pub fn $name_rust(&self) -> Result<Vec<String>, Error> {
            self.0
                .get($name_json)
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

        with_method!($setter, $name_rust, Vec<String>, $name_json);
    };
}

#[macro_export]
macro_rules! inner_to_map_str_str_method {
    ($name_rust:ident, $setter:ident, $name_json:expr) => {
        pub fn $name_rust(&self) -> Result<HashMap<String, String>, Error> {
            serde_json::from_value(
                self.0
                    .get($name_json)
                    .ok_or_else(|| FieldError::Missing)?
                    .clone(),
            )
            .map_err(|_| FieldError::Invalid.into())
        }

        with_method!($setter, $name_rust,HashMap<String, String>, $name_json);
    };
}

#[macro_export]
macro_rules! inner_str_to_struct_method {
    ($name_rust:ident, $setter:ident, $name_json:expr, $output:ident) => {
        pub fn $name_rust(&self) -> Result<$output, Error> {
            self.0
                .get($name_json)
                .ok_or_else(|| FieldError::Missing)?
                .as_str()
                .ok_or_else(|| FieldError::Invalid)?
                .try_into()
        }

        with_method!($setter, $name_rust, $output, $name_json);
    };
}

#[macro_export]
macro_rules! inner_object_to_struct_method {
    ($name_rust:ident, $setter:ident, $name_json:expr, $output:ident) => {
        pub fn $name_rust(&self) -> Result<$output, Error> {
            self.0
                .get($name_json)
                .ok_or_else(|| FieldError::Missing)?
                .as_object()
                .ok_or_else(|| FieldError::Invalid.into())
                .map(|v| v.into())
        }

        with_method!($setter, $name_rust, $output, $name_json);
    };
}

#[macro_export]
macro_rules! inner_array_str_to_vec_struct_method {
    ($name_rust:ident, $setter:ident, $name_json:expr, $output:ident) => {
        pub fn $name_rust(&self) -> Result<Vec<$output>, Error> {
            self.0
                .get($name_json)
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

        with_method!($setter, $name_rust, Vec<$output>, $name_json);
    };
}

#[macro_export]
macro_rules! inner_array_object_to_vec_struct_method {
    ($name_rust:ident, $setter:ident, $name_json:expr, $output:ident) => {
        pub fn $name_rust(&self) -> Result<Vec<$output>, Error> {
            self.0
                .get($name_json)
                .ok_or_else(|| FieldError::Missing)?
                .as_array()
                .ok_or_else(|| FieldError::Invalid)?
                .into_iter()
                .map(|method| {
                    method
                        .as_object()
                        .ok_or_else(|| FieldError::Invalid.into())
                        .map(|r| r.into())
                })
                .collect::<Result<Vec<$output>, Error>>()
        }

        with_method!($setter, $name_rust, Vec<$output>, $name_json);
    };
}

#[macro_export]
macro_rules! with_method {
    ($method:ident, $parameter:ident, $parameter_type:ty, $name_json:expr) => {
        pub fn $method(&mut self, $parameter: $parameter_type) -> Result<&mut Self, Error> {
            self.inner_mut().insert(
                $name_json.into(),
                serde_json::to_value($parameter).map_err(|_| FieldError::Invalid)?,
            );
            Ok(self)
        }
    };
}

#[macro_export]
macro_rules! inner_method {
    ($method:ident, $method_mut:ident) => {
        pub fn $method<'a>(&'a self) -> &'a serde_json::value::Map<String, serde_json::Value> {
            &self.0
        }
        pub fn $method_mut<'a>(
            &'a mut self,
        ) -> &'a mut serde_json::value::Map<String, serde_json::Value> {
            &mut self.0
        }
    };
}

#[macro_export]
macro_rules! inner_split_get_str_method {
    ($method:ident, $separator:expr, $nth:expr) => {
        pub fn $method(&self) -> Result<Vec<String>, Error> {
            self.inner()
                .as_array()
                .ok_or_else(|| FieldError::Invalid)?
                .iter()
                .map(|fingerprint| {
                    fingerprint
                        .as_str()
                        .ok_or_else(|| FieldError::Invalid)
                        .map(|s| {
                            s.split($separator)
                                .nth($nth)
                                .ok_or_else(|| FieldError::Invalid.into())
                                .map(|f| f.into())
                        })
                })
                .flatten()
                .collect::<Result<Vec<String>, Error>>()
        }
    };
}
