use super::super::{Model, Var, LtXY, GtXY, LeXY, GeXY, LtXYC, GtXYC, LeXYC, GeXYC};
use super::{LtXYCx, LtXYCy};

#[test]
fn propagator_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 255, "x");
    let y = Var::new(m.clone(), -2, 255, "y");
    let p1 = LtXYCx::new(m.clone(), x.clone(), y.clone(), -2);
    assert_eq!((p1.id(), x.max()), (0, 252));
    let p2 = LtXYCy::new(m.clone(), x.clone(), y.clone(), -2);
    assert_eq!((p2.id(), y.min()), (1, 1));
}

#[test]
fn ltxy_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 255, "x");
    let y = Var::new(m.clone(), -2, 255, "y");
    LtXY::new(m.clone(), x.clone(), y.clone());
    assert_eq!((x.max(), y.min()), (254, -1));
}

#[test]
fn gtxy_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 252, "x");
    let y = Var::new(m.clone(), 2, 255, "y");
    GtXY::new(m.clone(), x.clone(), y.clone());
    assert_eq!((x.min(), y.max()), (3, 251));
}

#[test]
fn lexy_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 255, "x");
    let y = Var::new(m.clone(), -2, 255, "y");
    LeXY::new(m.clone(), x.clone(), y.clone());
    assert_eq!((x.max(), y.min()), (255, -2));
}

#[test]
fn gexy_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 252, "x");
    let y = Var::new(m.clone(), 2, 255, "y");
    GeXY::new(m.clone(), x.clone(), y.clone());
    assert_eq!((x.min(), y.max()), (2, 252));
}

#[test]
fn ltxyc_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 255, "x");
    let y = Var::new(m.clone(), -2, 255, "y");
    LtXYC::new(m.clone(), x.clone(), y.clone(), -1);
    assert_eq!((x.max(), y.min()), (253, 0));
}

#[test]
fn gtxyc_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 252, "x");
    let y = Var::new(m.clone(), 2, 255, "y");
    GtXYC::new(m.clone(), x.clone(), y.clone(), 1);
    assert_eq!((x.min(), y.max()), (4, 250));
}

#[test]
fn lexyc_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 255, "x");
    let y = Var::new(m.clone(), -2, 255, "y");
    LeXYC::new(m.clone(), x.clone(), y.clone(), 1);
    assert_eq!((x.max(), y.min()), (255, -2));
}

#[test]
fn gexyc_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 252, "x");
    let y = Var::new(m.clone(), 2, 255, "y");
    GeXYC::new(m.clone(), x.clone(), y.clone(), 3);
    assert_eq!((x.min(), y.max()), (5, 249));
}
