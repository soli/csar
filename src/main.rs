#![desc = "Constraint Satisfaction in Rust"]
#![license = "MIT"]
#![crate_id = "csar#0.2"]
#![crate_type = "lib"]

use std::fmt;
use std::cell::RefCell;
use std::io;

/// Representation of finite domains as a list of intervals, maintaining
/// min and max for easy/quick access
struct Dom {
    min: int,
    max: int,
    intervals: Vec<(int, int)>
}

/// Runtime checked mutability with borrowing
struct Domain {
    dom: RefCell<Dom>
}

pub trait Propagator : ToStr {
    fn propagate(&mut self) -> Vec<Event>;
    fn register(&self);
    fn unregister(&self);
}

#[allow(dead_code)]
pub struct FDVar {
    name: String,
    dom: Domain,
    waitingOnMin: Vec<Box<Propagator>>,
    waitingOnMax: Vec<Box<Propagator>>,
    waitingOnIns: Vec<Box<Propagator>>
}

#[deriving(Show)]
pub enum Event {
    Min,
    Max,
    Ins
}

#[allow(dead_code)]
impl Domain {
    /// Domain created with initial bounds
    fn new(min: int, max: int) -> Domain {
        Domain {
            dom: RefCell::new(Dom {
                     min: min,
                     max: max,
                     intervals: vec![(min, max)]
                 })
        }
    }

    fn set_min(&self, min: int) {
        let mut dom = self.dom.borrow_mut();
        if min < dom.min { return; }
        if min > dom.max { return; } // TODO failure via conditions
        loop {
            match dom.intervals.get(0) {
                // note that the breaks are for the loop, not the matching
                &(x, _) if min < x => { dom.min = x; break; },
                &(_, y) if min > y => { dom.intervals.shift(); },
                &(_, y) => {
                    dom.min = min;
                    *dom.intervals.get_mut(0) = (min, y);
                    break;
                }
            }
        }
    }

    fn get_min(&self) -> int {
        self.dom.borrow().min
    }

    fn set_max(&self, max: int) {
        let mut dom = self.dom.borrow_mut();
        if max > dom.max { return; }
        if max < dom.min { return; } // TODO failure via conditions
        loop {
            match dom.intervals.last().unwrap() {
                &(_, y) if max > y => { dom.max = y; break; },
                &(x, _) if max < x => { dom.intervals.pop(); },
                &(x, _) => {
                    dom.max = max;
                    *dom.intervals.mut_last().unwrap() = (x, max);
                    break
                }
            }
        }
    }

    fn get_max(&self) -> int {
        self.dom.borrow().max
    }

    fn remove(&self, val: int) {
        let mut dom = self.dom.borrow_mut();
        if val > dom.max || val < dom.min { return; }
        let mut down = 0;
        let mut up = dom.intervals.len();
        let mut test;
        loop {
            test = down + (up - down) / 2;
            match dom.intervals.get(test) {
                &(x, _) if val < x => {
                    if test > down {
                        up = test;
                    } else {
                        break;
                    }
                },
                &(_, y) if val > y => {
                    if test < up - 1 {
                        down = test + 1;
                    } else {
                        break;
                    }
                },
                &(x, y) if val == x && val == y => {
                    dom.intervals.remove(test);
                    break;
                },
                &(x, y) if val == x => {
                    *dom.intervals.get_mut(test) = (x + 1, y);
                    break;
                },
                &(x, y) if val == y => {
                    *dom.intervals.get_mut(test) = (x, y - 1);
                    break;
                },
                &(x, y) => {
                    *dom.intervals.get_mut(test) = (x, val - 1);
                    dom.intervals.insert(test + 1, (val + 1, y));
                    break;
                }
            }
        }
        if test == 0 {
            match dom.intervals.get(test) {
                &(x, _) => dom.min = x
            }
        } else if test == dom.intervals.len() - 1 {
            match dom.intervals.get(test) {
                &(_, y) => dom.max = y
            }
        }
    }
}

impl fmt::Show for Domain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dom = self.dom.borrow();
        let mut s = format!("({}, {}) [", dom.min, dom.max);
        for &(min, max) in dom.intervals.iter() {
            s = s + min.to_str() + ".." + max.to_str() + ", ";
        }
        return write!(f, "{}]", s);
    }
}

#[allow(dead_code)]
#[allow(unused_variable)]
impl FDVar {
    pub fn new(min: int, max: int, name: &str) -> FDVar {
        assert!(min <= max);
        FDVar {
            name: name.to_string(),
            dom: Domain::new(min, max),
            waitingOnMin: vec![],
            waitingOnMax: vec![],
            waitingOnIns: vec![]
        }
    }

    pub fn min(&self) -> int {
        self.dom.get_min()
    }

    pub fn max(&self) -> int {
        self.dom.get_max()
    }

    fn set_min(&mut self, v: int) -> Vec<Event> {
        if v > self.min() {
            self.dom.set_min(v);
            if self.is_instanciated() {
                vec![Min, Ins]
            } else {
                vec![Min]
            }
        } else {
            vec![]
        }
    }

    fn set_max(&self, v: int) -> Vec<Event> {
        if v < self.max() {
            self.dom.set_max(v);
            if self.is_instanciated() {
                vec![Max, Ins]
            } else {
                vec![Max]
            }
        } else {
            vec![]
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

impl fmt::Show for FDVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.dom)
    }
}

pub struct LtXYx<'r> {
    x: &'r FDVar,
    y: &'r FDVar
}

impl<'r> LtXYx<'r> {
    pub fn new(x: &'r FDVar, y: &'r FDVar) -> LtXYx<'r> {
        let mut this = LtXYx { x: x, y: y };
        this.register();
        io::stderr().write_line(this.propagate().to_str().as_slice()).unwrap();
        this
    }
}

impl<'r> Propagator for LtXYx<'r> {
    fn register(&self) {
        self.y.add_waiting_max(self);
    }

    fn unregister(&self) {
        self.y.del_waiting_max(self);
    }

    fn propagate(&mut self) -> Vec<Event> {
        if self.x.max() < self.y.min() {
            // entailed
            self.unregister();
            vec![]
        } else if self.x.max() > self.y.max() - 1 {
            //if y.is_instanciated() {
            //   self.unregister();
            //}
            self.x.set_max(self.y.max() - 1)
        } else {
            vec![]
        }
    }
}
impl<'r> fmt::Show for LtXYx<'r> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} < {}", self.x.to_str(), self.y.to_str())
    }
}

#[cfg(test)]
mod tests;
