use serde::{Deserialize, Serialize, Deserializer};
use chrono::{DateTime, Utc};
use rusqlite::types::{ToSql, FromSql, ValueRef, ToSqlOutput};
use chrono::NaiveDateTime;
use std::str::FromStr;

#[derive(Debug)]
pub struct DateTimeWrapper(pub DateTime<Utc>);

impl FromStr for DateTimeWrapper {
    type Err = chrono::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let naive = NaiveDateTime::parse_from_str(s, "%Y.%m.%d %H:%M:%S")?;
        Ok(DateTimeWrapper(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc)))
    }
}

impl ToSql for DateTimeWrapper {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        let s = self.0.format("%Y-%m-%d %H:%M:%S").to_string();
        Ok(ToSqlOutput::from(s))
    }
}

impl FromSql for DateTimeWrapper {
    fn column_result(value: ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        value.as_str().and_then(|s| {
            NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                .map(|ndt| DateTimeWrapper(DateTime::<Utc>::from_naive_utc_and_offset(ndt, Utc)))
                .map_err(|e| rusqlite::types::FromSqlError::Other(Box::new(e)))
        })
    }
}
// Implement Serialize and Deserialize for DateTimeWrapper
impl Serialize for DateTimeWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DateTimeWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    pub id: Option<i64>,
    pub master_account_id: i64,
    pub ticket: i64,
    pub master_ticket: i64,
    pub symbol: String,
    pub trade_type: String,
    pub volume: f64,
    pub open_price: f64,
    pub open_time: DateTimeWrapper,
    pub close_price: Option<f64>,
    pub close_time: Option<DateTimeWrapper>,
    pub profit: Option<f64>,
    pub status: String,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub expiration: Option<DateTimeWrapper>,
    #[serde(skip_deserializing)]
    pub created_at: Option<DateTimeWrapper>,
    #[serde(skip_deserializing)]
    pub updated_at: Option<DateTimeWrapper>,
}
#[derive(Debug, Deserialize)]
pub struct SlaveInfo {
    pub slave_account_id: i64,
    pub master_account_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeClosure {
    pub master_account_id: i64,
    pub ticket: i64,
    pub server_id: i64,
    pub symbol: String,
    pub close_price: f64,
    pub close_time: DateTimeWrapper,
    pub profit: f64,
}

#[derive(Debug, Deserialize)]
pub struct TPSLUpdate {
    pub master_account_id: i64,
    pub server_id: i64,
    pub take_profit: f64,
    pub stop_loss: f64,
}