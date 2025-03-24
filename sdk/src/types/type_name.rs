//! Ubiqutously used name wrapper type. Useful to have this defined globally
//! so that we don't have to redefine it in every module that uses it.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TypeName {
    pub name: String,
}

impl std::fmt::Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_name_deser_display() {
        let name: TypeName = serde_json::from_str(r#"{"name":"test"}"#).unwrap();

        assert_eq!(name.to_string(), "test");
    }
}
