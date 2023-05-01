use fake::Dummy;
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::{Database, Decode};

#[derive(Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, Clone, Dummy)]
#[serde(rename_all = "lowercase")]
pub enum DayOfWeek {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl<DB: Database> sqlx::Type<DB> for DayOfWeek
where
    str: sqlx::Type<DB>,
{
    fn type_info() -> DB::TypeInfo {
        <str as sqlx::Type<DB>>::type_info()
    }

    fn compatible(ty: &<DB as sqlx::Database>::TypeInfo) -> bool {
        <&str as sqlx::Type<DB>>::compatible(ty)
    }
}

impl<'r, DB: Database> sqlx::Decode<'r, DB> for DayOfWeek
where
    &'r str: Decode<'r, DB>,
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<DB>>::decode(value)?;
        match value {
            "monday" => Ok(Self::Monday),
            "tuesday" => Ok(Self::Tuesday),
            "wednesday" => Ok(Self::Wednesday),
            "thursday" => Ok(Self::Thursday),
            "friday" => Ok(Self::Friday),
            v => Err(format!("Unknown enum variant: {}", v).into()),
        }
    }
}

impl<'q, DB: Database> sqlx::Encode<'q, DB> for DayOfWeek
where
    &'q str: sqlx::Encode<'q, DB>,
{
    fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        let result = match *self {
            Self::Monday => "monday",
            Self::Tuesday => "tuesday",
            Self::Wednesday => "wednesday",
            Self::Thursday => "thursday",
            Self::Friday => "friday",
        };

        <&str as sqlx::Encode<'_, DB>>::encode_by_ref(&result, buf)
    }
}
