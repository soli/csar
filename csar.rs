enum Domain {
   Interval(int, int),
   TreeDomain(~Domain, ~Domain)
}

trait Propagator : ToStr {
   fn propagate(&self);
}

struct FDVar {
   name: ~str,
   dom: Domain,
   waitingOnIns: ~[~Propagator]
}

impl Domain {
   fn new(min: int, max: int) -> Domain {
      Interval(min, max)
   }

   fn min(&self) -> int {
      match self {
         &Interval(x, _) => x,
         &TreeDomain(~ref left, _) => left.min()
      }
   }

   fn max(&self) -> int {
      match self {
         &Interval(_, y) => y,
         &TreeDomain(_, ~ref right) => right.max()
      }
   }
}

impl ToStr for Domain {
   fn to_str(&self) -> ~str {
      match self {
         &Interval(x, y) => format!("[{}..{}]", x, y),
         &TreeDomain(~ref left, ~ref right) =>
            format!("[{}..{}]", left.to_str(), right.to_str())
      }
   }
}

impl FDVar {
   fn new(min: int, max: int, name: ~str) -> FDVar {
      FDVar {
         name: name,
         dom: Domain::new(min, max),
         waitingOnIns: ~[]
      }
   }
}

impl ToStr for FDVar {
   fn to_str(&self) -> ~str {
      self.name + " (" + self.dom.to_str() + ")"
   }
}

fn main() {
   println("Constraint Satisfaction in Rust");
   let x = FDVar::new(0, 255, ~"x");
   println(x.to_str());
}
