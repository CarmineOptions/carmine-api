use actix_web::web::{self};

mod common;
mod v1;
mod v2;

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
            web::scope("/api")
                // v1
                .service(
                    web::scope("/v1")
                        .service(v1::live_options)
                        .service(v1::transactions)
                        .service(v1::all_transactions)
                        .service(v1::all_legacy_transactions)
                        .service(v1::votes)
                        .service(v1::pail_token)
                        .service(v1::pail_events)
                        .service(v1::get_insurance_event_history)
                        .service(v1::get_insurance_users)
                        .service(v1::braavos_proscore)
                        .service(v1::airdrop)
                        .service(v1::defispring)
                        .service(v1::option_volatility)
                        .service(v1::get_referral_events)
                        .service(v1::get_referral)
                        .service(v1::referral_event)
                        .service(v1::insurance_event)
                        .service(v1::get_top_user_points)
                        .service(v1::get_user_points)
                        .service(v1::trades)
                        .service(v1::trades_with_prices)
                        .service(v1::token_prices)
                        .service(v1::pool_state)
                        .service(v1::pool_state_last)
                        .service(v1::pool_apy)
                        .service(v1::prices)
                        .service(v1::proxy_call),
                )
                // v2
                .service(web::scope("/v2").service(v2::pool_apy)),
        );

    conf.service(scope);
}
