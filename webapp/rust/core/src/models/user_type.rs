use fake::Dummy;
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::{Database, Decode, Encode};

#[derive(Debug, PartialEq, Eq, Dummy)]
pub enum UserType {
    Student,
    Teacher,
}

impl<DB: Database> sqlx::Type<DB> for UserType
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

impl<'r, DB: Database> sqlx::Decode<'r, DB> for UserType
where
    &'r str: Decode<'r, DB>,
{
    fn decode(value: <DB as HasValueRef<'r>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<DB>>::decode(value)?;
        match value {
            "student" => Ok(Self::Student),
            "teacher" => Ok(Self::Teacher),
            v => Err(format!("Unknown enum variant: {}", v).into()),
        }
    }
}

impl<'q, DB: Database> sqlx::Encode<'q, DB> for UserType
where
    &'q str: Encode<'q, DB>,
{
    fn encode_by_ref(&self, buf: &mut <DB as HasArguments<'q>>::ArgumentBuffer) -> IsNull {
        let result = match *self {
            Self::Teacher => "teacher",
            Self::Student => "student",
        };

        <&str as Encode<'_, DB>>::encode_by_ref(&result, buf)
    }
}
