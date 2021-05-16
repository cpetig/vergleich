
pub trait DB {
    fn action(&mut self, x: i32);
}

pub struct Base {
}

pub struct Wrapper<'a, X: DB> (&'a mut X);

impl Base {
    pub fn scope<'a>(&'a mut self) -> Wrapper<'a,Base> {
        Wrapper(self)
    }
    pub fn new() -> Self {
        Self{}
    }
}

impl DB for Base {
    fn action(&mut self, x: i32) {
        println!("{}", x);
    }
}

impl<'b, 'a: 'b, X:DB> Wrapper<'a, X> {
    pub fn scope(&'b mut self) -> Wrapper<'b, Wrapper<'a, X>> {
        Wrapper(self)
    }
}

impl<'a, X:DB> DB for Wrapper<'a, X> {
    fn action(&mut self, x: i32) {
        print!(".");
        self.0.action(x);
    }
}

fn sub<'b, 'a : 'b, X: DB>(b: &'a mut Wrapper<'b, X>) {
    let j=1;
    let mut c = b.scope();
    c.action(5+j);
}

fn main() {
    let mut a = Base::new();
    a.action(0);
    for i in 1..5 {
        let mut b = a.scope();
        b.action(i);
        let x=&mut b;
        sub(x);
        sub(x);
        // for j in 0..4 
        // {
        //     let mut c = b.scope();
        //     c.action(5+j);
        // }
    }
}
