CREATE TABLE IF NOT EXISTS master_trades (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    master_account_id INTEGER NOT NULL,
    symbol TEXT NOT NULL,
    trade_type TEXT NOT NULL,
    volume REAL NOT NULL,
    open_price REAL NOT NULL,
    open_time TEXT NOT NULL,
    close_price REAL,
    close_time TEXT,
    profit REAL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS slave_trades (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    master_trade_id INTEGER NOT NULL,
    slave_account_id INTEGER NOT NULL,
    copied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (master_trade_id) REFERENCES master_trades(id)
);