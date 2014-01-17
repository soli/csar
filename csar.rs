struct Domain {
   // list of intervals based
   min: int,
   max: int,
   intervals: ~[(int, int)]
}

trait Propagator : ToStr {
   fn propagate(&self);
}

struct FDVar {
   name: ~str,
   dom: Domain,
   waitingOnIns: ~[~Propagator]
}

impl Domain {
   fn new(min: int, max: int) -> Domain {
      Domain {
         min: min,
         max: max,
         intervals: ~[(min, max)]
      }
   }

   fn set_min(&mut self, min: int) {
      if min < self.min { return; }
      if min > self.max { return; } // TODO failure via conditions
      loop {
         match self.intervals[0] {
            (x, _) if min < x => { self.min = x; break; },
            (_, y) if min > y => { self.intervals.shift(); },
            (_, y) => { self.min = min; self.intervals[0] = (min, y); break }
         }
      }
   }
}

impl ToStr for Domain {
   fn to_str(&self) -> ~str {
      let mut s = "(" + self.min.to_str() + ", " + self.max.to_str() + ") [";
      for &(min, max) in self.intervals.iter() {
         s = s + min.to_str() + ".." + max.to_str() + ", ";
      }
      return s + "]";
   }
}

impl FDVar {
   fn new(min: int, max: int, name: ~str) -> FDVar {
      assert!(min <= max);
      FDVar {
         name: name,
         dom: Domain::new(min, max),
         waitingOnIns: ~[]
      }
   }

   fn min(&self) -> int {
      self.dom.min
   }

   fn max(&self) -> int {
      self.dom.max
   }
}

impl ToStr for FDVar {
   fn to_str(&self) -> ~str {
      self.name + " (" + self.dom.to_str() + ")"
   }
}

#[cfg(test)]
mod tests {
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
}

#[allow(dead_code)]
fn main() {
   println("Constraint Satisfaction in Rust");
}
