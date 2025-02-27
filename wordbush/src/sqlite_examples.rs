use anyhow::Result;
use sqlite::State;

fn main() -> Result<()> {
    let connection = sqlite::open(":memory:")?;
    let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
    ";
    let res = connection.execute(query)?;
    println!("{:?}", res);

    // Method 1 to query
    let query = "SELECT * FROM users WHERE age > 50";

    connection
        .iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                println!("{} = {}", name, value.unwrap());
            }
            true
        })?;


    // Method 2 to query  
    let query = "SELECT * FROM users WHERE age > ?";
    let mut statement = connection.prepare(query)?;
    statement.bind((1, 50))?;
    while let Ok(State::Row) = statement.next() {
        println!("name = {}", statement.read::<String, _>("name")?);
        println!("age = {}", statement.read::<i64, _>("age")?);
    } 


    // Method 3 to query
    let query = "SELECT * FROM users WHERE age > ?";
    for row in connection
        .prepare(query)
        .unwrap()
        .into_iter()
        .bind((1, 50))?
        .map(|row| row.unwrap()){
            println!("name = {}", row.read::<&str, _>("name"));
            println!("age = {}", row.read::<i64, _>("age"));
        }

    Ok(())
}
