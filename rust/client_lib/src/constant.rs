
use lazy_static::lazy_static;
use directories::ProjectDirs;
use std::path::PathBuf;
lazy_static! {
    pub static ref PROJECT_DIRS: ProjectDirs = ProjectDirs::from("com", "kang", "bookshelf").unwrap();
    pub static ref DATA_DIR: PathBuf = PROJECT_DIRS.data_dir().to_path_buf();
    pub static ref LIBRARY_DIR: PathBuf = DATA_DIR.join("libraries");
    pub static ref DB_DIR: PathBuf = DATA_DIR.join("db");

    pub static ref HOME_DB_PATH: PathBuf = DB_DIR.join("home.db");


}


