
use anyhow::{Result, bail};
use std::{env, fs, path::{self, Path, PathBuf}};

/// To open an existing database 
/// Open the file,
/// Read the header that contains pointers to the 
/// table definitions.
/// Read the table definitions into and array of table 
/// structures.
/// The structures are defined in the buffers section

use super::super::buffers;

#[derive(Default)]
pub struct ConnectDb {
    pub file_name: String,
    pub is_open: bool,
    pub is_dirty: bool,
    pub is_loaded: bool,
}


impl ConnectDb {
    pub fn connect_database() -> Result<Self> {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            println!("You must enter the database name!\n");
            bail!("No database entered!");
        }
        let mut connect = Self::default();
        connect.file_name = args[1].clone();
        // Check file exists
        if connect.path_exists(connect.file_name.clone()) {
            //open_file
        } else  {
            // Create a new file
        }

        Ok(connect)
    }

    fn path_exists(&mut self, file_name: String) -> bool {
        
        match Path::new(&file_name).try_exists() {
            Ok(exists) => {
                if exists == true {
                    true
                } else {
                    false
                }
            }
            Err(_) => {
                false
            }
        }
    }

    pub fn close_database(&self) -> Result<()> {

        Ok(())
    }
}