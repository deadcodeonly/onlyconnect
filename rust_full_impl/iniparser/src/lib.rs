extern crate sys_info;

use std::collections::HashMap;
use std::collections::LinkedList;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use sys_info::hostname;

pub struct Ini
{
    data: HashMap::<String, String>,
}

impl Ini {

    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Ini, String> {
        let name = filename.as_ref().to_string_lossy().to_string();
        if let Ok(file) = File::open(filename) {
            let mut aux_data = HashMap::<String,LinkedList::<(String,String)>>::new();
            let mut section = String::from("");
            for line in io::BufReader::new(file).lines() {
                if let Ok(s) = line {
                    let s = s.trim_start();
                    let sr = s.trim_end();
                    if s.starts_with('#') {
                        continue;
                    } else if s.starts_with('[') && sr.ends_with(']') {
                        section = sr.strip_prefix('[').unwrap_or("").strip_suffix(']').unwrap_or("").to_string();
                        aux_data.insert(section.clone(), LinkedList::<(String,String)>::new());
                    } else {
                        let (k,v) = sr.split_once('=').unwrap_or((sr,""));
                        if k.is_empty() {
                            continue;
                        }
                        match aux_data.get_mut(&section) {
                            Some(l) => {
                                l.push_back((k.to_string(),v.to_string()))
                            },
                            None => {
                                let mut l = LinkedList::<(String,String)>::new();
                                l.push_back((k.to_string(),v.to_string()));
                                aux_data.insert(section.clone(),l);
                            },
                        }
                    }
                } else {
                    return Err(String::from("Error reading file ") + &name + ".");
                }
            }
            let data = Self::parse_aux_data(aux_data)?;
            Ok(Ini{data})
        } else {
            Err(String::from("Error openeng file ") + &name + ".")
        }
    }

    fn parse_aux_data(aux_data: HashMap<String,LinkedList<(String,String)>>) -> Result<HashMap<String,String>,String> {
        if let Some(l) = aux_data.get("DEFAULT") {
            if let Ok(host) = hostname() {
                let mut section = "".to_string();
                for (k,v) in l {
                    if k.to_string() == host {
                        section = v.to_string();
                        break;
                    }
                }
                if ! section.is_empty() {
                    let mut data = HashMap::<String, String>::new();
                    let mut check_cicle = HashSet::<String>::new();
                    Self::load_ini(&section, &aux_data, & mut data, & mut check_cicle)?;
                    Ok(data)
                } else {
                    Err("Not found section to host ".to_string() + &host + ".")
                }
            } else {
                Err("Host name is unknow.".to_string())
            }
        } else {
            Err("DEFAULT section not found.".to_string())
        }
    }

    fn load_ini(key: &str, aux_data: &HashMap<String,LinkedList<(String,String)>>, data: & mut HashMap<String, String>, check_cicle: & mut HashSet<String>) -> Result<(),String> {
        if let Some(l) = aux_data.get(key) {
            for (k, v) in l {
                if k == "INCLUDE" {
                    if check_cicle.insert(key.to_string()) {
                        Self::load_ini(v, aux_data, data, check_cicle)?;
                    } else {
                        return Err("Ciclique reference found to ".to_string() + key + ".");
                    }
                } else {
                    data.insert(k.to_string(), v.to_string());
                }
            }
            Ok(())
        } else {
            Err("Section ".to_string() + key + " not found.")
        }
    }

    pub fn get(self: Self, key: &str) -> Result<String,String> {
        if let Some(v) = self.data.get(key) {
            Ok(v.to_string())
        } else {
            Err(key.to_string() + " not found.")
        }
    }

    pub fn get_i64(self: Self, key: &str) -> Result<i64,String> {
        if let Some(v) = self.data.get(key) {
            Ok(v.parse::<i64>().unwrap_or(0))
        } else {
            Err(key.to_string() + " not found.")
        }
    }

    pub fn get_f64(self: Self, key: &str) -> Result<f64,String> {
        if let Some(v) = self.data.get(key) {
            Ok(v.parse::<f64>().unwrap_or(0.0))
        } else {
            Err(key.to_string() + " not found.")
        }
    }

    pub fn get_list(self: Self, key: &str) -> Result<LinkedList<String>,String> {
        if let Some(v) = self.data.get(key) {
            let mut l = LinkedList::<String>::new();
            for i in v.split(',') {
                l.push_back(i.to_string());
            }
            Ok(l)
        } else {
            Err(key.to_string() + " not found.")
        }
    }

    pub fn get_map(self: Self, key: &str) -> Result<HashMap<String,String>,String> {
        if let Some(v) = self.data.get(key) {
            let mut m = HashMap::<String,String>::new();
            for i in v.split(',') {
                let (k,v) = i.split_once(':').unwrap_or((i,""));
                m.insert(k.to_string(), v.to_string());
            }
            Ok(m)
        } else {
            Err(key.to_string() + " not found.")
        }
    }

}

#[cfg(test)]
mod tests;
