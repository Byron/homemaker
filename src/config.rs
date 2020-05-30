extern crate serde;
extern crate toml;

use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::string::String;
use toml::value;

#[derive(Serialize, Deserialize, Debug)]
pub struct ManagedObject {
  pub source: String,
  pub destination: String,
  pub method: String,
  pub task: String,
  pub solution: String,
  pub dependencies: Vec<String>
}

impl fmt::Display for ManagedObject {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} {} {} {} {}", self.source, self.method, self.destination, self.task, self.solution)
  }
}

impl Default for ManagedObject {
  fn default() -> Self {
    ManagedObject { source: String::from(""),
                    destination: String::from(""),
                    method: String::from(""),
                    task: String::from(""),
                    solution: String::from(""),
                    dependencies: Vec::new()
    }
  }
}

#[derive(Deserialize, Clone)]
pub struct Config {
  #[serde(deserialize_with = "deserialize_files")]
  pub files: Vec<(String, value::Value)>,
}

impl Default for Config {
  fn default() -> Self {
    Config { files: Vec::new() }
  }
}

/*
  this is all such terrible rust please don't look at it
*/
impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let mut mos: Vec<ManagedObject> = Vec::new();
      for _f in self.files.iter() {
        let mut mo = ManagedObject::default();
        match _f.1.get("source") {
          None => (),
          Some(_x) =>  {
            mo.source = String::from(_x.as_str().unwrap());
          }
        }
        match _f.1.get("method") {
          None => (),
          Some(_x) => {
            if _x.as_str().unwrap() == "symlink" {
              mo.method = String::from("symlink");
            }
          }
        }
        match _f.1.get("destination") {
          None => (),
          Some(_x) => {
            mo.destination = String::from(_x.as_str().unwrap());
          }
        }
        mos.push(mo);
      }
      write!(f, "{:#?}", mos)
    }
}

pub fn deserialize_files<'de, D>(deserializer: D) -> Result<Vec<(String, value::Value)>, D::Error>
where
  D: Deserializer<'de>,
{
  let mut files: Vec<(String, value::Value)> = Vec::new();
  let raw_files: Vec<value::Table> = Deserialize::deserialize(deserializer)?;
  println!("HI!");
  let raw_tasks = raw_files.clone();
  for mut entry in raw_files {
    if let Some(name) = entry.remove("file") {
      println!("{}", name);
      if let Some(name) = name.as_str() {
        files.push((name.to_owned(), value::Value::Table(entry)));
      }
    }
  }
  for mut entry in raw_tasks {
    if let Some(name) = entry.remove("task") {
      println!("{}", name);
      if let Some(name) = name.as_str() {
        files.push((name.to_owned(), value::Value::Table(entry)));
      }
    }
  }
  Ok(files)
}

pub fn as_managed_objects(config: Config) -> Vec<ManagedObject> {
  let mut mos: Vec<ManagedObject> = Vec::new();
  for _f in config.files.iter() {
    let mut mo = ManagedObject::default();
    match _f.1.get("solution") {
      None => (),
      Some(_x) =>  {
        mo.solution = String::from(_x.as_str().unwrap());
      }
    }
    match _f.1.get("task") {
      None => (),
      Some(_x) =>  {
        mo.task = String::from(_x.as_str().unwrap());
      }
    }
    match _f.1.get("source") {
      None => (),
      Some(_x) =>  {
        mo.source = String::from(_x.as_str().unwrap());
      }
    }
    match _f.1.get("method") {
      None => (),
      Some(_x) => {
        if _x.as_str().unwrap() == "symlink" {
          mo.method = String::from("symlink");
        }
      }
    }
    match _f.1.get("destination") {
      None => (),
      Some(_x) => {
        mo.destination = String::from(_x.as_str().unwrap());
      }
    }
    mos.push(mo);
  }
  return mos;
}

/*
let config: Config = deserialize_file(matches.value_of("config").unwrap())?;
*/

fn open_config(file: &str) -> io::Result<fs::File> {
  fs::File::open(file)
}

pub fn deserialize_file(file: &str) -> Result<Config, String> {
  let mut contents = String::new();
  let g = match open_config(file) {
    Ok(_a) => _a,
    Err(e) => return Err(e.to_string())
  };
  let mut file_contents = BufReader::new(g);
  match file_contents.read_to_string(&mut contents) {
    Ok(v) => v,
    Err(_e) => 0
  };
  println!("file: {}", &file);
  toml::from_str(&contents).or_else(|e| Err(e.to_string()))
}
