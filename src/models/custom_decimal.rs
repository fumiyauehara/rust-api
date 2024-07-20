use std::io::Write;
use std::str::FromStr;
use rust_decimal::Decimal;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql, Output};
use diesel::sql_types::Numeric;
use diesel::{AsExpression, FromSqlRow};
use diesel::mysql::{Mysql, MysqlValue};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Numeric)]
pub struct CustomDecimal(Decimal);

impl FromSql<Numeric, Mysql> for CustomDecimal {
    fn from_sql(mysql_value: MysqlValue) -> deserialize::Result<Self> {
        let string = std::str::from_utf8(mysql_value.as_bytes())?;
        Decimal::from_str(string).map(CustomDecimal).map_err(Into::into)
    }
}

impl ToSql<Numeric, Mysql> for CustomDecimal {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Mysql>) -> serialize::Result {
        let bytes = self.0.to_string().into_bytes();
        out.write_all(&bytes)?;
        Ok(serialize::IsNull::No)
    }
}