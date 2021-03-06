// Purge all references to a crate from the database.
//
// Please be super sure you want to do this before running this.
//
// Usage:
//      cargo run --bin delete-crate crate-name

#![deny(warnings)]

extern crate cargo_registry;
extern crate diesel;

use diesel::prelude::*;
use std::env;
use std::io;
use std::io::prelude::*;

use cargo_registry::Crate;
use cargo_registry::schema::crates;

#[allow(dead_code)]
fn main() {
    let conn = cargo_registry::db::connect_now().unwrap();
    conn.transaction::<_, diesel::result::Error, _>(|| {
        delete(&conn);
        Ok(())
    }).unwrap()
}

fn delete(conn: &PgConnection) {
    let name = match env::args().nth(1) {
        None => {
            println!("needs a crate-name argument");
            return;
        }
        Some(s) => s,
    };

    let krate = Crate::by_name(&name).first::<Crate>(conn).unwrap();
    print!(
        "Are you sure you want to delete {} ({}) [y/N]: ",
        name,
        krate.id
    );
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    if !line.starts_with("y") {
        return;
    }

    println!("deleting the crate");
    let n = diesel::delete(crates::table.find(krate.id))
        .execute(conn)
        .unwrap();
    println!("  {} deleted", n);

    print!("commit? [y/N]: ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    if !line.starts_with("y") {
        panic!("aborting transaction");
    }
}
