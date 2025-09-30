use serde::{de::Deserializer, ser::Serializer, Deserialize, Serialize};

/// Deserialize a `Vec<u8>` into a [reqwest::Url].
pub fn deserialize_bytes_to_url<'de, D>(deserializer: D) -> Result<reqwest::Url, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
    let url = String::from_utf8(bytes).map_err(serde::de::Error::custom)?;

    reqwest::Url::parse(&url).map_err(serde::de::Error::custom)
}

/// Inverse of [deserialize_bytes_to_url].
pub fn serialize_url_to_bytes<S>(value: &reqwest::Url, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let url = value.to_string();
    let bytes = url.into_bytes();

    bytes.serialize(serializer)
}

/// Deserialize a `Vec<u8>` into a [serde_json::Value].
pub fn deserialize_bytes_to_json_value<'de, D>(
    deserializer: D,
) -> Result<serde_json::Value, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
    let value = String::from_utf8(bytes).map_err(serde::de::Error::custom)?;

    serde_json::from_str(&value).map_err(serde::de::Error::custom)
}

/// Inverse of [deserialize_bytes_to_json_value].
pub fn serialize_json_value_to_bytes<S>(
    value: &serde_json::Value,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let value = serde_json::to_string(value).map_err(serde::ser::Error::custom)?;
    let bytes = value.into_bytes();

    bytes.serialize(serializer)
}

/// Custom parser for deserializing to a [u64] from Sui Events. They wrap this
/// type as a string to avoid overflow.
///
/// See [sui_sdk::rpc_types::SuiMoveValue] for more information.
pub fn deserialize_sui_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let value: String = Deserialize::deserialize(deserializer)?;
    let value = value.parse::<u64>().map_err(serde::de::Error::custom)?;

    Ok(value)
}

/// Inverse of [deserialize_sui_u64] for indexing reasons.
pub fn serialize_sui_u64<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    value.to_string().serialize(serializer)
}

/// Deserialize a `Vec<u8>` into a `String` using lossy UTF-8 conversion.
pub fn deserialize_bytes_to_lossy_utf8<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

pub fn deserialize_string_to_datetime<'de, D>(
    deserializer: D,
) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: String = Deserialize::deserialize(deserializer)?;
    let timestamp = value.parse::<i64>().map_err(serde::de::Error::custom)?;
    let datetime = chrono::DateTime::from_timestamp_millis(timestamp);

    datetime.ok_or(serde::de::Error::custom("datetime out of range"))
}

#[cfg(test)]
mod tests {
    use {super::*, serde::Deserialize};

    #[derive(Deserialize, Serialize)]
    struct TestUrlStruct {
        #[serde(
            deserialize_with = "deserialize_bytes_to_url",
            serialize_with = "serialize_url_to_bytes"
        )]
        url: reqwest::Url,
    }

    #[derive(Deserialize, Serialize)]
    struct TestSuiU64Struct {
        #[serde(
            deserialize_with = "deserialize_sui_u64",
            serialize_with = "serialize_sui_u64"
        )]
        value: u64,
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct TestDescriptionStruct {
        #[serde(deserialize_with = "deserialize_bytes_to_lossy_utf8")]
        value: String,
    }

    #[test]
    fn test_lossy_utf8_deserialization_exact() {
        // The array [49, 50, 51] corresponds to a valid UTF-8 byte sequence,
        // which is the string "123".
        let input = r#"{"value":[49,50,51]}"#;
        let result: TestDescriptionStruct = serde_json::from_str(input).unwrap();
        assert_eq!(result.value, "123");
    }

    #[test]
    fn test_lossy_utf8_deserialization_lossy() {
        // The array [49, 50, 255, 48] does not correspond to a valid UTF-8 byte sequence.
        // "12\u{FFFD}0" is its lossy UTF-8 representation.
        let input = r#"{"value":[49,50,255,48]}"#;
        let result = serde_json::from_str::<TestDescriptionStruct>(input).unwrap();
        assert_eq!(result.value, "12\u{FFFD}0");
    }

    #[test]
    fn test_url_deser_ser() {
        let bytes = b"https://example.com/";
        let input = format!(r#"{{"url":{}}}"#, serde_json::to_string(&bytes).unwrap());

        let result: TestUrlStruct = serde_json::from_str(&input).unwrap();

        assert_eq!(
            result.url,
            reqwest::Url::parse("https://example.com").unwrap()
        );

        let ser = serde_json::to_string(&result).unwrap();
        assert_eq!(ser, input);
    }

    #[test]
    fn test_sui_u64_deser_ser() {
        let input = r#"{"value":"123"}"#;
        let result: TestSuiU64Struct = serde_json::from_str(input).unwrap();
        assert_eq!(result.value, 123);

        let ser = serde_json::to_string(&result).unwrap();
        assert_eq!(ser, input);
    }
}
