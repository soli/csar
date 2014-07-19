#![desc = "Constraint Satisfaction in Rust"]
#![license = "MIT"]
#![crate_id = "csar#0.1"]
// Segfaults as lib (???), so we stay as bin for now...
//#[crate_type = "lib"];
#![feature(managed_boxes)]

use std::cell::RefCell;

#[allow(dead_code)]
fn main() {
   return;
}

/// Representation of finite domains as a list of intervals, maintaining
/// min and max for easy/quick access
struct Domain {
   min: int,
   max: int,
   intervals: Vec<int>
}

pub trait Propagator : ToStr {
   fn propagate(&mut self) -> Vec<Event>;
   fn register(&self);
   fn unregister(&self);
}

pub struct FDVar {
   name: String,
   dom: Domain,
   waitingOnMin: Vec<Propagator>,
   waitingOnMax: Vec<Propagator>,
   waitingOnIns: Vec<Propagator>
}

pub enum Event {
   Min,
   Max,
   Ins
}

impl Domain {
   /// Domain created with initial bounds
   fn new(min: int, max: int) -> Domain {
      Domain {
         min: min,
         max: max,
         intervals: vec![(min, max)]
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
               self.intervals[test] = (x, y - 1);
               break;
            },
            (x, y) => {
               self.intervals[test] = (x, val - 1);
               self.intervals.insert(test + 1, (val + 1, y));
               break;
            }
         }
      }
      if test == 0 {
         match self.intervals[test] {
            (x, _) => self.min = x
         }
      } else if test == self.intervals.len() - 1 {
         match self.intervals[test] {
            (_, y) => self.max = y
         }
      }
   }
}

impl ToStr for Domain {
   fn to_str(&self) -> String {
      let mut s = "(" + self.min.to_str() + ", " + self.max.to_str() + ") [";
      for &(min, max) in self.intervals.iter() {
         s = s + min.to_str() + ".." + max.to_str() + ", ";
      }
      return s + "]";
   }
}

impl FDVar {
   pub fn new(min: int, max: int, name: String) -> FDVar {
      assert!(min <= max);
      FDVar {
         name: name,
         dom: Domain::new(min, max),
         waitingOnMin: vec![],
         waitingOnMax: vec![],
         waitingOnIns: vec![]
      }
   }

   pub fn min(&self) -> int {
      self.dom.min
   }

   pub fn max(&self) -> int {
      self.dom.max
   }

   fn set_min(&mut self, v: int) -> Vec<Event> {
      if v > self.min() {
         self.dom.set_min(v);
         if self.is_instanciated() {
            [Min, Ins]
         } else {
            [Min]
         }
      } else {
         []
      }
   }

   fn set_max(&mut self, v: int) -> Vec<Event> {
      if v < self.max() {
         self.dom.set_max(v);
         if self.is_instanciated() {
            [Max, Ins]
         } else {
            [Max]
         }
      } else {
         []
      }
   }

   fn is_instanciated(&self) -> bool {
      self.min() == self.max()
   }

   fn add_waiting_min(&self, p: &Propagator) {}

   fn add_waiting_max(&self, p: &Propagator) {}

   fn add_waiting_ins(&self, p: &Propagator) {}

   fn del_waiting_min(&self, p: &Propagator) {}

   fn del_waiting_max(&self, p: &Propagator) {}

   fn del_waiting_ins(&self, p: &Propagator) {}
}

impl ToStr for FDVar {
   fn to_str(&self) -> String {
      self.name + " (" + self.dom.to_str() + ")"
   }
}

pub struct LtXYx {
   x: @RefCell<FDVar>,
   y: @RefCell<FDVar>
}

impl LtXYx {
   pub fn new(x: @RefCell<FDVar>, y: @RefCell<FDVar>) -> LtXYx {
      let mut this = LtXYx { x: x, y: y };
      this.register();
      this.propagate();
      this
   }
}

impl Propagator for LtXYx {
   fn register(&self) {
      let mut y = self.y.borrow_mut();
      y.get().add_waiting_max(self);
   }

   fn unregister(&self) {
      let mut y = self.y.borrow_mut();
      y.get().del_waiting_max(self);
   }

   fn propagate(&mut self) -> Vec<Event> {
      let mut xx = self.x.borrow_mut();
      let x = xx.get();
      let mut yy = self.y.borrow_mut();
      let y = yy.get();
      if x.max() < y.min() {
         // entailed
         self.unregister();
         []
      } else if x.max() > y.max() - 1 {
         //if y.is_instanciated() {
         //   self.unregister();
         //}
         x.set_max(y.max() - 1)
      } else {
         []
      }
   }
}
impl ToStr for LtXYx {
   fn to_str(&self) -> String {
      "x < y"
     // format!("{} < {}", self.x.to_str(), self.y.to_str())
   }
}

#[cfg(test)]
mod tests;
