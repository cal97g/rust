extern crate binance;

use binance::websockets::*;
use std::sync::atomic::AtomicBool;

impl FromStr

fn main() {
    let keep_running = AtomicBool::new(true); // Used to control the event loop
    let agg_trade: String = format!("!ticker@arr");
    let mut web_socket: WebSockets = WebSockets::new(|event: WebsocketEvent| {
        match event {
            WebsocketEvent::DayTicker(ticker_events) => {
                for tick_event in ticker_events {
                    if tick_event.symbol == "BTCUSDT" {
                        let btcusdt: binance::model::TradesEvent = tick_event.average_price.parse().unwrap();
                        let btcusdt_close: f32 = tick_event.current_close.parse().unwrap();
                        println!("{} - {}", btcusdt, btcusdt_close);
                    }
                }
            },
            _ => (),
        };

        Ok(())
    });

    web_socket.connect(&agg_trade).unwrap(); // check error
    if let Err(e) = web_socket.event_loop(&keep_running) {
        match e {
            err => {
               println!("Error: {}", err);
            }
        }
    }
}
