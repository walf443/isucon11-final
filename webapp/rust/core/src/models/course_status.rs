use fake::Dummy;
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::{Database, Decode, Encode};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, Dummy)]
#[serde(rename_all = "kebab-case")]
pub enum CourseStatus {
    Registration,
    InProgress,
    Closed,
}

impl<DB: Database> sqlx::Type<DB> for CourseStatus
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

impl<'r, DB: Database> sqlx::Decode<'r, DB> for CourseStatus
where
    &'r str: Decode<'r, DB>,
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<DB>>::decode(value)?;
        match value {
            "registration" => Ok(Self::Registration),
            "in-progress" => Ok(Self::InProgress),
            "closed" => Ok(Self::Closed),
            v => Err(format!("Unknown enum variant: {}", v).into()),
        }
    }
}

impl<'q, DB: Database> sqlx::Encode<'q, DB> for CourseStatus
where
    &'q str: Encode<'q, DB>,
{
    fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        let result = match *self {
            Self::Registration => "registration",
            Self::InProgress => "in-progress",
            Self::Closed => "closed",
        };

        <&str as Encode<'_, DB>>::encode_by_ref(&result, buf)
    }
}
