use napi_derive::napi;

#[napi(object)]
pub struct GetConnectionStringOptions {
    pub container_id_or_name: String,
    pub db_username: Option<String>,
    pub db_password: Option<String>,
    pub verify: Option<bool>,
}

impl From<GetConnectionStringOptions> for atlas_local::models::GetConnectionStringOptions {
    fn from(source: GetConnectionStringOptions) -> Self {
        Self {
            container_id_or_name: source.container_id_or_name,
            db_username: source.db_username,
            db_password: source.db_password,
            verify: source.verify,
        }
    }
}

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_connection_string_options() {
        let options = GetConnectionStringOptions {
            container_id_or_name: "test_container".into(),
            db_username: Some("test_user".into()),
            db_password: Some("test_pass".into()),
            verify: Some(true),
        };

        let get_connection_string_options: atlas_local::models::GetConnectionStringOptions = options.into();

        assert_eq!(get_connection_string_options.container_id_or_name, "test_container");
        assert_eq!(get_connection_string_options.db_username, Some("test_user".into()));
        assert_eq!(get_connection_string_options.db_password, Some("test_pass".into()));
        assert_eq!(get_connection_string_options.verify, Some(true));
    }
}
