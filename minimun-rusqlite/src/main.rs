use rusqlite::{Connection, Result};
use std::{env, path::PathBuf, usize};

const DATABASE: &str = "students.db3";

#[derive(Debug, PartialEq)]
struct Student {
    #[allow(dead_code)]
    id: i32,
    name: String,
    height: String,
}

fn main() -> Result<()> {
    // 使用 rusqlite 连接 sqlite 数据库
    let connection = Connection::open(compute_database_path())?;

    // 在 rusqlite 里使用 sqlite 语法创建表格
    create_table(&connection)?;

    // 在 rusqlite 里使用 sqlite 语法从表格中获取数据
    let database_students = get_students(&connection)?;

    // 如果表中不存在和要插入的数据相冋的数据，则插入数据。
    let students = prepare_raw_data();
    let exist = database_students.iter().any(|database_student| {
        students
            .iter()
            .any(|student| student.name == database_student.name)
    });

    if !exist {
        insert_students(&connection, students)?;
    }

    // 打印中表格中取出的数据
    let students = get_students(&connection)?;
    for student in students {
        println!("Found student {:?}", student);
    }

    Ok(())
}

fn compute_database_path() -> PathBuf {
    env::current_dir()
        .expect("Fail to get directory.")
        .join(DATABASE)
}

fn create_table(connection: &Connection) -> Result<usize> {
    connection.execute(
        "CREATE TABLE IF NOT EXISTS students (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            height TEXT NOT NULL 
        )",
        (),
    )
}

fn get_students(connection: &Connection) -> Result<Vec<Student>> {
    let mut statement = connection.prepare("SELECT id, name, height FROM students")?;

    let student_iterator = statement.query_map([], |row| {
        let student = Student {
            id: row.get(0)?,
            name: row.get(1)?,
            height: row.get(2)?,
        };

        Ok(student)
    })?;

    student_iterator.collect::<Result<Vec<Student>>>()
}

fn prepare_raw_data() -> Vec<Student> {
    [
        Student {
            id: 0,
            name: "下江コハル".to_string(),
            height: "148cm".to_string(),
        },
        Student {
            id: 1,
            name: "小鳥遊ホシノ".to_string(),
            height: "145cm".to_string(),
        },
        Student {
            id: 2,
            name: "浅黄ムツキ".to_string(),
            height: "144cm".to_string(),
        },
        Student {
            id: 3,
            name: "霞沢ミユ".to_string(),
            height: "149cm".to_string(),
        },
    ]
    .into_iter()
    .collect::<Vec<Student>>()
}

fn insert_students(connection: &Connection, students: Vec<Student>) -> Result<()> {
    let mut statement =
        connection.prepare("INSERT INTO students (name, height) VALUES (?1, ?2)")?;

    for student in students {
        statement.execute((&student.name, &student.height))?;
    }

    Ok(())
}

#[cfg(test)]
mod minimun_resqlite_tests {
    use super::*;

    #[test]
    fn database_connected() -> Result<()> {
        Connection::open(compute_database_path())?;
        Ok(())
    }

    #[test]
    fn table_created() -> Result<()> {
        let connection = Connection::open_in_memory()?;
        create_table(&connection)?;
        get_students(&connection)?;

        Ok(())
    }

    #[test]
    fn data_inserted() -> Result<()> {
        main()?;
        main()?;

        let connection = Connection::open(compute_database_path())?;
        let students = prepare_raw_data();
        let database_students = get_students(&connection)?;

        assert!(database_students.len() == students.len());

        Ok(())
    }
}
