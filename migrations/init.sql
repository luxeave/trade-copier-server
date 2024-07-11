-- Create master_trades table
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
    status TEXT CHECK(status IN ('open', 'closed')) DEFAULT 'open',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create slave_trades table
CREATE TABLE IF NOT EXISTS slave_trades (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    master_trade_id INTEGER NOT NULL,
    slave_account_id INTEGER NOT NULL,
    status TEXT CHECK(status IN ('open', 'closed')) DEFAULT 'open',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (master_trade_id) REFERENCES master_trades(id)
);

-- Create trade_closures table
CREATE TABLE IF NOT EXISTS trade_closures (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    master_trade_id INTEGER NOT NULL,
    close_price REAL NOT NULL,
    close_time TEXT NOT NULL,
    profit REAL NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (master_trade_id) REFERENCES master_trades(id)
);

-- Create index on master_account_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_master_trades_account_id ON master_trades(master_account_id);

-- Create index on master_trade_id and slave_account_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_slave_trades_master_slave ON slave_trades(master_trade_id, slave_account_id);

-- Create index on master_trade_id for trade_closures
CREATE INDEX IF NOT EXISTS idx_trade_closures_master_trade_id ON trade_closures(master_trade_id);