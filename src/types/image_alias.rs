use crate::{macros::*, types::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct ImageAlias(pub JsonWrapperMap);
impl TryFrom<&serde_json::Value> for ImageAlias {
    type Error = crate::Error;

    fn try_from(json: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(ImageAlias(json.clone().try_into()?))
    }
}

impl ImageAlias {
    get_set_json!(name, with_name, "name", &str);
    get_set_json!(description, with_description, "description", &str);
    get_set_json!(target, with_target, "target", &str);
    get_set_json!(image_type, with_image_type, "type", &str, ImageType);
}
