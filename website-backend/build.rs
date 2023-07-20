use camino::Utf8PathBuf;
use std::io::Write;

fn main() {
    // Create a db for local development and tests if it doesn't exist.
    let db_path = Utf8PathBuf::from("./local/local.db");
    if !db_path.exists() {
        // Make sure the local directory exists.
        let local_dir = db_path.parent().unwrap();
        if !local_dir.exists() {
            std::fs::create_dir_all(local_dir).unwrap();
        }
        // Create the db.
        let mut db = std::fs::File::create(db_path).unwrap();
        db.flush().unwrap();
    }
}
