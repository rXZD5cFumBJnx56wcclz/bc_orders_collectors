use crate::prelude::*;
use std::collections::hash_map::ValuesMut;

pub struct CLEAR;

// impl CLEAR {
//     pub fn new() -> Self;
// }

fn order_set_is_active(values: ValuesMut<String, Order>) {
    for order in values {
        order.is_active = false;
    }
}

impl OrderCollector for CLEAR {
    fn collect_orders(&self, cell: &TradeCell) {
        if cell.positions.borrow().is_empty() {
            order_set_is_active(cell.trigger_orders.borrow_mut().values_mut());
            order_set_is_active(cell.limit_orders.borrow_mut().values_mut());
        }
    }
}
