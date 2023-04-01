#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}
impl sqlx::Type<sqlx::MySql> for DayOfWeek {
    fn type_info() -> sqlx::mysql::MySqlTypeInfo {
        str::type_info()
    }

    fn compatible(ty: &sqlx::mysql::MySqlTypeInfo) -> bool {
        <&str>::compatible(ty)
    }
}
impl<'r> sqlx::Decode<'r, sqlx::MySql> for DayOfWeek {
    fn decode(
        value: sqlx::mysql::MySqlValueRef<'r>,
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        match <&'r str>::decode(value)? {
            "monday" => Ok(Self::Monday),
            "tuesday" => Ok(Self::Tuesday),
            "wednesday" => Ok(Self::Wednesday),
            "thursday" => Ok(Self::Thursday),
            "friday" => Ok(Self::Friday),
            v => Err(format!("Unknown enum variant: {}", v).into()),
        }
    }
}
impl<'q> sqlx::Encode<'q, sqlx::MySql> for DayOfWeek {
    fn encode_by_ref(&self, buf: &mut Vec<u8>) -> sqlx::encode::IsNull {
        match *self {
            Self::Monday => "monday",
            Self::Tuesday => "tuesday",
            Self::Wednesday => "wednesday",
            Self::Thursday => "thursday",
            Self::Friday => "friday",
        }
        .encode_by_ref(buf)
    }
}
