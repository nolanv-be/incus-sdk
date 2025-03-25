macro_rules! get_set_bool {
    ($name_rust:ident, $setter:ident, $name_json:expr) => {
        pub fn $name_rust(&self) -> Result<bool, $crate::Error> {
            $crate::types::get_bool(self.inner(), $name_json)
        }

        set_map!($setter, $name_rust, bool, $name_json);
    };
}
pub(crate) use get_set_bool;

macro_rules! get_set_u64 {
    ($name_rust:ident, $setter:ident, $name_json:expr) => {
        pub fn $name_rust(&self) -> Result<u64, $crate::Error> {
            $crate::types::get_u64(self.inner(), $name_json)
        }

        set_map!($setter, $name_rust, u64, $name_json);
    };
}
pub(crate) use get_set_u64;

macro_rules! get_set_string {
    ($name_rust:ident, $setter:ident, $name_json:expr) => {
        pub fn $name_rust(&self) -> Result<String, $crate::Error> {
            $crate::types::get_string(self.inner(), $name_json)
        }

        set_map!($setter, $name_rust, String, $name_json);
    };
}
pub(crate) use get_set_string;

macro_rules! get_set_vec_str {
    ($name_rust:ident, $setter:ident, $name_json:expr) => {
        pub fn $name_rust(&self) -> Result<Vec<String>, $crate::Error> {
            $crate::types::get_vec_string(self.inner(), $name_json)
        }

        set_map!($setter, $name_rust, Vec<String>, $name_json);
    };
}
pub(crate) use get_set_vec_str;

macro_rules! get_set_map_string_string {
    ($name_rust:ident, $setter:ident, $name_json:expr) => {
        pub fn $name_rust(&self) -> Result<std::collections::HashMap<String, String>, $crate::Error> {
            $crate::types::get_map_string_string(self.inner(), $name_json)
        }

        set_map!($setter, $name_rust,std::collections::HashMap<String, String>, $name_json);
    };
}
pub(crate) use get_set_map_string_string;

macro_rules! get_set_struct_from_string {
    ($name_rust:ident, $setter:ident, $name_json:expr, $output:ident) => {
        pub fn $name_rust(&self) -> Result<$output, $crate::Error> {
            $crate::types::get_struct_from_string(self.inner(), $name_json)
        }

        set_map!($setter, $name_rust, $output, $name_json);
    };
}
pub(crate) use get_set_struct_from_string;

macro_rules! get_set_struct {
    ($name_rust:ident, $setter:ident, $name_json:expr, $output:ident) => {
        pub fn $name_rust(&self) -> Result<$output, $crate::Error> {
            $crate::types::get_struct(self.inner(), $name_json)
        }

        set_map!($setter, $name_rust, $output, $name_json);
    };
}
pub(crate) use get_set_struct;

macro_rules! get_set_array_str_to_vec_struct {
    ($name_rust:ident, $setter:ident, $name_json:expr, $output:ident) => {
        pub fn $name_rust(&self) -> Result<Vec<$output>, $crate::Error> {
            $crate::types::get_vec_struct_from_array_string(self.inner(), $name_json)
        }

        set_map!($setter, $name_rust, Vec<$output>, $name_json);
    };
}
pub(crate) use get_set_array_str_to_vec_struct;

macro_rules! get_set_vec_struct {
    ($name_rust:ident, $setter:ident, $name_json:expr, $output:ident) => {
        pub fn $name_rust(&self) -> Result<Vec<$output>, $crate::Error> {
            $crate::types::get_vec_struct(self.inner(), $name_json)
        }

        set_map!($setter, $name_rust, Vec<$output>, $name_json);
    };
}
pub(crate) use get_set_vec_struct;

macro_rules! set_map {
    ($method:ident, $parameter:ident, $parameter_type:ty, $name_json:expr) => {
        pub fn $method(&mut self, $parameter: $parameter_type) -> Result<&mut Self, $crate::Error> {
            $crate::types::insert_in_map(self.inner_mut(), $name_json, $parameter)?;
            Ok(self)
        }
    };
}
pub(crate) use set_map;

macro_rules! get_set_inner {
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
pub(crate) use get_set_inner;
