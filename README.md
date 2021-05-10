# Vergleich: A library to compare results between program runs

(Vergleich [fair-gli-h] is German for comparison)

- Create a `ProgramRun` instance connecting to a sqlite3 database.
- Optionally stack some `context()`s
- Log `value()`s in the program run. If previous measurements exist they are compared. `value()` like `dbg!()` returns the value for easier use.

See the test in src/lib.rs for an example:

```Rust
let mut p = ProgramRun::new("test.sqlite")?;
p.value("outer", 42.0);
let mut c = p.context("ctx");
c.value("val1", 17.2);
let mut c2 = c.context("inner");
c2.value("val2", 33.3);
let mut c3 = p.context("ctx3");
c3.value("val", 2.0);
p.value("outer2", 13.0);
```
