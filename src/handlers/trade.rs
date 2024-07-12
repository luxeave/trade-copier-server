use actix_web::{web, HttpResponse, Responder};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use crate::models::trade::{Trade, SlaveInfo, TradeClosure, TPSLUpdate};
use crate::db;
use crate::errors::{ApiError, internal_error};

type DbPool = Pool<SqliteConnectionManager>;

pub async fn get_new_trades_for_slave(
    slave_info: web::Json<SlaveInfo>,
    pool: web::Data<DbPool>,
) -> Result<impl Responder, ApiError> {
    let conn = pool.get()?;
    let trades = db::get_new_trades_for_slave(&conn, slave_info.slave_account_id, slave_info.master_account_id)?;
    Ok(HttpResponse::Ok().json(trades))
}

pub async fn close_trade(closure: web::Json<TradeClosure>, pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let conn = pool.get().map_err(|e| {
        log::error!("Failed to get database connection: {:?}", e);
        internal_error(e)
    })?;
    
    log::info!("Received trade closure request: {:?}", closure);

    // Try to find the trade by server_id first
    let trade = db::get_trade_by_server_id(&conn, closure.server_id)?;

    // If not found, try to find by ticket
    let trade = match trade {
        Some(t) => t,
        None => {
            log::warn!("Trade not found by server_id: {}. Trying ticket.", closure.server_id);
            db::get_trade_by_ticket(&conn, closure.ticket)?
                .ok_or_else(|| {
                    log::error!("Trade not found for server_id: {} and ticket: {}", closure.server_id, closure.ticket);
                    ApiError::InternalServerError(format!("Trade not found for server_id: {} and ticket: {}", closure.server_id, closure.ticket))
                })?
        }
    };

    // Unwrap the trade.id, or return an error if it's None
    let trade_id = trade.id.ok_or_else(|| {
        log::error!("Trade found but has no ID. Server ID: {}, Ticket: {}", closure.server_id, closure.ticket);
        ApiError::InternalServerError("Trade found but has no ID".to_string())
    })?;

    match db::close_trade(&conn, trade_id, &closure) {
        Ok(_) => {
            log::info!("Trade closed successfully: {:?}", closure);
            Ok(HttpResponse::Ok().json("Trade closed successfully"))
        },
        Err(e) => {
            log::error!("Failed to close trade: {:?}", e);
            Err(internal_error(e))
        }
    }
}

pub async fn update_tpsl(update: web::Json<TPSLUpdate>, pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let conn = pool.get().map_err(ApiError::PoolError)?;
    db::update_trade_tpsl(&conn, &update).map_err(ApiError::DatabaseError)?;
    Ok(HttpResponse::Ok().json("TP/SL updated successfully"))
}

pub async fn add_or_update_trade(trade: web::Json<Trade>, pool: web::Data<DbPool>) -> Result<impl Responder, ApiError> {
    let conn = pool.get().map_err(ApiError::PoolError)?;
    
    let existing_trade = db::get_trade_by_ticket(&conn, trade.ticket)
        .map_err(ApiError::DatabaseError)?;
    
    match existing_trade {
        Some(existing) => {
            db::update_trade(&conn, existing.id.unwrap(), &trade).map_err(ApiError::DatabaseError)?;
            Ok(HttpResponse::Ok().json("Trade updated successfully"))
        },
        None => {
            let trade_id = db::insert_master_trade(&conn, &trade).map_err(ApiError::DatabaseError)?;
            Ok(HttpResponse::Ok().json(trade_id))
        },
    }
}