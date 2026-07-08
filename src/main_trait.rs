use std::any::Any;

use bc_utils_lg::structs::trade::TradeCell;

pub trait OrderCollector: Any {
    fn collect_orders(
        &self,
        cell: &TradeCell,
    );
}
