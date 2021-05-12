
pub struct A {
}

pub struct B<'a> {
    a: &'a mut A,
}

impl A {
    pub fn scope<'b, 'a: 'b>(&'a mut self) -> B<'b> {
        B{a: self}
    }
    pub fn new() -> Self {
        Self{}
    }
    pub fn action(x: i32) {
        println!("{}", x);
    }
}

impl<'b, 'a: 'b> B<'a> {
    pub fn scope(&'a mut self) -> B<'b> {
        B{a: self.a}
    }
}

fn main() {
    let mut a = A::new();
    for i in 0..4 {
        let mut b = a.scope();
        for j in 0..4 {
            let mut c = b.scope();
        }
    }
}
