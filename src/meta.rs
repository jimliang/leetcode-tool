#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum MetaData {
    Base {
        name: String,
        params: Vec<MetaDataParam>,
        r#return: MetaDataReturn,
    },
    Class {
        classname: String,
        constructor: MetaDataConstructor,
        methods: Vec<MetaDataMethod>,
        r#return: MetaDataReturn,
        // systemdesign: bool,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetaDataConstructor {
    params: Vec<MetaDataParam>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetaDataParam {
    pub name: String,
    #[serde(rename = "type")]
    #[serde(with = "type_serde")]
    pub r#type: MetaDataType,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetaDataReturn {
    #[serde(rename = "type")]
    #[serde(with = "type_serde")]
    pub r#type: MetaDataType,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum MetaDataType {
    String,
    Integer,
    TreeNode,
    ListNode,
    Character,
    Void,
    Bool,
    List(Box<MetaDataType>),
    Unknow(String),
}

pub fn parse_type(s: &str) -> MetaDataType {
    let val: Result<MetaDataType, ()> = match s {
        "TreeNode" => Ok(MetaDataType::TreeNode),
        "ListNode" => Ok(MetaDataType::ListNode),
        "string" => Ok(MetaDataType::String),
        "string[]" | "list<string>" => Ok(MetaDataType::List(Box::new(MetaDataType::String))),
        "string[][]" | "list<list<string>>" => Ok(MetaDataType::List(Box::new(
            MetaDataType::List(Box::new(MetaDataType::String)),
        ))),
        "integer" | "long" => Ok(MetaDataType::Integer),
        "integer[]" | "list<integer>" | "long[]" | "list<long>" => {
            Ok(MetaDataType::List(Box::new(MetaDataType::Integer)))
        }
        "integer[][]" | "list<list<integer>>" | "long[][]" | "list<list<long>>" => {
            Ok(MetaDataType::List(Box::new(MetaDataType::List(Box::new(
                MetaDataType::Integer,
            )))))
        }
        "boolean" => Ok(MetaDataType::Bool),
        "character" => Ok(MetaDataType::Character),
        "void" => Ok(MetaDataType::Void),
        _ => Ok(MetaDataType::Unknow(s.to_owned())),
    };
    val.unwrap()
}
impl From<&str> for MetaDataType {
    fn from(s: &str) -> Self {
        parse_type(s)
    }
}

impl From<String> for MetaDataType {
    fn from(s: String) -> Self {
        parse_type(&s)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetaDataMethod {
    pub name: String,
    pub params: Vec<MetaDataParam>,
    pub r#return: MetaDataReturn,
}

mod type_serde {
    use super::MetaDataType;
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(t: &MetaDataType, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // if let Some(ref d) = *t {
        return s.serialize_str(&format!("{:?}", t));
        // }
        // s.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<MetaDataType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        if let Some(s) = s {
            return Ok(s.into());
        }

        Ok(MetaDataType::Unknow("".to_owned()))
    }
}
pub fn parse_meta(s: &str) -> anyhow::Result<MetaData> {
    let meta = serde_json::from_str(s)?;
    Ok(meta)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_type() {
        let meta_type: MetaDataType = "list<list<string>>".into();
        println!("{:?}", meta_type);
    }

    #[test]
    fn test_parse() {
        let a = parse_meta("{\n  \"name\": \"buddyStrings\",\n  \"params\": [\n    {\n      \"name\": \"s\",\n      \"type\": \"string\"\n    },\n    {\n      \"name\": \"goal\",\n      \"type\": \"string\"\n    }\n  ],\n  \"return\": {\n    \"type\": \"boolean\"\n  }\n}").unwrap();
        println!("--> meta: {:?}", a)
    }
}
