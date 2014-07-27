use super::super::{Model, Var, EqXY, NeqXY, EqXYC, NeqXYC, EqXC, NeqXC};
use super::{NeqXYCxy};

#[test]
fn neqxycxy_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 255, "x");
    let y = Var::new(m.clone(), 10, 10, "y");
    let p1 = NeqXYCxy::new(m.clone(), x.clone(), y.clone(), -11);
    assert_eq!((p1.id(), x.min(), x.max()), (0, -2, 255));
    let p2 = NeqXYCxy::new(m.clone(), x.clone(), y.clone(), -12);
    assert_eq!((p2.id(), x.min(), x.max()), (1, 0, 255));
    let p3 = NeqXYCxy::new(m.clone(), x.clone(), y.clone(), 245);
    assert_eq!((p3.id(), x.min(), x.max()), (2, 0, 254));
}

#[test]
fn eqxy_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), 8, 255, "x");
    let y = Var::new(m.clone(), -2, 128, "y");
    EqXY::new(m.clone(), x.clone(), y.clone());
    assert_eq!((x.min(), x.max(), y.min(), y.max()), (8, 128, 8, 128));
}

#[test]
fn eqxyc_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), 8, 255, "x");
    let y = Var::new(m.clone(), -2, 128, "y");
    EqXYC::new(m.clone(), x.clone(), y.clone(), 2);
    assert_eq!((x.min(), x.max(), y.min(), y.max()), (8, 130, 6, 128));
}

#[test]
fn eqxc_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), 8, 255, "x");
    EqXC::new(m.clone(), x.clone(), 42);
    assert_eq!((x.min(), x.max()), (42, 42));
    assert!(x.is_instanciated());
}

#[test]
#[ignore]   // FIXME waking up propagators
fn neqxy_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), 8, 255, "x");
    let y = Var::new(m.clone(), -2, 128, "y");
    NeqXY::new(m.clone(), x.clone(), y.clone());
    EqXC::new(m.clone(), x.clone(), 128);
    assert_eq!((x.min(), x.max(), y.min(), y.max()), (128, 128, -2, 127));
}

#[test]
fn neqxyc_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), 8, 255, "x");
    let y = Var::new(m.clone(), -2, -2, "y");
    NeqXYC::new(m.clone(), x.clone(), y.clone(), 257);
    assert_eq!((x.min(), x.max(), y.min(), y.max()), (8, 254, -2, -2));
}

#[test]
fn neqxc_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), 8, 255, "x");
    NeqXC::new(m.clone(), x.clone(), 9);
    NeqXC::new(m.clone(), x.clone(), 10);
    NeqXC::new(m.clone(), x.clone(), 8);
    assert_eq!((x.min(), x.max()), (11, 255));
}
