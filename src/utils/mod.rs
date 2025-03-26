pub fn get_unprefixed_string<'a>(value: &'a str, prefix: &'a str) -> Result<&'a str, crate::Error> {
    value
        .strip_prefix(prefix)
        .ok_or(crate::error::FieldError::Invalid.into())
        .map(|version| version.into())
}
