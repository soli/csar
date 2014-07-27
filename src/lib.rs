#![desc = "Constraint Satisfaction in Rust"]
#![license = "MIT"]
#![crate_id = "csar#0.2"]
#![crate_type = "lib"]

#![feature(struct_inherit)]
#![feature(globs)]

use std::fmt;
use std::cell::RefCell;
use std::collections::hashmap::HashMap;
use std::rc::{Rc, Weak};

pub use ltxy::{LtXY, LtXYC, LeXY, LeXYC, GtXY, GtXYC, GeXY, GeXYC, LtXC, GtXC, LeXC, GeXC};
pub use eqxy::{EqXY, EqXYC, EqXC, NeqXY, NeqXYC, NeqXC};

#[allow(dead_code)]
pub struct Mod {
    vars: RefCell<Vec<Rc<FDVar>>>,
    propagators: RefCell<Vec<Rc<Box<Propagator>>>>,
    waiting: RefCell<HashMap<(uint, Event), Vec<uint>>>
}

/// wrapping Mod in an Rc
/// cannot use type Model = Rc<Mod> since that would forbid using impl Model
pub struct Model;

/// Representation of finite domains as a list of intervals, maintaining
/// min and max for easy/quick access
#[deriving(Clone)]
struct Dom {
    min: int,
    max: int,
    intervals: Vec<(int, int)>
}

/// Runtime checked mutability with borrowing
#[deriving(Clone)]
struct Domain {
    dom: RefCell<Dom>
}

trait Propagator {
    fn id(&self) -> uint;
    fn model(&self) -> Weak<Mod>;

    fn events(&self) -> Vec<(uint, Event)>;
    fn propagate(&self) -> Vec<uint>;

    fn register(&self) {
        for &(var, event) in self.events().iter() {
            self.model().upgrade().unwrap().add_waiting(var, event, self.id());
        }
    }

    fn unregister(&self) {
        for &(var, event) in self.events().iter() {
            self.model().upgrade().unwrap().del_waiting(var, event, self.id());
        }
    }
}

pub virtual struct Prop {
    id: uint,
    model: Weak<Mod>,
    vars: Vec<Rc<FDVar>>
}

#[deriving(Clone)]
pub struct FDVar {
    model: Weak<Mod>,
    id: uint,
    name: String,
    dom: Domain
}

/// wrapping FDVar in an Rc
pub struct Var;

#[deriving(Show, Hash, Eq, PartialEq)]
pub enum Event {
    Min,
    Max,
    Ins
}

#[allow(dead_code)]
impl Model {
    fn new() -> Rc<Mod> {
        Rc::new(Mod {
            vars: RefCell::new(Vec::new()),
            propagators: RefCell::new(Vec::new()),
            waiting: RefCell::new(HashMap::new())
        })
    }
}

#[allow(dead_code)]
impl Mod {
    fn add_var(&self, var: Rc<FDVar>) {
        self.vars.borrow_mut().push(var);
    }

    fn add_prop(&self, prop: Rc<Box<Propagator>>) {
        self.propagators.borrow_mut().push(prop.clone());
        prop.register();
        self.propagate(self.propagators.borrow().len() - 1);
    }

    fn add_waiting(&self, var: uint, event: Event, propagator: uint) {
        let mut waiting = self.waiting.borrow_mut();
        if waiting.contains_key(&(var, event)) {
            waiting.get_mut(&(var, event)).push(propagator);
        } else {
            waiting.insert((var, event), vec![propagator]);
        }
    }

    fn del_waiting(&self, var: uint, event: Event, propagator: uint) {
        self.waiting.borrow_mut().get_mut(&(var, event)).remove(propagator);
    }

    fn get_waiting(&self, var: uint, event: Event) -> Vec<uint> {
        let waiting = self.waiting.borrow();
        match waiting.find_copy(&(var, event)) {
            Some(vec) => vec,
            None => Vec::new()
        }
    }

    fn propagate(&self, id: uint) {
        println!("propagating for {}", id.to_str());
        let woken = self.propagators.borrow().get(id).propagate();
        println!("waking {}", woken.to_str());
        for &propid in woken.iter() {
            self.propagate(propid);
        }
    }

    fn propagate_vec(&self, ids: Vec<uint>) {
        println!("propagating for {}", ids.to_str());
        for &propid in ids.iter() {
            self.propagate(propid);
        }
    }
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

impl Var {
    pub fn new(model: Rc<Mod>, min: int, max: int, name: &str) -> Rc<FDVar> {
        assert!(min <= max);
        let id = model.vars.borrow().len();
        let v = Rc::new(FDVar {
            model: model.downgrade(),
            id: id,
            name: name.to_string(),
            dom: Domain::new(min, max)
        });
        model.add_var(v.clone());
        v
    }
}

impl FDVar {
    pub fn min(&self) -> int {
        self.dom.get_min()
    }

    pub fn max(&self) -> int {
        self.dom.get_max()
    }

    fn set_min(&self, v: int) -> Vec<uint> {
        if v > self.min() {
            self.dom.set_min(v);
            let model = self.model.upgrade().unwrap();
            if self.is_instanciated() {
                model.get_waiting(self.id, Min).append(model.get_waiting(self.id, Ins).as_slice())
            } else {
                model.get_waiting(self.id, Min)
            }
        } else {
            vec![]
        }
    }

    fn set_max(&self, v: int) -> Vec<uint> {
        if v < self.max() {
            self.dom.set_max(v);
            let model = self.model.upgrade().unwrap();
            if self.is_instanciated() {
                model.get_waiting(self.id, Max).append(model.get_waiting(self.id, Ins).as_slice())
            } else {
                model.get_waiting(self.id, Max)
            }
        } else {
            vec![]
        }
    }

    fn remove(&self, v: int) -> Vec<uint> {
        let min = self.min();
        let max = self.max();
        match v {
            vv if vv < min || vv > max => vec![],
            vv if vv == min => self.set_min(vv + 1),
            vv if vv == max => self.set_max(vv - 1),
            _ => {
                self.dom.remove(v);
                vec![]
            }
        }
    }

    fn is_instanciated(&self) -> bool {
        self.min() == self.max()
    }
}

impl fmt::Show for FDVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.dom)
    }
}

mod ltxy;
mod eqxy;

#[cfg(test)]
mod tests;
