use actix_web::{web, HttpResponse, Responder};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use crate::models::trade::{Trade, SlaveInfo};
use crate::db;
use crate::errors::{ApiError, internal_error};

type DbPool = Pool<SqliteConnectionManager>;

pub async fn add_trade(trade: web::Json<Trade>, pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let conn = pool.get().map_err(|e| internal_error(e))?;
    let trade_id = db::insert_master_trade(&conn, &trade).map_err(|e| internal_error(e))?;
    Ok(HttpResponse::Ok().json(trade_id))
}

pub async fn get_new_trades_for_slave(
    slave_info: web::Json<SlaveInfo>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get()?;
    let trades = db::get_new_trades_for_slave(&conn, slave_info.slave_account_id, slave_info.master_account_id)?;
    Ok(HttpResponse::Ok().json(trades))
}