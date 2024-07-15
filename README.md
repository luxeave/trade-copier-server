# Multi-Account Trade Copier System

This system allows for copying trades from one or more master accounts to a slave account using MetaTrader 4 and a web server backend.

## Components

1. Master EA (master_v4.mq4)
2. Slave EA (slave_v10.mq4)
3. Web Server (Rust-based)
4. Database (SQLite)

## Features

- Copy trades from master account(s) to slave account
- Support for multiple master accounts by running multiple instances of the Slave EA
- Dynamic lot size calculation options
- Trade modification and closure synchronization
- Blacklisting mechanism to prevent reopening manually closed trades

## Setup

### Master Account

1. Attach the Master EA (master_v4.mq4) to any chart in the MT4 terminal of your master account.
2. Configure the EA with the appropriate server URL and master account ID.

### Slave Account

1. For each master account you want to copy from, open a new chart in the MT4 terminal of your slave account.
2. Attach the Slave EA (slave_v10.mq4) to each of these charts.
3. Configure each instance of the Slave EA with:
   - The server URL
   - The slave account ID (same for all instances)
   - A unique master account ID for each instance
   - Desired lot size mode and risk parameters

### Web Server Setup and Build Instructions

Prerequisites:
- Rust programming language (latest stable version)
- Cargo (Rust's package manager, usually comes with Rust)
- SQLite3

Setup and Build:
1. Clone the repository:
```
git clone [your-repository-url]
cd [repository-name]
```
2. Navigate to the server directory:
```
cd [path-to-server-directory]
```
3. Create a `.env` file in the server directory with the following content:
```
DATABASE_URL=trade_copier.db
SERVER_ADDR=127.0.0.1:8080
```
Adjust these values as needed for your environment.

4. Build the project:
```
cargo build --release
```
5. Run database migrations (if applicable):
```
cargo run --bin migrate
```
Note: Ensure you have a migration script or process set up.

6. Start the server:
```
cargo run --release
```
The server should now be running and listening on the specified address.

Customization:
- To change the server port or address, modify the `SERVER_ADDR` in the `.env` file.
- To use a different database file, change the `DATABASE_URL` in the `.env` file.

Troubleshooting:
- If you encounter any build errors, ensure your Rust toolchain is up to date:
```
rustup update
```
- For database connection issues, check that SQLite3 is properly installed and the database file is accessible.

## Usage

1. Start the web server.
2. Run the Master EA on the master account(s).
3. Run the Slave EA instances on separate charts in the slave account.
4. The system will automatically copy trades, modifications, and closures from each master to the slave.

## Configuration Options

### Slave EA

- `ServerURL`: URL of the web server
- `SlaveAccountID`: ID of the slave account
- `MasterAccountID`: ID of the master account to copy from (unique for each instance)
- `CheckInterval`: How often to check for new trades (in seconds)
- `LotSizeMode`: Choose between master's lot size, fixed lot size, or dynamic lot calculation
- `lot_fix`: Fixed lot size (if chosen)
- `risk_percent`: Risk percentage for dynamic lot calculation
- `sl_pips`: Stop loss in pips for dynamic lot calculation

## Notes

- Ensure your slave account has sufficient margin to handle trades from all master accounts.
- Be aware of potential conflicts if different masters open opposing trades on the same instrument.
- Consider adjusting lot sizes or risk parameters for each Slave EA instance to manage overall risk.
- The system includes a blacklisting mechanism to prevent reopening of manually closed trades.

## Troubleshooting

- Check MT4 journal logs for any error messages.
- Verify server logs for request/response issues.
- Ensure proper network connectivity between MT4 terminals and the web server.