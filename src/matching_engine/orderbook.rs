use std::collections::HashMap;
use std::io::Error;

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum OrderType {
    Bid,
    Ask,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
  orderType: OrderType,
  symbol: String,
  size: u32
}

#[derive(Debug, Clone)]
pub struct Limit {
    price: u32,
    symbol: String,
    pub orders: Vec<Order>,
}

#[derive(Debug, Clone)]
pub struct Orderbook {
    pub asks: HashMap<u32, Limit>,
    pub bids: HashMap<u32, Limit>,
}


impl Order {
    pub fn new(orderType: OrderType, symbol: String, size: u32) -> Self {
        Order {
            orderType,
            symbol,
            size,
        }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0
    } 
}

impl Limit {
    fn new(price: u32,  symbol: String) -> Self {
        Limit {
            price,
            symbol,
            orders: Vec::new(),
        }

    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }

    fn fill_order(&mut self, order: &mut Order) -> Result<Order, Error> {
        for limit_order in self.orders.iter_mut() {
            match order.size >= limit_order.size {
                true => {
                    order.size -= limit_order.size;
                    limit_order.size = 0
                }
                false => {
                    limit_order.size -= order.size;
                    order.size = 0
                }
            }

            if order.is_filled() {
                return Ok(order.clone())
            }
        }
        Ok(order.clone())
    }

    
}

impl Orderbook {
    pub fn new() -> Self {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn fill_order(&mut self, order: &mut Order, price: u32) -> Result<Order, Error> {
        match order.orderType {
            OrderType::Bid => {
                for (key, value) in &mut self.asks {
                    if *key >= price {
                        match value.fill_order(order) {
                            Ok(order) => {
                               if order.is_filled() {
                                 return Ok(order);
                               }
                            }
                            Err(e) => {
                                return Err(e);

                            }
                        }
                    }
                }
                return Ok(order.clone())

            }
            OrderType::Ask => {
                for (key, value) in &mut self.bids {
                    if *key >= price {
                        match value.fill_order(order) {
                            Ok(order) => {
                               if order.is_filled() {
                                 return Ok(order);
                               }
                            }
                            Err(e) => {
                                return Err(e);

                            }
                        }
                    }
                }
                return Ok(order.clone())
            }
        }
    }

    pub fn add_order(&mut self, price: u32, order: Order, symbol: String) {

        match order.orderType {
            OrderType::Bid => match self.bids.get_mut(&price) {
                Some(limit) => limit.add_order(order),
                None => {
                    let mut limit = Limit::new(price, symbol);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
            OrderType::Ask => match self.asks.get_mut(&price) {
                Some(limit) => limit.add_order(order),
                None => {
                    let mut limit = Limit::new(price, symbol);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
        }
    }


}
