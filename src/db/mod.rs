use crate::models::trade::{Trade, TradeClosure, TPSLUpdate};
use log::info;
use rusqlite::{params, Connection, Result};

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(include_str!("../../migrations/init.sql"))?;
    Ok(())
}

pub fn insert_master_trade(conn: &Connection, trade: &Trade) -> Result<i64, rusqlite::Error> {
    conn.execute(
        "INSERT INTO master_trades (master_account_id, ticket, symbol, trade_type, volume, open_price, open_time, close_price, close_time, profit, status, take_profit, stop_loss)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
        params![
            trade.master_account_id, trade.ticket, trade.symbol, trade.trade_type, trade.volume,
            trade.open_price, trade.open_time, trade.close_price, trade.close_time,
            trade.profit, trade.status, trade.take_profit, trade.stop_loss
        ],
    )?;

    Ok(conn.last_insert_rowid())
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
        AND (st.id IS NULL OR (mt.status = 'closed' AND st.status = 'open') OR mt.updated_at > st.updated_at)
        AND mt.created_at >= datetime('now', '-5 minutes')
    ";

    let mut stmt = conn.prepare(query)?;
    
    let trade_iter = stmt.query_map(params![master_account_id, slave_account_id], |row| {
        Ok(Trade {
            id: row.get(0)?,
            master_account_id: row.get(1)?,
            ticket: row.get(2)?,  // Add this line
            symbol: row.get(3)?,
            trade_type: row.get(4)?,
            volume: row.get(5)?,
            open_price: row.get(6)?,
            open_time: row.get(7)?,
            close_price: row.get(8)?,
            close_time: row.get(9)?,
            profit: row.get(10)?,
            status: row.get(11)?,
            take_profit: row.get(12)?,
            stop_loss: row.get(13)?,
        })
    })?;

    let trades: Result<Vec<Trade>, _> = trade_iter.collect();
    trades
}

pub fn check_foreign_keys(conn: &Connection) -> Result<bool> {
    let mut stmt = conn.prepare("PRAGMA foreign_keys")?;
    let foreign_keys_enabled: bool = stmt.query_row([], |row| row.get(0))?;
    info!("Foreign keys enabled: {}", foreign_keys_enabled);
    Ok(foreign_keys_enabled)
}

pub fn update_trade_tpsl(conn: &Connection, update: &TPSLUpdate) -> Result<(), rusqlite::Error> {
    conn.execute(
        "UPDATE master_trades SET take_profit = ?1, stop_loss = ?2 WHERE id = ?3 AND master_account_id = ?4",
        params![update.take_profit, update.stop_loss, update.server_id, update.master_account_id],
    )?;
    Ok(())
}

pub fn update_trade(conn: &Connection, id: i64, trade: &Trade) -> Result<(), rusqlite::Error> {
    conn.execute(
        "UPDATE master_trades SET 
        master_account_id = ?1, ticket = ?2, symbol = ?3, trade_type = ?4, volume = ?5, 
        open_price = ?6, open_time = ?7, close_price = ?8, close_time = ?9, profit = ?10, 
        status = ?11, take_profit = ?12, stop_loss = ?13
        WHERE id = ?14",
        params![
            trade.master_account_id,
            trade.ticket,  // Add this line
            trade.symbol,
            trade.trade_type,
            trade.volume,
            trade.open_price,
            trade.open_time,
            trade.close_price,
            trade.close_time,
            trade.profit,
            trade.status,
            trade.take_profit,
            trade.stop_loss,
            id
        ],
    )?;
    Ok(())
}

pub fn get_trade_by_server_id(conn: &Connection, server_id: i64) -> Result<Option<Trade>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM master_trades WHERE id = ?")?;
    
    let mut trade_iter = stmt.query_map(params![server_id], |row| {
        Ok(Trade {
            id: row.get(0)?,
            master_account_id: row.get(1)?,
            ticket: row.get(2)?,
            symbol: row.get(3)?,
            trade_type: row.get(4)?,
            volume: row.get(5)?,
            open_price: row.get(6)?,
            open_time: row.get(7)?,
            close_price: row.get(8)?,
            close_time: row.get(9)?,
            profit: row.get(10)?,
            status: row.get(11)?,
            take_profit: row.get(12)?,
            stop_loss: row.get(13)?,
        })
    })?;

    trade_iter.next().transpose()
}

pub fn get_trade_by_ticket(conn: &Connection, ticket: i64) -> Result<Option<Trade>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT * FROM master_trades WHERE ticket = ?1")?;
    
    let mut trade_iter = stmt.query_map(params![ticket], |row| {
        Ok(Trade {
            id: row.get(0)?,
            master_account_id: row.get(1)?,
            ticket: row.get(2)?,
            symbol: row.get(3)?,
            trade_type: row.get(4)?,
            volume: row.get(5)?,
            open_price: row.get(6)?,
            open_time: row.get(7)?,
            close_price: row.get(8)?,
            close_time: row.get(9)?,
            profit: row.get(10)?,
            status: row.get(11)?,
            take_profit: row.get(12)?,
            stop_loss: row.get(13)?,
        })
    })?;

    trade_iter.next().transpose()
}

pub fn close_trade(conn: &Connection, trade_id: i64, closure: &TradeClosure) -> Result<(), rusqlite::Error> {
    conn.execute(
        "UPDATE master_trades SET status = 'closed', close_price = ?, close_time = ?, profit = ? WHERE id = ?",
        params![closure.close_price, closure.close_time, closure.profit, trade_id],
    )?;
    Ok(())
}