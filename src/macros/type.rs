macro_rules! get_set_json {
    ($name_rust:ident, $setter:ident, $name_json:expr, bool) => {
        pub fn $name_rust(&self) -> Result<bool, $crate::Error> {
            self.0.get_bool($name_json)
        }

        set_map!($setter, $name_rust, bool, $name_json);
    };
    ($name_rust:ident, $setter:ident, $name_json:expr, u64) => {
        pub fn $name_rust(&self) -> Result<u64, $crate::Error> {
            self.0.get_u64($name_json)
        }

        set_map!($setter, $name_rust, u64, $name_json);
    };
    ($name_rust:ident, $setter:ident, $name_json:expr, &str) => {
        pub fn $name_rust(&self) -> Result<&str, $crate::Error> {
            self.0.get_str($name_json)
        }

        set_map!($setter, $name_rust, &str, $name_json);
    };
    ($name_rust:ident, $setter:ident, $name_json:expr, Vec<&str>) => {
        pub fn $name_rust(&self) -> Result<Vec<&str>, $crate::Error> {
            self.0.get_strs($name_json)
        }

        set_map!($setter, $name_rust, Vec<&str>, $name_json);
    };
    ($name_rust:ident, $setter:ident, $name_json:expr, std::collections::HashMap<String, String>) => {
        pub fn $name_rust(&self) -> Result<std::collections::HashMap<String, String>, $crate::Error> {
            self.0.get_map_string_string($name_json)
        }

        set_map!($setter, $name_rust,std::collections::HashMap<String, String>, $name_json);
    };
    ($name_rust:ident, $setter:ident, $name_json:expr, &str, $output:ident) => {
        pub fn $name_rust(&self) -> Result<$output, $crate::Error> {
            self.0.get_str($name_json)?.try_into()
        }

        set_map!($setter, $name_rust, $output, $name_json);
    };
    ($name_rust:ident, $setter:ident, $name_json:expr, u64, $output:ident) => {
        pub fn $name_rust(&self) -> Result<$output, $crate::Error> {
            self.0.get_u64($name_json)?.try_into()
        }

        set_map!($setter, $name_rust, $output, $name_json);
    };
    ($name_rust:ident, $setter:ident, $name_json:expr, &serde_json::Value, $output:ident) => {
        pub fn $name_rust(&self) -> Result<$output, $crate::Error> {
            self.0.get_json_value($name_json)?.try_into()
        }

        set_map!($setter, $name_rust, $output, $name_json);
    };
    ($name_rust:ident, $setter:ident, $name_json:expr, Vec<&str>, $output:ident) => {
        pub fn $name_rust(&self) -> Result<Vec<$output>, $crate::Error> {
            self.0
            .get_strs($name_json)?
            .into_iter()
            .map(|i| i.try_into())
            .collect()
        }

        set_map!($setter, $name_rust, Vec<$output>, $name_json);
    };
    ($name_rust:ident, $setter:ident, $name_json:expr, &Vec<serde_json::Value>, $output:ident) => {
        pub fn $name_rust(&self) -> Result<Vec<$output>, $crate::Error> {
            self.0
            .get_json_values($name_json)?
            .into_iter()
            .map(|i| i.try_into())
            .collect()
        }

        set_map!($setter, $name_rust, Vec<$output>, $name_json);
    };
}
pub(crate) use get_set_json;

macro_rules! set_map {
    ($method:ident, $parameter:ident, $parameter_type:ty, $name_json:expr) => {
        pub fn $method(mut self, $parameter: $parameter_type) -> Result<Self, $crate::Error> {
            self.0.insert_in_map($name_json, $parameter)?;
            Ok(self)
        }
    };
}
pub(crate) use set_map;

macro_rules! get_unprefixed_strs {
    ($method:ident, $prefix:expr) => {
        pub fn $method(&self) -> Result<Vec<&str>, $crate::Error> {
            self.0
                .as_strs()?
                .into_iter()
                .map(|s| $crate::utils::get_unprefixed_string(s, $prefix))
                .collect()
        }
    };
}
pub(crate) use get_unprefixed_strs;
