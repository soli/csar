use super::FDVar;
use super::Domain;
use super::LtXYx;
use std::cell::RefCell;

#[test]
fn creates_new_var() {
   let x = FDVar::new(-2, 255, "x");
   assert!(x.min() == -2);
   assert!(x.max() == 255);
}

fn min_is_min(d: &Domain) -> bool {
   match d.intervals[0] {
      (x, _) => x == d.min
   }
}

fn max_is_max(d: &Domain) -> bool {
   match d.intervals[d.intervals.len() - 1] {
      (_, y) => y == d.max
   }
}

fn setup_domain_simple() -> Domain {
   Domain { min: -3, max: 64, intervals: [(-3, 2), (4, 42), (54, 64)] }
}

fn teardown(d: &Domain) {
   assert!(min_is_min(d));
   assert!(max_is_max(d));
}

#[test]
fn sets_min_lower() {
   let mut d = setup_domain_simple();
   d.set_min(-4);
   assert!(d.min == -3);
   teardown(&d);
}

#[test]
fn sets_min_middle() {
   let mut d = setup_domain_simple();
   let values = [-2, 8, 42, 54, 64];
   let lengths = [3, 2, 2, 1, 1];
   let mut v : int;
   for i in range(0, values.len()) {
      v = values[i];
      d.set_min(v);
      assert!(d.min == v);
      assert!(d.intervals.len() == lengths[i])
   }
   teardown(&d);
}

#[test]
fn sets_min_in_hole() {
   let mut d = setup_domain_simple();
   d.set_min(43);
   assert!(d.min == 54);
   teardown(&d);
}

#[test]
// #[should_fail]
fn sets_min_too_high() {
   let mut d = setup_domain_simple();
   d.set_min(65);
   assert!(d.min == -3);
   teardown(&d);
}

#[test]
fn sets_max_higher() {
   let mut d = setup_domain_simple();
   d.set_max(65);
   assert!(d.max == 64);
   teardown(&d);
}

#[test]
fn sets_max_middle() {
   let mut d = setup_domain_simple();
   let values = [63, 54, 42, 8, -3];
   let lengths = [3, 3, 2, 2, 1];
   let mut v : int;
   for i in range(0, values.len()) {
      v = values[i];
      d.set_max(v);
      assert!(d.max == v);
      assert!(d.intervals.len() == lengths[i])
   }
   teardown(&d);
}

#[test]
fn sets_max_in_hole() {
   let mut d = setup_domain_simple();
   d.set_max(43);
   assert!(d.max == 42);
   teardown(&d);
}

#[test]
// #[should_fail]
fn sets_max_too_low() {
   let mut d = setup_domain_simple();
   d.set_max(-4);
   assert!(d.max == 64);
   teardown(&d);
}

fn setup_domain_holy() -> Domain {
   Domain {
      min: -3,
      max: 64,
      intervals: [(-3, 2), (4, 18), (20, 24), (30, 30),
         (32, 34), (36, 38), (40, 42), (54, 64)] }
}

#[test]
fn remove_outside() {
   let mut d = setup_domain_holy();
   let e = setup_domain_holy();
   d.remove(-8);
   d.remove(3);
   d.remove(19);
   d.remove(31);
   d.remove(35);
   d.remove(48);
   d.remove(128);
   assert!(d.intervals.len() == e.intervals.len());
   for i in range(0, d.intervals.len()) {
      assert!(d.intervals[i] == e.intervals[i]);
   }
   teardown(&d);
}

#[test]
fn remove_inside() {
   let mut d = setup_domain_holy();
   let values = [-3, -1, 30, 36, 64];
   for &v in values.iter() {
      d.remove(v)
   }
   for &v in values.iter() {
      for &(x, y) in d.intervals.iter() {
         assert!(v < x || v > y, format!("{} is not outside [{}..{}]", v, x, y));
      }
   }
   assert!(d.intervals.len() == 8);
   teardown(&d);
}

#[test]
fn it_does_propagate() {
   let x = FDVar::new(-2, 255, "x");
   let y = FDVar::new(-2, 255, "y");
   let xx = @RefCell::new(x);
   let yy = @RefCell::new(y);
   let p = LtXYx::new(xx, yy);
   let mut foo = xx.borrow_mut();
   assert!(foo.get().max() == 254);
}
