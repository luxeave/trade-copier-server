use crate::models::trade::{Trade, TradeClosure};
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
        LEFT JOIN slave_trades st ON mt.id = st.master_trade_id AND st.slave_account_id = ?2
        WHERE mt.master_account_id = ?1
        AND (st.id IS NULL OR (mt.status = 'closed' AND st.status = 'open'))
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
            status: row.get(10)?,
        })
    })?;

    let trades: Result<Vec<Trade>, rusqlite::Error> = trade_iter.collect();
    let trades = trades?;

    // Record these trades as shown to the slave
    for trade in &trades {
        if trade.status == "open" {
            conn.execute(
                "INSERT INTO slave_trades (master_trade_id, slave_account_id, status) VALUES (?1, ?2, 'open')",
                params![trade.id, slave_account_id],
            )?;
        } else if trade.status == "closed" {
            conn.execute(
                "UPDATE slave_trades SET status = 'closed' WHERE master_trade_id = ?1 AND slave_account_id = ?2",
                params![trade.id, slave_account_id],
            )?;
        }
    }

    Ok(trades)
}

pub fn check_foreign_keys(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("PRAGMA foreign_keys")?;
    let foreign_keys_enabled: bool = stmt.query_row([], |row| row.get(0))?;
    info!("Foreign keys enabled: {}", foreign_keys_enabled);
    Ok(foreign_keys_enabled)
}

pub fn insert_trade_closure(conn: &Connection, closure: &TradeClosure) -> Result<(), rusqlite::Error> {
    // First, check if the master trade exists
    let master_trade_exists: bool = conn.query_row(
        "SELECT 1 FROM master_trades WHERE id = ? AND master_account_id = ?",
        params![closure.ticket, closure.master_account_id],
        |_| Ok(true)
    ).unwrap_or(false);

    if !master_trade_exists {
        log::error!("Master trade not found for ticket: {} and account: {}", closure.ticket, closure.master_account_id);
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    conn.execute(
        "UPDATE master_trades SET status = 'closed', close_price = ?1, close_time = ?2, profit = ?3 WHERE id = ?4 AND master_account_id = ?5",
        params![closure.close_price, closure.close_time, closure.profit, closure.ticket, closure.master_account_id],
    ).map_err(|e| {
        log::error!("Failed to update master_trades: {:?}", e);
        e
    })?;

    conn.execute(
        "INSERT INTO trade_closures (master_trade_id, close_price, close_time, profit) VALUES (?1, ?2, ?3, ?4)",
        params![closure.ticket, closure.close_price, closure.close_time, closure.profit],
    ).map_err(|e| {
        log::error!("Failed to insert into trade_closures: {:?}", e);
        e
    })?;

    Ok(())
}
