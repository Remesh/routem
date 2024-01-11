use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Debug, Clone, PartialEq)]
pub struct ParamType {
    pub typename: &'static str,
    pub check: fn(&str) -> bool,
}

pub fn check_str(_s: &str) -> bool {
    true
}

pub fn check_uuid(s: &str) -> bool {
    uuid::Uuid::try_parse(s).is_ok()
}

pub fn check_int(s: &str) -> bool {
    s.parse::<i64>().is_ok()
}

pub const STRING_PARAM: ParamType = ParamType {
    typename: "string",
    check: check_str,
};

pub const UUID_PARAM: ParamType = ParamType {
    typename: "uuid",
    check: check_uuid,
};

pub const INT_PARAM: ParamType = ParamType {
    typename: "int",
    check: check_int,
};

pub type ParamMap = HashMap<&'static str, ParamType>;

pub static DEFAULT_PARAM_TYPES: Lazy<ParamMap> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(STRING_PARAM.typename, STRING_PARAM);
    m.insert(UUID_PARAM.typename, UUID_PARAM);
    m.insert(INT_PARAM.typename, INT_PARAM);
    m
});
