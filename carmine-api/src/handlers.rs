use actix_web::web;

mod common;
mod v0;
mod v1;

pub fn format_tx(tx: &String) -> String {
    if tx.len() <= 3 || &tx[..2] != "0x" {
        // len 3 is 0x0 -> do not remove this zero
        return tx.to_string();
    }
    let tmp: String = tx.to_lowercase().chars().skip(2).collect();
    let without_leading_zeroes = tmp.trim_start_matches('0');
    let res = format!("0x{}", without_leading_zeroes);
    match res.len() {
        // 0x0000 -> 0x -> return 0x0
        2 => "0x0".to_string(),
        _ => res,
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("")
        .service(common::liveness_probe_handler)
        .service(
            web::scope("api")
                .service(v1::live_options)
                .service(v1::transactions)
                .service(v1::all_transactions)
                .service(v1::airdrop)
                .service(v0::all_non_expired_handler)
                .service(v0::trade_history_handler)
                .service(v0::all_trade_history_handler),
        );

    conf.service(scope);
}
