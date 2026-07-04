use pretty_assertions::assert_eq as assert_eq_pr;

use bc_orders_collectors::clear::CLEAR;

use crate::unit::prelude::*;

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
