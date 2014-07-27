use super::{Event, Ins, Prop, Mod, FDVar, Propagator, LeXY, GeXY, LeXYC, GeXYC, LeXC, GeXC};

use std::rc::{Rc, Weak};

/// X = Y
pub struct EqXY;

impl EqXY {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) {
        // TODO merge
        LeXY::new(model.clone(), x.clone(), y.clone());
        GeXY::new(model, x, y);
    }
}

/// X = Y + C
pub struct EqXYC;

impl EqXYC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) {
        // TODO merge
        LeXYC::new(model.clone(), x.clone(), y.clone(), c);
        GeXYC::new(model, x, y, c);
    }
}

/// X = C
pub struct EqXC;

impl EqXC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, c: int) {
        // TODO merge
        LeXC::new(model.clone(), x.clone(), c);
        GeXC::new(model, x, c);
    }
}

/// X != Y
pub struct NeqXY;

impl NeqXY {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) {
        NeqXYCxy::new(model, x, y, 0);
    }
}

/// X != Y + C
pub struct NeqXYC;

impl NeqXYC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) {
        NeqXYCxy::new(model, x, y, c);
    }
}

/// X != C
pub struct NeqXC;

#[allow(unused_variable)]
impl NeqXC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, c: int) {
        x.remove(c);
    }
}

struct NeqXYCxy : Prop {
    c: int
}

impl NeqXYCxy {
    fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) {
        let id = model.propagators.borrow().len();
        let this = NeqXYCxy { model: model.downgrade(), id: id, vars: vec![x, y], c: c};
        let p = Rc::new((box this) as Box<Propagator>);
        model.add_prop(p);
    }

    fn x(&self) -> Rc<FDVar> {
        self.vars.get(0).clone()
    }

    fn y(&self) -> Rc<FDVar> {
        self.vars.get(1).clone()
    }
}

impl Propagator for NeqXYCxy {
    fn id(&self) -> uint {
        self.id
    }
    fn model(&self) -> Weak<Mod> {
        self.model.clone()
    }

    fn events(&self) -> Vec<(uint, Event)> {
        vec![(self.y().id, Ins), (self.x().id, Ins)]
    }

    fn propagate(&self) -> Vec<uint> {
        if self.x().is_instanciated() {
            self.unregister();
            self.y().remove(self.x().min() - self.c)
        }
        else if self.y().is_instanciated() {
            self.unregister();
            self.x().remove(self.y().min() + self.c)
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests;
