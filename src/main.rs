#[desc = "Constraint Satisfaction in Rust"];
#[license = "MIT"];
#[crate_id = "csar#0.1"];
// Segfaults as lib (???), so we stay as bin for now...
//#[crate_type = "lib"];
#[allow(dead_code)]
fn main() {
   return;
}

/// Representation of finite domains as a list of intervals, maintaining
/// min and max for easy/quick access
struct Domain {
   min: int,
   max: int,
   intervals: ~[(int, int)]
}

trait Propagator : ToStr {
   fn propagate(&self);
}

pub struct FDVar {
   name: ~str,
   dom: Domain,
   waitingOnIns: ~[~Propagator]
}

impl Domain {
   /// Domain created with initial bounds
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

   fn set_max(&mut self, max: int) {
      if max > self.max { return; }
      if max < self.min { return; } // TODO failure via conditions
      loop {
         match self.intervals[self.intervals.len() - 1] {
            (_, y) if max > y => { self.max = y; break; },
            (x, _) if max < x => { self.intervals.pop(); },
            (x, _) => {
               self.max = max;
               self.intervals[self.intervals.len() - 1] = (x, max);
               break
            }
         }
      }
   }

   fn remove(&mut self, val: int) {
      if val > self.max || val < self.min { return; }
      let mut down = 0;
      let mut up = self.intervals.len();
      let mut test;
      loop {
         test = down + (up - down) / 2;
         match self.intervals[test] {
            (x, _) if val < x => {
               if test > down {
                  up = test;
               } else {
                  break;
               }
            },
            (_, y) if val > y => {
               if test < up - 1 {
                  down = test + 1;
               } else {
                  break;
               }
            },
            (x, y) if val == x && val == y => {
               self.intervals.remove(test);
               break;
            },
            (x, y) if val == x => {
               self.intervals[test] = (x + 1, y);
               break;
            },
            (x, y) if val == y => {
               self.intervals[test] = (x, y + 1);
               break;
            },
            (x, y) => {
               self.intervals[test] = (x, val - 1);
               self.intervals.insert(test + 1, (val + 1, y));
               break;
            }
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
   pub fn new(min: int, max: int, name: ~str) -> FDVar {
      assert!(min <= max);
      FDVar {
         name: name,
         dom: Domain::new(min, max),
         waitingOnIns: ~[]
      }
   }

   pub fn min(&self) -> int {
      self.dom.min
   }

   pub fn max(&self) -> int {
      self.dom.max
   }
}

impl ToStr for FDVar {
   fn to_str(&self) -> ~str {
      self.name + " (" + self.dom.to_str() + ")"
   }
}

#[cfg(test)]
mod tests; 
