use super::{Event, Max, Min, Prop, Mod, FDVar, Propagator};

use std::rc::{Rc, Weak};

/// X < Y
pub struct LtXY;

impl LtXY {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) {
        LtXYC::new(model, x, y, 0);
    }
}

/// X < Y + C
pub struct LtXYC;

impl LtXYC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) {
        LtXYCx::new(model.clone(), x.clone(), y.clone(), c);
        LtXYCy::new(model, x.clone(), y.clone(), c);
    }
}

/// X < C
pub struct LtXC;

#[allow(unused_variable)]
impl LtXC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, c: int) {
        // FIXME do not throw away events...
        x.set_max(c - 1);
    }
}

/// X =< Y
pub struct LeXY;

impl LeXY {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) {
        LtXYC::new(model, x, y, 1);
    }
}

/// X =< Y + C
pub struct LeXYC;

impl LeXYC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) {
        LtXYC::new(model, x, y, c + 1);
    }
}

/// X =< C
pub struct LeXC;

#[allow(unused_variable)]
impl LeXC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, c: int) {
        // FIXME do not throw away events...
        x.set_max(c);
    }
}

/// X > Y
pub struct GtXY;

impl GtXY {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) {
        LtXYC::new(model, y, x, 0);
    }
}

/// X > Y + C
pub struct GtXYC;

impl GtXYC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) {
        LtXYC::new(model, y, x, -c);
    }
}

/// X > C
pub struct GtXC;

#[allow(unused_variable)]
impl GtXC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, c: int) {
        // FIXME do not throw away events...
        x.set_min(c + 1);
    }
}

/// X >= Y
pub struct GeXY;

impl GeXY {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) {
        LtXYC::new(model, y, x, 1);
    }
}

/// X >= Y + C
pub struct GeXYC;

impl GeXYC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) {
        LtXYC::new(model, y, x, 1 - c);
    }
}

/// X >= C
pub struct GeXC;

#[allow(unused_variable)]
impl GeXC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, c: int) {
        // FIXME do not throw away events...
        x.set_min(c);
    }
}

struct LtXYCx : Prop {
    c: int
}

impl LtXYCx {
    fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) -> Rc<Box<Propagator>> {
        let id = model.propagators.borrow().len();
        let this = LtXYCx { model: model.downgrade(), id: id, vars: vec![x, y], c: c};
        this.register();
        this.propagate();
        let p = Rc::new((box this) as Box<Propagator>);
        model.add_prop(p.clone());
        p
    }

    fn x(&self) -> Rc<FDVar> {
        self.vars.get(0).clone()
    }

    fn y(&self) -> Rc<FDVar> {
        self.vars.get(1).clone()
    }
}

impl Propagator for LtXYCx {
    fn id(&self) -> uint {
        self.id
    }
    fn model(&self) -> Weak<Mod> {
        self.model.clone()
    }

    fn events(&self) -> Vec<(uint, Event)> {
        vec![(self.y().id, Max)]
    }

    fn propagate(&self) -> Vec<uint> {
        if self.x().max() < self.y().min() + self.c {
            // entailed
            self.unregister();
            vec![]
        } else if self.x().max() > self.y().max() + self.c - 1 {
            //if y.is_instanciated() {
            //   self.unregister();
            //}
            let max = self.y().max() + self.c - 1;
            self.x().set_max(max)
        } else {
            vec![]
        }
    }
}

struct LtXYCy : Prop {
    c: int
}

impl LtXYCy {
    fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) -> Rc<Box<Propagator>> {
        let id = model.propagators.borrow().len();
        let this = LtXYCy { model: model.downgrade(), id: id, vars: vec![x, y], c: c};
        this.register();
        this.propagate();
        let p = Rc::new((box this) as Box<Propagator>);
        model.add_prop(p.clone());
        p
    }

    fn x(&self) -> Rc<FDVar> {
        self.vars.get(0).clone()
    }

    fn y(&self) -> Rc<FDVar> {
        self.vars.get(1).clone()
    }
}

impl Propagator for LtXYCy {
    fn id(&self) -> uint {
        self.id
    }
    fn model(&self) -> Weak<Mod> {
        self.model.clone()
    }

    fn events(&self) -> Vec<(uint, Event)> {
        vec![(self.x().id, Min)]
    }

    fn propagate(&self) -> Vec<uint> {
        if self.x().max() < self.y().min() + self.c {
            // entailed
            self.unregister();
            vec![]
        } else if self.y().min() < self.x().min() - self.c + 1 {
            //if y.is_instanciated() {
            //   self.unregister();
            //}
            let min = self.x().min() - self.c + 1;
            self.y().set_min(min)
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests;
