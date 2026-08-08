use std::any::Any;

use bc_utils_lg::structs::trade::TradeState;

pub trait OrderCollector: Any {
    fn collect_orders(&self, state: &TradeState);
}
