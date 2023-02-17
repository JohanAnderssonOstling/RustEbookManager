use std::fs;
use rusqlite::*;
use crate::{constant, library_model};
use std::path::PathBuf;
use library_model::ffi;
use crate::library_model::ffi::*;

pub struct LibraryDB {
	connection: rusqlite::Connection,
}


impl LibraryDB {
	pub fn open(uuid: &str) -> Self {
		let db_url = constant::DATA_DIR.join(uuid).join("library.db");
		let conn = Connection::open(db_url).unwrap();
		let mut library_db = LibraryDB {
			connection: conn
		};
		library_db.create_schema();
		library_db
	}
	pub fn create_schema(&self) {
		let folder_query = "CREATE TABLE IF NOT EXISTS FOLDER (
        folder_id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        parent_folder_id INTEGER NOT NULL
        );";
		self.connection.execute(folder_query, []).unwrap();


		let book_query = "CREATE TABLE IF NOT EXISTS BOOK (
        book_uuid TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        path TEXT NOT NULL,
        read_location TEXT DEFAULT \"0\",
        read_percentage INTEGER DEFAULT 0,
        title TEXT,
        isbn TEXT,
        folder_id INTEGER NOT NULL
        );";
		self.connection.execute(book_query, []).unwrap();
	}

//Bookqueries

	pub fn add_book(&self, book: &Book) -> Result<usize> {
		let query = "INSERT INTO BOOK (book_uuid, name, path, read_location, read_percentage, title, isbn, folder_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?)";
		let mut statement = self.connection.prepare(query)?;
		let result = statement.execute(params![book.uuid, book.name, book.path, book.read_location, book.read_percentage, book.title, book.isbn, book.folder_id]);
		result
	}

	pub fn add_books(&self, books: &Vec<Book>) -> Result<usize> {
		let mut result = 0;
		for book in books {
			result = self.add_book(book)?;
		}
		Ok(result)
	}

	pub fn select<T>(&self, query: &str, params: &[&dyn ToSql], func: fn(&rusqlite::Row) -> T) -> Vec<T> {
		let mut stmt = self.connection.prepare(query).unwrap();
		let mut rows = match stmt.query(params) {
			Ok(rows) => rows,
			Err(e) => panic!("Query {} failed with error: {}", query, e),
		};

		let mut result = Vec::new();
		while let Some(row) = rows.next().unwrap() {
			result.push(func(&row));
		}

		result
	}

	pub fn row_to_book(row: &rusqlite::Row) -> Book {
		Book {
			uuid: row.get(0).unwrap(),
			name: row.get(1).unwrap(),
			path: row.get(2).unwrap(),
			read_location: row.get(3).unwrap(),
			read_percentage: row.get(4).unwrap(),
			title: row.get(5).unwrap(),
			isbn: row.get(6).unwrap(),
			folder_id: row.get(7).unwrap(),
		}
	}

	pub fn row_to_folder(row: &rusqlite::Row) -> Dir {
		Dir {
			id: row.get(0).unwrap(),
			name: row.get(1).unwrap(),
			parent_id: row.get(2).unwrap(),
		}
	}

	pub fn get_books_by_folder(&self, folder_id: u32) -> Vec<Book> {
		let query = "SELECT * FROM BOOK where folder_id = ?";
		self.select(query, params![folder_id], LibraryDB::row_to_book)
	}

	pub fn delete_book(&self) {
		let query = "DELETE FROM BOOK WHERE book_uuid = ?";
	}

	pub fn set_book_location(&self, book_uuid: &str, location: &str, percentage: u32) {
		println!("set_book_location: {} {} {}", book_uuid, location, percentage);
		let query: &str = "UPDATE BOOK SET read_location = ?, read_percentage = ? WHERE book_uuid = ?";
		let mut stmt: Statement = self.connection.prepare(query).unwrap();
		stmt.execute(params![location, percentage, book_uuid])
			.expect("TODO: panic message");
	}

	pub fn get_book_location(&self, book_uuid: &str) -> ReadPosition{
		let query = "SELECT read_location, read_percentage FROM BOOK WHERE book_uuid = ?";
		let mut stmt = self.connection.prepare(query).unwrap();
		let mut rows = stmt.query(params![book_uuid]).unwrap();
		let row = rows.next().unwrap().unwrap();
		ReadPosition {
			read_location: row.get(0).unwrap(),
			read_percentage: row.get(1).unwrap(),
		}
	}

//Folderqueries

	pub fn add_folder(&self, folder: &Dir) {
		let query = "INSERT INTO FOLDER (name, parent_folder_id) VALUES (?, ?)";
		let mut stmt = self.connection.prepare(query).unwrap();
		stmt.execute(params![folder.name, folder.parent_id]).unwrap();
	}

	pub fn add_folders(&self, folders: &Vec<Dir>) {
		for folder in folders {
			self.add_folder(folder);
		}
	}

	pub fn get_folders(&self, parent_id: u32) -> Vec<Dir> {
		let query = "SELECT * FROM FOLDER WHERE parent_folder_id = ?";
		self.select(query, params![parent_id], LibraryDB::row_to_folder)
	}

	pub fn delete_folder(&self) {
		let query = "DELETE FROM FOLDER WHERE folder_id = ?";
	}

	pub fn delete_folders(&self) {
		let query = "DELETE FROM FOLDER";
		self.connection.execute(query, []).unwrap();
	}

	pub fn close(&self) {
		//self.connection.close().unwrap();
	}
}

#[cfg(test)]
mod tests{
	use super::*;

}