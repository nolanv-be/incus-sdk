use crate::{macros::*, types::*};

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Operation(pub JsonWrapperMap);
impl TryFrom<&serde_json::Value> for Operation {
    type Error = crate::Error;

    fn try_from(json: &serde_json::Value) -> Result<Self, Self::Error> {
        Ok(Operation(json.clone().try_into()?))
    }
}

impl Operation {
    get_set_json!(class, with_class, "class", &str, OperationClass);

    get_set_json!(created_at, with_created_at, "created_at", &str);

    get_set_json!(description, with_description, "description", &str);

    get_set_json!(err, with_err, "err", &str);

    get_set_json!(id, with_id, "id", &str);

    get_set_json!(location, with_location, "location", &str);

    get_set_json!(
        metadata,
        with_metadata,
        "metadata",
        &serde_json::Value,
        JsonWrapperMap
    );

    get_set_json!(
        resources,
        with_resources,
        "resources",
        &serde_json::Value,
        JsonWrapperMap
    );

    get_set_json!(
        status_code,
        with_status_code,
        "status_code",
        u64,
        IncusResponseStatus
    );

    get_set_json!(updated_at, with_updated_at, "updated_at", &str);
}
