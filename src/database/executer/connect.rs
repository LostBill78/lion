
use anyhow::{Result, bail};
use bincode::decode_from_reader;
use bitfield_struct::bitfield;
use std::{env, fs::{self, File, OpenOptions}, io::{self, BufReader, Read}, path::{self, Path, PathBuf}};
use crate::terminal::Terminal;

use super::super::buffers::Page;
use serde::{Serialize, Deserialize};
// //use bincode::{deserialize_from, serialize_into, Config};
/// To open an existing database 
/// Open the file,
/// Read the header that contains pointers to the 
/// table definitions.
/// Read the table definitions into and array of table 
/// structures.
/// The structures are defined in the buffers section

use super::super::buffers;

#[bitfield(u8)]
pub struct Settings {
    #[bits(5)]
    base: usize,
    pub is_open: bool,
    pub is_dirty: bool,
    pub is_loaded: bool,
}
#[derive(Default)]
pub struct ConnectDb {
    pub file_name: String,
    pub settings: Settings,
    pub page1: Page,
}

// What about using a Buffer eg.. BufWriter??
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
            // open_file
            // read the header and meta data
            connect.page1 = connect.open_file(connect.file_name.clone())?;
        } else  {
            // Create a new file
            // Insert a small header
            connect.create_file(connect.file_name.clone())?;
            connect.page1 = connect.open_file(connect.file_name.clone())?;
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

    fn open_file(&mut self, file_name: String) -> Result<Page> {

        let file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .open(file_name);
        match file {
            Ok(file) => {
                let page = self.read_metadata(file)?;
                self.settings.set_is_open(true);
                Ok(page)
            }
            Err(e) => bail!("Unable to open: {}", e),
        }
    }
    fn read_metadata(&mut self, mut file: File) -> Result<Page> {
        let mut page_vec: Vec<u8> = Vec::new();
        // now read the first 4096 bytes into this page_vec
        file.read_to_end(&mut page_vec).expect("Should be able to read data");
        // Terminal::print(format!("{}", page_vec.len()));
        // Terminal::execute()?;
        let mut page_array: [u8; 4096] = [0; 4096];
        page_array.copy_from_slice(&page_vec);
        Ok(Page { content: page_array })
    }

    fn create_file(&mut self, file_name: String) -> Result<File> {
        let file = OpenOptions::new()
                        .create(true)
                        .read(true)
                        .write(true)
                        .open(file_name);
        match file {
            Ok(file) => {
                self.write_basedata(&file);
                self.settings.set_is_open(true);
                Ok(file)
            }
            Err(e) => bail!("Unable to open: {}", e),
        }
    }
    fn write_basedata(&mut self, file: &File) -> Result<Page> {
        let mut page_ray: [u8; 4096] = [0; 4096];
        let pepper = "Bill Hayes created my database   Number 1.0";
        
        page_ray[..pepper.len()].copy_from_slice(pepper.as_bytes());
        page_ray[4095] = b'E';

        fs::write(self.file_name.clone(), &page_ray);
        // Return the resulting Page -- heap pointer.
        Ok(Page { content: page_ray } )
    }

    pub fn write_data(&self, data: Vec<u8>) -> Result<()> {
        let start_index = 4096 - data.len();
        let end_index = 4096;
        let mut destination = self.page1.content;
        let mut page_slice = &mut destination[start_index..end_index];
        page_slice.copy_from_slice(data.as_slice());
        fs::write(self.file_name.clone(), self.page1.content);
        Ok(())
    }
    pub fn close_database(&self) -> Result<()> {
        // Add code when using BufWriter??
        // Make sure every update to database has been written

        Ok(())
    }
}