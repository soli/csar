use super::FDVar;
use super::Domain;

#[test]
fn creates_new_var() {
   let x = FDVar::new(-2, 255, ~"x");
   assert!(x.min() == -2);
   assert!(x.max() == 255);
}

fn min_is_min(d: &Domain) -> bool {
   match d.intervals[0] {
      (x, _) => x == d.min
   }
}

fn setup_domain() -> Domain {
   Domain { min: -3, max: 64, intervals: ~[(-3, 2), (4, 42), (54, 64)] }
}

fn teardown(d: &Domain) {
   assert!(min_is_min(d));
}

#[test]
fn sets_min_lower() {
   let mut d = setup_domain();
   d.set_min(-4);
   assert!(d.min == -3);
   teardown(&d);
}

#[test]
fn sets_min_middle_first() {
   let mut d = setup_domain();
   let values = ~[-2, 8, 42, 54, 64];
   for &i in values.iter() {
      d.set_min(i);
      assert!(d.min == i);
   }
   teardown(&d);
}

#[test]
fn sets_min_in_hole() {
   let mut d = setup_domain();
   d.set_min(43);
   assert!(d.min == 54);
   teardown(&d);
}

#[test]
// #[should_fail]
fn sets_min_too_high() {
   let mut d = setup_domain();
   d.set_min(65);
   assert!(d.min == -3);
   teardown(&d);
}
