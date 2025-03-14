use std::env;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use carmine_api_db::lp_value::update_lp_prices;
use carmine_api_fetcher::braavos::{update_braavos_proscore, update_braavos_referrals};
use carmine_api_rpc_gateway::{blast_api_latest_block_number, carmine_latest_block_number};
use tokio::time::{sleep, Duration};

use carmine_api_core::telegram_bot;
use carmine_api_starknet::{
    plug_holes_amm_state, update_database_amm_state, update_database_events,
};

const BLOCK_OFFSET: i64 = 10;
const PLUG_HOLES: bool = true;
const GET_NEW_BLOCKS: bool = true;
const GET_NEW_EVENTS: bool = true;
const UPDATE_POOL_PRICES: bool = true;
const BRAAVOS_PROSCORE: bool = true;
const BRAAVOS_REFERRAL: bool = true;

const BLOCK_DISCREPENCY_THRESHOLD: i64 = 5;

const LOCAL_IP: &str = "127.0.0.1";
const DOCKER_IP: &str = "0.0.0.0";

fn ip_address() -> &'static str {
    match env::var("ENVIRONMENT") {
        Ok(v) if v == "local" => LOCAL_IP,
        _ => DOCKER_IP,
    }
}

async fn report_block_discrepency() {
    let (carm_block_number, blast_block_number) = match (
        carmine_latest_block_number().await,
        blast_api_latest_block_number().await,
    ) {
        (Ok(carm), Ok(blast)) => (carm, blast),
        (Err(_), _) => {
            // carmine failed, report it
            telegram_bot::send_message("Failed getting latest block number from Carmine Juno node")
                .await;
            return;
        }
        // blast failed, but carmine ok - do not report
        _ => return,
    };

    let diff = (blast_block_number - carm_block_number).abs();
    if diff > BLOCK_DISCREPENCY_THRESHOLD {
        let msg = format!(
            "BLOCK DISCREPENCY is {}: Carmine: {}, BlastApi: {}",
            diff, carm_block_number, blast_block_number
        );

        telegram_bot::send_message(msg.as_str()).await;
    }
}

#[get("/")]
async fn liveness() -> impl Responder {
    HttpResponse::Ok().body("alive")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("👷 Starting fetcher");

    if GET_NEW_EVENTS {
        println!("🛠️  Spawning event fetching thread...");
        actix_web::rt::spawn(async move {
            loop {
                if let Err(err) =
                    actix_web::rt::spawn(async { update_database_events().await }).await
                {
                    // failed, probably network overload, wait to send message
                    sleep(Duration::from_secs(10)).await;
                    println!("update_database_events panicked\n{:?}", err);
                    telegram_bot::send_message(
                        "Carmine API `update_database_events` just panicked",
                    )
                    .await;
                } else {
                    println!("Database updated with events");
                }
                sleep(Duration::from_secs(150)).await;
            }
        });
    }

    if GET_NEW_BLOCKS {
        println!("🛠️  Spawning new blocks fetching thread...");
        actix_web::rt::spawn(async move {
            loop {
                report_block_discrepency().await;

                if let Err(err) =
                    actix_web::rt::spawn(async { update_database_amm_state(BLOCK_OFFSET).await })
                        .await
                {
                    // failed, probably network overload, wait to send message
                    sleep(Duration::from_secs(100)).await;
                    println!("Update database amm state panicked\n{:?}", err);
                    telegram_bot::send_message(
                        "Carmine API `update_database_amm_state` just panicked",
                    )
                    .await;
                } else {
                    println!("Database updated with AMM state");
                }
                sleep(Duration::from_secs(150)).await;
            }
        });
    }

    if PLUG_HOLES {
        println!("🛠️  Spawning hole plugging thread...");
        actix_web::rt::spawn(async move {
            loop {
                if let Err(err) = actix_web::rt::spawn(async { plug_holes_amm_state().await }).await
                {
                    // failed, probably network overload, wait to send message
                    sleep(Duration::from_secs(120)).await;
                    println!("Plug holes panicked\n{:?}", err);
                    telegram_bot::send_message("Carmine API `plug_holes_amm_state` just panicked")
                        .await;
                } else {
                    println!("Holes in AMM state pluged");
                }
                sleep(Duration::from_secs(150)).await;
            }
        });
    }

    if UPDATE_POOL_PRICES {
        println!("🛠️  Spawning LP price updating thread...");
        actix_web::rt::spawn(async move {
            loop {
                if let Err(err) = actix_web::rt::spawn(async move {
                    update_lp_prices();
                    Ok::<(), ()>(())
                })
                .await
                {
                    // failed, probably network overload, wait to send message
                    sleep(Duration::from_secs(120)).await;
                    println!("Update pool prices panicked\n{:?}", err);
                    telegram_bot::send_message("Carmine API `update_lp_prices` just panicked")
                        .await;
                } else {
                    println!("LP prices updated");
                }
                sleep(Duration::from_secs(150)).await;
            }
        });
    }

    if BRAAVOS_PROSCORE {
        println!("🛠️  Spawning Braavos proscore updating thread...");
        actix_web::rt::spawn(async move {
            loop {
                if let Err(err) =
                    actix_web::rt::spawn(async { update_braavos_proscore().await }).await
                {
                    // failed, probably network overload, wait to send message
                    sleep(Duration::from_secs(120)).await;
                    println!("Update Braavos proscore panicked\n{:?}", err);
                    telegram_bot::send_message(
                        "Carmine API `update_braavos_proscore` just panicked",
                    )
                    .await;
                } else {
                    println!("Braavos proscore updated");
                }
                sleep(Duration::from_secs(150)).await;
            }
        });
    }

    if BRAAVOS_REFERRAL {
        println!("🛠️  Spawning Braavos referral updating thread...");
        actix_web::rt::spawn(async move {
            loop {
                if let Err(err) = actix_web::rt::spawn(async move {
                    update_braavos_referrals();
                    Ok::<(), ()>(())
                })
                .await
                {
                    // failed, probably network overload, wait to send message
                    sleep(Duration::from_secs(120)).await;
                    println!("Update braavos referral panicked\n{:?}", err);
                    telegram_bot::send_message(
                        "Carmine API `update_braavos_referrals` just panicked",
                    )
                    .await;
                } else {
                    println!("Braavos referrals updated");
                }
                sleep(Duration::from_secs(150)).await;
            }
        });
    }

    println!("🚀 Fetcher started successfully");

    HttpServer::new(|| App::new().service(liveness))
        .bind((ip_address(), 8080))?
        .run()
        .await
}
