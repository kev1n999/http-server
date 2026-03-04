use rusqlite::{Connection, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DebugPeople {
  id: i64,
  name: String,
  age: i64,
}

pub fn connect_db() -> Result<Connection> {
  let conn = Connection::open("peoples.db")?;
  create_db(&conn)?;
  Ok(conn)
}
fn create_db(conn: &Connection) -> Result<()> {
  conn.execute(
    "CREATE TABLE IF NOT EXISTS people (
      id INTEGER PRIMARY KEY,
      name TEXT NOT NULL UNIQUE,
      age INTEGER NOT NULL
    )",
  [],)?;
  Ok(())
}
pub fn insert_new_people(conn: &Connection, name: &str, age: &str) -> Result<()> {
  conn.execute("
    INSERT INTO People (name, age) VALUES (?1, ?2) ON CONFLICT(name) DO NOTHING
  ", [name, age],)?;
  Ok(())
}
pub fn delete_people(conn: &Connection, id: i64) -> Result<()> {
  conn.execute("
    DELETE FROM People WHERE id = ?1
  ", [id,],)?;
  Ok(())
}
pub fn fetch_people(conn: &Connection) -> Result<Vec<DebugPeople>>{
  let mut stmt = conn.prepare("SELECT id, name, age FROM people")?;
  let rows = stmt.query_map([], |row| {
    Ok(DebugPeople {
      id: row.get(0)?,
      name: row.get(1)?,
      age: row.get(2)?,
    })
  })?;
  let mut vec_people: Vec<DebugPeople> = Vec::new();
  for p in rows { vec_people.push(p?); }
  Ok(vec_people)
}
