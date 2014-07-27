use super::{Event, Max, Min, Prop, Mod, FDVar, Propagator};

use std::rc::{Rc, Weak};

#[allow(dead_code)]
/// X < Y
pub struct LtXY;

#[allow(dead_code)]
impl LtXY {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) {
        LtXYC::new(model, x, y, 0);
    }
}

#[allow(dead_code)]
/// X < Y + C
pub struct LtXYC;

#[allow(dead_code)]
impl LtXYC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) {
        LtXYCx::new(model.clone(), x.clone(), y.clone(), c);
        LtXYCy::new(model.clone(), x.clone(), y.clone(), c);
    }
}

#[allow(dead_code)]
/// X =< Y
pub struct LeXY;

#[allow(dead_code)]
impl LeXY {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) {
        LtXYC::new(model, x, y, 1);
    }
}

#[allow(dead_code)]
/// X =< Y + C
pub struct LeXYC;

#[allow(dead_code)]
impl LeXYC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) {
        LtXYC::new(model, x, y, c + 1);
    }
}

#[allow(dead_code)]
/// X > Y
pub struct GtXY;

#[allow(dead_code)]
impl GtXY {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) {
        LtXYC::new(model, y, x, 0);
    }
}

#[allow(dead_code)]
/// X > Y + C
pub struct GtXYC;

#[allow(dead_code)]
impl GtXYC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) {
        LtXYC::new(model, y, x, -c);
    }
}

#[allow(dead_code)]
/// X >= Y
pub struct GeXY;

#[allow(dead_code)]
impl GeXY {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) {
        LtXYC::new(model, y, x, 1);
    }
}

#[allow(dead_code)]
/// X >= Y + C
pub struct GeXYC;

#[allow(dead_code)]
impl GeXYC {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>, c: int) {
        LtXYC::new(model, y, x, 1 - c);
    }
}

struct LtXYCx : Prop {
    c: int
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
