use crate::prelude::*;
use std::collections::hash_map::ValuesMut;

pub struct CLEAR;

fn order_set_is_active(values: ValuesMut<String, Order>) {
    for order in values {
        order.is_active = false;
    }
}

impl OrderCollector for CLEAR {
    fn collect_orders(
        &self,
        cell: &TradeCell,
    ) {
        if cell.positions.borrow().is_empty() {
            order_set_is_active(cell.trigger_orders.borrow_mut().values_mut());
            order_set_is_active(cell.limit_orders.borrow_mut().values_mut());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq as assert_eq_pr;

    use crate::prelude_tests::prelude::*;

    static COLLECTOR: LazyLock<CLEAR> = LazyLock::new(|| CLEAR);

    #[test]
    fn collect_orders_res_1() {
        let trade_cell = TradeCell::new(100., vec![1.], vec![2.]);
        trade_cell
            .positions
            .borrow_mut()
            .insert("1".to_string(), Position::default());
        trade_cell
            .limit_orders
            .borrow_mut()
            .insert("id_1".to_string(), Order::default());
        let res = TradeCell::new(100., vec![1.], vec![2.]);
        res.positions
            .borrow_mut()
            .insert("1".to_string(), Position::default());
        res.limit_orders
            .borrow_mut()
            .insert("id_1".to_string(), Order::default());
        COLLECTOR.collect_orders(&trade_cell);
        assert_eq_pr!(&trade_cell, &res,);
        trade_cell.positions.borrow_mut().remove("1");
        res.positions.borrow_mut().remove("1");
        res.limit_orders
            .borrow_mut()
            .entry("id_1".to_string())
            .and_modify(|o| o.set_is_active(false));
        COLLECTOR.collect_orders(&trade_cell);
        assert_eq_pr!(&trade_cell, &res,);
    }
}
