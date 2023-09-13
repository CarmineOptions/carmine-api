use std::env;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tokio::time::{sleep, Duration};

use carmine_api_core::telegram_bot;
use carmine_api_starknet::{
    plug_holes_amm_state, update_database_amm_state, update_database_events,
};

const BLOCK_OFFSET: i64 = 5;
const PLUG_HOLES: bool = true;
const GET_NEW_BLOCKS: bool = true;
const GET_NEW_EVENTS: bool = true;

const LOCAL_IP: &str = "127.0.0.1";
const DOCKER_IP: &str = "0.0.0.0";

fn ip_address() -> &'static str {
    match env::var("ENVIRONMENT") {
        Ok(v) if v == "local" => LOCAL_IP,
        _ => DOCKER_IP,
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

    println!("🚀 Fetcher started successfully");

    HttpServer::new(|| App::new().service(liveness))
        .bind((ip_address(), 8080))?
        .run()
        .await
}
