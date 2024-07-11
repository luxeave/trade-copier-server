use crate::models::trade::Trade;
use log::{debug, info};
use rusqlite::{params, Connection, Result};

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(include_str!("../../migrations/init.sql"))?;
    Ok(())
}

pub fn insert_master_trade(conn: &Connection, trade: &Trade) -> Result<i64> {
    let _ = conn.execute(
        "INSERT INTO master_trades (master_account_id, symbol, trade_type, volume, open_price, open_time, close_price, close_time, profit)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            &trade.master_account_id,
            &trade.symbol,
            &trade.trade_type,
            &trade.volume,
            &trade.open_price,
            &trade.open_time,
            &trade.close_price,
            &trade.close_time,
            &trade.profit,
        ),
    )?;

    let id = conn.last_insert_rowid();
    debug!("Inserted new master trade with ID: {}", id);

    Ok(id)
}

pub fn get_new_trades_for_slave(
    conn: &Connection,
    slave_account_id: i64,
    master_account_id: i64,
) -> Result<Vec<Trade>, rusqlite::Error> {
    let query = "
        SELECT mt.* 
        FROM master_trades mt
        WHERE mt.master_account_id = ?1
        AND mt.id NOT IN (
            SELECT master_trade_id 
            FROM slave_trades 
            WHERE slave_account_id = ?2
        )
        AND mt.created_at >= datetime('now', '-5 minutes')
    ";

    let mut stmt = conn.prepare(query)?;
    let trade_iter = stmt.query_map(params![master_account_id, slave_account_id], |row| {
        Ok(Trade {
            id: row.get(0)?,
            master_account_id: row.get(1)?,
            symbol: row.get(2)?,
            trade_type: row.get(3)?,
            volume: row.get(4)?,
            open_price: row.get(5)?,
            open_time: row.get(6)?,
            close_price: row.get(7)?,
            close_time: row.get(8)?,
            profit: row.get(9)?,
        })
    })?;

    let trades: Result<Vec<Trade>, rusqlite::Error> = trade_iter.collect();
    let trades = trades?;

    // Record these trades as shown to the slave
    for trade in &trades {
        conn.execute(
            "INSERT INTO slave_trades (master_trade_id, slave_account_id) VALUES (?1, ?2)",
            params![trade.id, slave_account_id],
        )?;
    }

    Ok(trades)
}

pub fn check_foreign_keys(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("PRAGMA foreign_keys")?;
    let foreign_keys_enabled: bool = stmt.query_row([], |row| row.get(0))?;
    info!("Foreign keys enabled: {}", foreign_keys_enabled);
    Ok(foreign_keys_enabled)
}