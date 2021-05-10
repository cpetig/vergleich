use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Value {
    id: String,
    value: f32,
}

impl Value {}

pub struct Context<'a> {
    prefix: String,
    run: &'a mut ProgramRun,
}

impl<'a> Context<'a> {
    pub fn value(&mut self, name: &str, val: f32) -> f32 {
        let mut name2: String = self.prefix.clone();
        name2.push('.');
        name2.push_str(name);
        self.run.value(&name2, val);
        val
    }
}

impl<'b, 'a: 'b> Context<'a> {
    pub fn context(&'a mut self, name: &str) -> Context<'b> {
        let mut prefix = self.prefix.clone();
        prefix.push('.');
        prefix.push_str(name);
        Context {
            prefix,
            run: self.run,
        }
    }
}

pub struct ProgramRun {
    connection: Connection,
}

impl ProgramRun {
    pub fn new(filename: &str) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            connection: Connection::open_in_memory()?,
        })
    }
    pub fn context<'a>(&'a mut self, name: &str) -> Context<'a> {
        Context {
            prefix: name.into(),
            run: self,
        }
    }
    pub fn value(&mut self, name: &str, v: f32) -> f32 {
        println!("value {} {}", name, v);
        v
    }
}

#[cfg(test)]
mod tests {
    use crate::ProgramRun;

    #[test]
    fn smoke_test() -> Result<(), rusqlite::Error> {
        let mut p = ProgramRun::new("test.sqlite")?;
        p.value("outer", 42.0);
        let mut c = p.context("ctx");
        let value1 = c.value("val1", 17.2);
        let mut c2 = c.context("inner");
        let value2 = c2.value("val2", 33.3);
        println!("{} {}", value1, value2);
        Ok(())
    }
}
