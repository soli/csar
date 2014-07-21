use super::{Event, Max, Min, Prop, Mod, FDVar, Propagator};

#[cfg(test)]
use super::{Model, Var};

use std::rc::{Rc, Weak};

#[allow(dead_code)]
pub struct LtXY;

#[allow(dead_code)]
impl LtXY {
    pub fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) {
        LtXYx::new(model.clone(), x.clone(), y.clone());
        LtXYy::new(model.clone(), x.clone(), y.clone());
    }
}

struct LtXYx : Prop;

#[allow(dead_code)]
impl LtXYx {
    fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) -> Rc<Box<Propagator>> {
        let id = model.propagators.borrow().len();
        let this = LtXYx { model: model.downgrade(), id: id, vars: vec![x, y]};
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

impl Propagator for LtXYx {
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
        if self.x().max() < self.y().min() {
            // entailed
            self.unregister();
            vec![]
        } else if self.x().max() > self.y().max() - 1 {
            //if y.is_instanciated() {
            //   self.unregister();
            //}
            let max = self.y().max() - 1;
            self.x().set_max(max)
        } else {
            vec![]
        }
    }
}

struct LtXYy : Prop;

#[allow(dead_code)]
impl LtXYy {
    fn new(model: Rc<Mod>, x: Rc<FDVar>, y: Rc<FDVar>) -> Rc<Box<Propagator>> {
        let id = model.propagators.borrow().len();
        let this = LtXYy { model: model.downgrade(), id: id, vars: vec![x, y]};
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

impl Propagator for LtXYy {
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
        if self.x().max() < self.y().min() {
            // entailed
            self.unregister();
            vec![]
        } else if self.y().min() < self.x().min() + 1 {
            //if y.is_instanciated() {
            //   self.unregister();
            //}
            let min = self.x().min() + 1;
            self.y().set_min(min)
        } else {
            vec![]
        }
    }
}

#[test]
fn it_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 255, "x");
    let y = Var::new(m.clone(), -2, 255, "y");
    let p1 = LtXYx::new(m.clone(), x.clone(), y.clone());
    assert!(p1.id() == 0);
    assert!(x.max() == 254);
    let p2 = LtXYy::new(m.clone(), x.clone(), y.clone());
    assert!(p2.id() == 1);
    assert!(y.min() == -1);
}
