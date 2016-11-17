#[allow(non_camel_case_types, dead_code)]
#[derive(Debug)]
pub enum ApiError {
    FORBIDDEN,
    UNKNOWN_TOKEN,
    BAD_JSON,
    NOT_JSON,
    NOT_FOUND,
    LIMIT_EXCEEDED,
    USER_IN_USE,
    INVALID_USERNAME,
    ROOM_IN_USE,
    BAD_PAGINATION,
    THREEPID_IN_USE,
    THREEPID_NOT_FOUND,
    THREEPID_NOT_TRUSTED,
    NOT_IMPLEMENTED,
}
