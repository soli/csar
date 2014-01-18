#[desc = "Constraint Satisfaction in Rust"];
#[license = "MIT"];
#[crate_id = "csar#0.1"];
// Segfaults, so we stay as bin for now...
//#[crate_type = "lib"];
#[allow(dead_code)]
fn main() {
   return;
}

struct Domain {
   // list of intervals based
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
