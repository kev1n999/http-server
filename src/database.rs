use rusqlite::{Connection, Result};

pub fn connect_db() -> Result<Connection> {
  let conn = Connection::open("peoples.db")?;
  create_db(&conn)?;
  Ok(conn)
}
pub fn create_db(conn: &Connection) -> Result<()> {
  conn.execute("
    CREATE TABLE IF NOT EXISTS People (
      name TEXT,
      age INTEGER,
    )
  ", (),)?;
  Ok(())
}
pub fn insert_new_people(conn: Connection, name: &str, age: &str) -> Result<()> {
  conn.execute("
    INSERT INTO People (name, age) VALUES (?1, ?2)
  ", [name, age],)?;
  Ok(())
}
