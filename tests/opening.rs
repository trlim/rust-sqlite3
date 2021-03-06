#![feature(core, io, env)]

extern crate sqlite3;

use std::default::Default;
use std::env;

use sqlite3::{
    Access,
    DatabaseConnection,
    DatabaseUpdate,
    Query,
    ResultRowAccess,
    SqliteResult,
};
use sqlite3::access;
use sqlite3::access::flags::OPEN_READONLY;

pub fn main() {
    let args : Vec<String> = env::args().collect();
    let usage = "args: [-r] filename";

    let cli_access = {
        let ok = |&: flags, dbfile| Some(access::ByFilename { flags: flags, filename: dbfile });

        let arg = |&: n| {
            if args.len() > n { Some(args[n].as_slice()) }
            else { None }
        };

        match (arg(1), arg(2)) {
            (Some("-r"), Some(dbfile))
                => ok(OPEN_READONLY, dbfile),
            (Some(dbfile), None)
                => ok(Default::default(), dbfile),
            (_, _)
                => None
        }
    };

    fn use_access<A: Access>(access: A) -> SqliteResult<Vec<Person>> {
        let mut conn = try!(DatabaseConnection::new(access));
        make_people(&mut conn)
    }


    fn lose(why: &str) {
        env::set_exit_status(1);
        writeln!(&mut std::old_io::stderr(), "{}", why).unwrap()
    }

    match cli_access {
        Some(a) => match use_access(a) {
            Ok(x) => println!("Ok: {:?}", x),
            Err(oops) => lose(format!("oops!: {:?}", oops).as_slice())
        },
        None => lose(usage)
    }
}


#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
}

fn make_people(conn: &mut DatabaseConnection) -> SqliteResult<Vec<Person>> {
    try!(conn.exec("CREATE TABLE person (
                 id              SERIAL PRIMARY KEY,
                 name            VARCHAR NOT NULL
               )"));

    {
        let mut tx = try!(conn.prepare("INSERT INTO person (id, name)
                           VALUES (0, 'Dan')"));
        let changes = try!(conn.update(&mut tx, &[]));
        assert_eq!(changes, 1);
    }

    let mut stmt = try!(conn.prepare("SELECT id, name FROM person"));

    let mut ppl = vec!();
    try!(stmt.query(
        &[], &mut |row| {
            ppl.push(Person {
                id: row.get(0),
                name: row.get(1)
            });
            Ok(())
        }));
    Ok(ppl)
}

// Local Variables:
// flycheck-rust-library-path: ("../target")
// End:
