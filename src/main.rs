#![allow(dead_code)]
use std::{cell::Cell, fmt::Debug};

struct DiffBox<'a>{
    value: f64,
    children: Option<(Box<DiffBox<'a>>, Box<DiffBox<'a>>)>,
    op: Option<&'a str>,
}

impl <'a> DiffBox<'a> {
    fn new(value: f64) -> DiffBox<'a> {
        DiffBox { value, children: None, op: None }
    }

    fn add(self: Self, other: DiffBox) -> DiffBox {
        DiffBox::new(self.value + other.value)
    }

    fn mul(self: Self, other: DiffBox) -> DiffBox {
        DiffBox::new(self.value * other.value)
    }

    fn back(self: Self, parent_grad: f64) -> InterpretedDiffBox {

        let grad = match self.op {
            None => panic!("cannot get"),
            Some("*") => {
                parent_grad * 
            }
            // Some("*") =>
        }

        InterpretedDiffBox { grad: 3.4 }
    }
}

struct InterpretedDiffBox {
    grad: f64
}

fn main() {
    let a = DiffBox::new(3.0);
    let b = DiffBox::new(7.0);
    let c = DiffBox::new(4.0);

    let f1 = a.mul(b.add(c));

    println!("{:?}", f1.value);
}