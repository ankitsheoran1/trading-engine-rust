use std::collections::HashMap;
use super::orderbook::{Order, Orderbook};


#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct TradingPair {
    base: String,
    quote: String,
}

impl TradingPair {
  pub fn new(base: String, quote: String) -> TradingPair {
      TradingPair { base, quote }
  }

  pub fn to_string(self) -> String {
      format!("{}_{}", self.base, self.quote)
  }
}

#[derive(Clone)]

pub  struct MatchingEngine {
  pub orderbooks: HashMap<TradingPair, Orderbook>
  
}

impl MatchingEngine {
  pub fn new() -> MatchingEngine {
    MatchingEngine {
      orderbooks: HashMap::new(),
    }
  }

  pub fn add_new_market(&mut self, pair: TradingPair) {
    self.orderbooks.insert(pair.clone(), Orderbook::new());
  }

  pub fn place_limit_order(&mut self, pair: TradingPair, price: u32, mut order: Order) -> Result<Order, String> {
    match self.orderbooks.get_mut(&pair) {
      Some(orderbook) => {
        orderbook.fill_order(&mut order, price);
        println!("placed limit order at price level {}", price);
        Ok(order)
      }
      None => Err(format!( 
        "the orderbook for the given trading pair ({}) does not exist",
        pair.to_string())),
    }
  }

  pub fn add_order(&mut self, pair: TradingPair, price: u32, order: Order) -> Result<(), String> {
    if let Some(orderbook) = self.orderbooks.get_mut(&pair) {
        orderbook.add_order(price, order, pair.base.clone());
        Ok(())
    } else {
        Err("Market is not present".to_string())
    }
 }

 pub fn debug(&self) {
  for (_, orderbook) in &self.orderbooks {
      for (_, limit) in &orderbook.asks {
          println!("ask orderbook {:?}", orderbook);
          for order in &limit.orders {
              println!("ask orders are {:?}", order);
          }
      }
  }

  for (_, orderbook) in &self.orderbooks {
      for (_, limit) in &orderbook.bids {
          println!("bid orderbook {:?}", orderbook);
          for order in &limit.orders {
              println!("bid orders are {:?}", order);
          }
      }
  }
}
}