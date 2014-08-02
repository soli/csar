use super::{Model, Var, Domain, IntervalDomain, IntervalDom, BitDomain};

use std::cell::RefCell;

#[test]
fn creates_new_var() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 255, "x");
    assert_eq!((x.id, x.min(), x.max()), (0, -2, 255));
    assert_eq!(m.clone().vars.borrow().len(), 1);
    let y = Var::new(m.clone(), -2, 255, "y");
    assert_eq!(y.id, 1);
    assert_eq!(m.clone().vars.borrow().len(), 2);
}

fn min_is_min(d: &IntervalDomain) -> bool {
    match d.dom.borrow().intervals.get(0) {
        &(x, _) => x == d.get_min()
    }
}

fn max_is_max(d: &IntervalDomain) -> bool {
    match d.dom.borrow().intervals.last() {
        Some(&(_, y)) => y == d.get_max(),
        _             => false
    }
}

fn setup_domain_simple() -> IntervalDomain {
    IntervalDomain {
        dom: RefCell::new(IntervalDom {
                 min: -3,
                 max: 64,
                 intervals: vec![(-3, 2), (4, 42), (54, 64)]
             })
    }
}

fn intervals_bounds_are_coherent(d: &IntervalDomain) {
    assert!(min_is_min(d));
    assert!(max_is_max(d));
}

#[test]
fn sets_min_lower() {
    let d = setup_domain_simple();
    d.set_min(-4);
    assert_eq!(d.get_min(), -3);
    intervals_bounds_are_coherent(&d);
}

#[test]
fn sets_min_middle() {
    let d = setup_domain_simple();
    let values = [-2, 8, 42, 54, 64];
    let lengths = [3, 2, 2, 1, 1];
    let mut v : int;
    for i in range(0, values.len()) {
        v = values[i];
        d.set_min(v);
        assert_eq!(d.get_min(), v);
        assert_eq!(d.dom.borrow().intervals.len(), lengths[i])
    }
    intervals_bounds_are_coherent(&d);
}

#[test]
fn sets_min_in_hole() {
    let d = setup_domain_simple();
    d.set_min(43);
    assert_eq!(d.get_min(), 54);
    intervals_bounds_are_coherent(&d);
}

#[test]
// #[should_fail]
fn sets_min_too_high() {
    let d = setup_domain_simple();
    d.set_min(65);
    assert_eq!(d.get_min(), -3);
    intervals_bounds_are_coherent(&d);
}

#[test]
fn sets_max_higher() {
    let d = setup_domain_simple();
    d.set_max(65);
    assert_eq!(d.get_max(), 64);
    intervals_bounds_are_coherent(&d);
}

#[test]
fn sets_max_middle() {
    let d = setup_domain_simple();
    let values = [63, 54, 42, 8, -3];
    let lengths = [3, 3, 2, 2, 1];
    let mut v : int;
    for i in range(0, values.len()) {
        v = values[i];
        d.set_max(v);
        assert_eq!(d.get_max(), v);
        assert_eq!(d.dom.borrow().intervals.len(), lengths[i])
    }
    intervals_bounds_are_coherent(&d);
}

#[test]
fn sets_max_in_hole() {
    let d = setup_domain_simple();
    d.set_max(43);
    assert_eq!(d.get_max(), 42);
    intervals_bounds_are_coherent(&d);
}

#[test]
// #[should_fail]
fn sets_max_too_low() {
    let d = setup_domain_simple();
    d.set_max(-4);
    assert_eq!(d.get_max(), 64);
    intervals_bounds_are_coherent(&d);
}

fn setup_domain_holy() -> IntervalDomain {
    IntervalDomain {
        dom: RefCell::new(IntervalDom {
                 min: -3,
                 max: 64,
                 intervals: vec![(-3, 2), (4, 18), (20, 24), (30, 30),
                 (32, 34), (36, 38), (40, 42), (54, 64)]
             })
    }
}

#[test]
fn remove_outside() {
    let d = setup_domain_holy();
    let e = setup_domain_holy();
    d.remove(-8);
    d.remove(3);
    d.remove(19);
    d.remove(31);
    d.remove(35);
    d.remove(48);
    d.remove(128);
    assert_eq!(d.dom.borrow().intervals.len(), e.dom.borrow().intervals.len());
    for i in range(0, d.dom.borrow().intervals.len()) {
        assert_eq!(d.dom.borrow().intervals.get(i), e.dom.borrow().intervals.get(i));
    }
    intervals_bounds_are_coherent(&d);
}

#[test]
fn remove_inside() {
    let d = setup_domain_holy();
    let values = [-3, -1, 30, 36, 64];
    for &v in values.iter() {
        d.remove(v)
    }
    for &v in values.iter() {
        for &(x, y) in d.dom.borrow().intervals.iter() {
            assert!(v < x || v > y, format!("{} is not outside [{}..{}]", v, x, y));
        }
    }
    assert_eq!(d.dom.borrow().intervals.len(), 8);
    intervals_bounds_are_coherent(&d);
}

#[test]
#[should_fail]
#[allow(unused_variable)]
fn bitdomain_is_small() {
    let d : BitDomain = Domain::new(-5, 59);
}


#[test]
fn bitdomain_is_consistent() {
    let d : BitDomain = Domain::new(-4, 59);
    assert_eq!((-4, 59), (d.get_min(), d.get_max()));
}
