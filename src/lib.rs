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
        let connection = Connection::open(filename)?;
        if let Err(e) =
            connection.query_row("SELECT count(id) from vergleich_values", [], |_| Ok(()))
        {
            println!("Err 0 {}", e);
            //            match e {

            // assume the database is empty, create table
            connection.execute(
                "CREATE TABLE vergleich_values (id TEXT PRIMARY KEY, value FLOAT)",
                [],
            )?;
            //        }
        }
        Ok(Self { connection })
    }
    pub fn context<'b, 'a: 'b>(&'a mut self, name: &str) -> Context<'b> {
        Context {
            prefix: name.into(),
            run: self,
        }
    }
    pub fn value(&mut self, name: &str, v: f32) -> f32 {
        if let Ok(f) = self.connection.query_row(
            "SELECT value from vergleich_values where id=?1",
            params![name],
            |row| row.get::<_, f32>(0),
        ) {
            println!("old value {}", f);
            self.connection
                .execute(
                    "UPDATE vergleich_values SET value=?2 where id=?1",
                    params![name, v],
                )
                .unwrap_or_else(|e| {
                    println!("Error {}", e);
                    0
                });
        } else {
            self.connection
                .execute(
                    "INSERT INTO vergleich_values VALUES (?1, ?2)",
                    params![name, v],
                )
                .unwrap_or_else(|e| {
                    println!("Error {}", e);
                    0
                });
        }
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
        //p.value("conflict", 13.0);
        let value1 = c.value("val1", 17.2);
        let value2 = {
            let mut c2 = c.context("inner");
            c2.value("val2", 33.3)
        };
        //c.value("conflict", 14.0); // TODO: Is there a way to make this non-conflicting?
        let mut c3 = p.context("ctx3");
        c3.value("val", 2.0);
        p.value("no conflict", 13.0);
        println!("{} {}", value1, value2);
        Ok(())
    }
}
