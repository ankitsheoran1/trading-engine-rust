mod matching_engine;
// use std::sync::Arc;

use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{OrderType, Order};
// use std::time::Duration;
// use std::thread;



fn process_ask_order(engine: &mut MatchingEngine) {
    println!("Before processing orderbooks {:?}", engine.orderbooks);

    for orderbook in engine.orderbooks.values_mut() {
        for (&price, ask_limit) in &mut orderbook.asks {
            let mut orders_to_update = Vec::new();

            for (i, order) in ask_limit.orders.iter().enumerate() {
                match engine.clone().place_limit_order(TradingPair::new("BTC".to_string(), "USD".to_string()), price, order.clone()) {
                    Ok(updated_order) => {
                        orders_to_update.push((i, updated_order));
                    },
                    Err(e) => {
                        println!("Error placing limit order: {:?}", e);
                    }
                }
            }

            // Update the orders in the orderbook
            for (i, updated_order) in orders_to_update {
                ask_limit.orders[i] = updated_order;
            }
        }
    }

    println!("After processing orderbooks {:?}", engine.orderbooks);
}




fn main() {
    let mut engine = MatchingEngine::new();
    let btc_usd_pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(btc_usd_pair.clone());
    engine.add_order(btc_usd_pair.clone(), 8, Order::new(OrderType::Ask, "BTC".to_string(), 4)).unwrap();
    engine.add_order(btc_usd_pair.clone(), 9, Order::new(OrderType::Ask, "BTC".to_string(), 2)).unwrap();
    engine.add_order(btc_usd_pair.clone(), 10, Order::new(OrderType::Ask, "BTC".to_string(), 1)).unwrap();

    engine.add_order(btc_usd_pair.clone(), 9, Order::new(OrderType::Bid, "BTC".to_string(), 2)).unwrap();


    engine.add_order(btc_usd_pair.clone(), 9, Order::new(OrderType::Bid, "BTC".to_string(), 1)).unwrap();

    engine.add_order(btc_usd_pair.clone(), 10, Order::new(OrderType::Bid, "BTC".to_string(), 2)).unwrap();
    engine.add_order(btc_usd_pair.clone(), 11, Order::new(OrderType::Bid, "BTC".to_string(), 3)).unwrap();
    process_ask_order(&mut engine);
    engine.debug()
}


