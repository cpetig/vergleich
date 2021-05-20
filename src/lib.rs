use rusqlite::{params, Connection, Result};

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

// 'a lives longest (we should agree on a convention here)
impl<'c, 'b: 'c, 'a: 'b> Context<'a> {
    pub fn context(&'b mut self, name: &str) -> Context<'c> {
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
    epsilon: f32,
}

impl ProgramRun {
    pub fn new(filename: &str) -> Result<Self, rusqlite::Error> {
        let connection = Connection::open(filename)?;
        if let Err(e) =
            connection.query_row("SELECT count(id) from vergleich_values", params![], |_| Ok(()))
        {
            println!("Err 0 {}", e);
            //            match e {

            // assume the database is empty, create table
            connection.execute(
                "CREATE TABLE vergleich_values (id TEXT PRIMARY KEY, value FLOAT)",
                params![],
            )?;
            //        }
        }
        Ok(Self {
            connection,
            epsilon: 1e-7,
        })
    }
    pub fn context<'b, 'a: 'b>(&'a mut self, name: &str) -> Context<'b> {
        Context {
            prefix: name.into(),
            run: self,
        }
    }
    pub fn difference(name: &str, v_old: f32, v_new: f32) {
        println!("Variable {} old {} new {}", name, v_old, v_new);
    }
    pub fn value(&mut self, name: &str, v: f32) -> f32 {
        if let Ok(f) = self.connection.query_row(
            "SELECT value from vergleich_values where id=?1",
            params![name],
            |row| row.get::<_, f64>(0),
        ) {
            if ((f as f32) - v).abs() > self.epsilon {
                Self::difference(name, f as f32, v);
            }
            //println!("old value {}", f);
            self.connection
                .execute(
                    "UPDATE vergleich_values SET value=?2 where id=?1",
                    params![name, v as f64],
                )
                .unwrap_or_else(|e| {
                    println!("Error {}", e);
                    0
                });
        } else {
            self.connection
                .execute(
                    "INSERT INTO vergleich_values VALUES (?1, ?2)",
                    params![name, v as f64],
                )
                .unwrap_or_else(|e| {
                    println!("Error {}", e);
                    0
                });
        }
        // println!("value {} {}", name, v);
        v
    }
    pub fn set_epsilon(&mut self, val:f32) {
        self.epsilon = val;
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
        //p.value("conflict", 13.0);
        let _value1 = c.value("val1", 17.2);
        for i in 0..3 {
            let mut c4 = c.context(&i.to_string());
            c4.value("loopval", i as f32);
        }
        c.value("mid2", 14.0);
        let mut c3 = p.context("ctx3");
        c3.value("val", 2.0);
        p.value("no conflict", 13.0);
        // println!("{} {}", value1, value2);
        Ok(())
    }
}
