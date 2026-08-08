use crate::prelude::*;

pub struct CLEAR;

impl OrderCollector for CLEAR {
    fn collect_orders(&self, state: &TradeState) {
        if state.positions.borrow().is_empty() {
            for v in state.orders.borrow_mut().values_mut() {
                v.is_active = false;
            }
            for v in state.orders_storage.borrow_mut().values_mut() {
                v.0.is_active = false;
            }
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
        let trade_cell = TradeState::new(100.);
        trade_cell
            .positions
            .borrow_mut()
            .insert(1, Position::default());
        trade_cell
            .orders
            .borrow_mut()
            .insert("id_1", Order::default());
        let res = TradeState::new(100.);
        res.positions.borrow_mut().insert(1, Position::default());
        res.orders.borrow_mut().insert("id_1", Order::default());
        COLLECTOR.collect_orders(&trade_cell);
        assert_eq_pr!(&trade_cell, &res,);
        trade_cell.positions.borrow_mut().remove(&1);
        res.positions.borrow_mut().remove(&1);
        res.orders.borrow_mut().entry("id_1").and_modify(|o| {
            o.is_active = false;
        });
        COLLECTOR.collect_orders(&trade_cell);
        assert_eq_pr!(&trade_cell, &res,);
    }
}
