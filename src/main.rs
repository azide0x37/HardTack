extern crate postgres;

use postgres::{Connection, SslMode};

struct Offender {
    doc_id: i32,
    name: String,
}

fn main() {
    let conn = Connection::connect("postgres://wicebfabsysqtu:O-8yWv1vVfdYaG-TsvIMAVOTvR@ec2-50-17-249-73.compute-1.amazonaws.com:5432/d4iqsk43frobnr

", &SslMode::None).unwrap();

    conn.execute("CREATE TABLE offenders (
    doc_id   SERIAL PRIMARY KEY,
    name    VARCHAR NOT NULL
    )", &[]).unwrap();

    let me = Offender {
        doc_id: 666666,
        name: "Friggin' the devil, okay?".to_string()
    };

    conn.execute("INSERT INTO offenders (doc_id, name) VALUES ($1, $2)", &[&me.doc_id, &me.name]).unwrap();

    let stmt = conn.prepare("SELECT id, name FROM offenders").unwrap();
    for row in stmt.query(&[]).unwrap() {
        let offender = Offender {
            doc_id: row.get(0),
            name: row.get(1)
        };
        
        println!("Found person {}", offender.name);
    }
}
