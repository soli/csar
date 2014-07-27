use super::super::{Model, Var};
use super::{LtXYCx, LtXYCy};

#[test]
fn it_does_propagate() {
    let m = Model::new();
    let x = Var::new(m.clone(), -2, 255, "x");
    let y = Var::new(m.clone(), -2, 255, "y");
    let p1 = LtXYCx::new(m.clone(), x.clone(), y.clone(), -2);
    assert!(p1.id() == 0);
    assert!(x.max() == 252);
    let p2 = LtXYCy::new(m.clone(), x.clone(), y.clone(), -2);
    assert!(p2.id() == 1);
    assert!(y.min() == 1);
}
