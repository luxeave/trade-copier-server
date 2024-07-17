PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS master_trades (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    master_account_id INTEGER NOT NULL,
    ticket INTEGER NOT NULL,
    master_ticket INTEGER NOT NULL,
    symbol TEXT NOT NULL,
    trade_type TEXT NOT NULL,
    volume REAL NOT NULL,
    open_price REAL NOT NULL,
    open_time DATETIME NOT NULL,
    close_price REAL,
    close_time DATETIME,
    profit REAL,
    status TEXT NOT NULL,
    take_profit REAL,
    stop_loss REAL,
    expiration DATETIME,  
    created_at DATETIME NOT NULL DEFAULT (datetime('now')),
    updated_at DATETIME NOT NULL DEFAULT (datetime('now'))
);
CREATE TABLE IF NOT EXISTS slave_trades (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    master_trade_id INTEGER NOT NULL,
    slave_account_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (master_trade_id) REFERENCES master_trades(id)
);

CREATE TABLE IF NOT EXISTS trade_closures (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    master_trade_id INTEGER NOT NULL,
    close_price REAL NOT NULL,
    close_time TEXT NOT NULL,
    profit REAL NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (master_trade_id) REFERENCES master_trades(id)
);

CREATE INDEX IF NOT EXISTS idx_master_trades_account_id ON master_trades(master_account_id);
CREATE INDEX IF NOT EXISTS idx_slave_trades_master_trade_id ON slave_trades(master_trade_id);
CREATE INDEX IF NOT EXISTS idx_slave_trades_slave_account_id ON slave_trades(slave_account_id);
CREATE INDEX IF NOT EXISTS idx_trade_closures_master_trade_id ON trade_closures(master_trade_id);

-- Add trigger to automatically update the updated_at field
CREATE TRIGGER IF NOT EXISTS update_master_trades_timestamp 
AFTER UPDATE ON master_trades
FOR EACH ROW
BEGIN
    UPDATE master_trades SET updated_at = datetime('now') WHERE id = NEW.id;
END;