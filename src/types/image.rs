use crate::{macros::*, types::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Image(pub JsonWrapperMap);
impl TryFrom<&serde_json::Value> for Image {
    type Error = crate::Error;

    fn try_from(json: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(Image(json.clone().try_into()?))
    }
}

impl Image {
    get_set_json!(
        aliases,
        with_aliases,
        "aliases",
        &Vec<serde_json::Value>,
        ImageAlias
    );

    get_set_json!(
        architecture,
        with_architecture,
        "architecture",
        &str,
        Architecture
    );

    get_set_json!(auto_update, with_auto_update, "auto_update", bool);

    get_set_json!(cached, with_cached, "cached", bool);

    get_set_json!(created_at, with_created_at, "created_at", &str);

    get_set_json!(expires_at, with_expires_at, "expires_at", &str);

    get_set_json!(filename, with_filename, "filename", &str);

    get_set_json!(fingerprint, with_fingerprint, "fingerprint", &str);

    get_set_json!(last_used_at, with_last_used_at, "last_used_at", &str);

    get_set_json!(profiles, with_profiles, "profiles", Vec<&str>);

    get_set_json!(project, with_project, "project", &str);

    get_set_json!(properties, with_properties, "properties", std::collections::HashMap<String, String>);

    get_set_json!(public, with_public, "public", bool);

    get_set_json!(size, with_size, "size", u64);

    get_set_json!(image_type, with_image_type, "type", &str, ImageType);

    get_set_json!(
        update_source,
        with_update_source,
        "update_source",
        &serde_json::Value,
        ImageSource
    );

    get_set_json!(uploaded_at, with_uploaded_at, "uploaded_at", &str);

    get_set_json!(
        compression_algorithm,
        with_compression_algorithm,
        "compression_algorithm",
        &str
    );
}
