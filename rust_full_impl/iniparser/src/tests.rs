use super::*;

use std::fs::{File, remove_file};
use std::io::prelude::*;
use sys_info::hostname;

#[test]
fn no_have_file() {
    match Ini::new("xpto.xpto") {
        Ok(_ini) => panic!("Ok to file xpto.xpto not found."),
        Err(s) => assert_eq!(s, String::from("Error openeng file xpto.xpto.")),
    }
}

fn no_host_config() -> &'static str {
    r###"
[DEFAULT]
nicas=DEV
[DEV]
TESTE_NUM=1
    "###
}

#[test]
fn have_file_no_host() {
    if let Ok(mut file) = File::create("xpto.no_host") {
        writeln!(file, "{}", no_host_config()).unwrap();
    } else {
        panic!("Error creating xpto.no_host file.");
    }
    let host = hostname().unwrap();
    match Ini::new("xpto.no_host") {
        Ok(_ini) => {
            remove_file("xpto.no_host").unwrap();
            panic!("Ok to file xpto.no_host.")
        },
        Err(s) => assert_eq!(s, "Not found section to host ".to_string() + &host + "."),
    }
    remove_file("xpto.no_host").unwrap();
}

fn host_config() -> String {
    let host = hostname().unwrap();
    "[DEFAULT]\n".to_string() + &host + "=DEV" + &r###"
    [DEV]
        #COMENT=coment
        STRING=teste de string
        NUM_INTEGER=10
        NUM_FLOAT=10.5
        LIST=element1,element2,element3
        MAP=key1:element1,key2:element2,key3:element3
    "###.to_string()
}

#[test]
fn have_file_host() {
    if let Ok(mut file) = File::create("xpto.ok") {
        writeln!(file, "{}", host_config()).unwrap();
    } else {
        panic!("Error creating xpto.ok file.");
    }
    match Ini::new("xpto.ok") {
        Ok(_ini) => (),
        Err(_) => {
            remove_file("xpto.ok").unwrap();
            panic!("Error in file xpto.ok.")
        },
    }
    remove_file("xpto.ok").unwrap();
}

#[test]
fn test_get() {
    if let Ok(mut file) = File::create("xpto.ok") {
        writeln!(file, "{}", host_config()).unwrap();
    } else {
        panic!("Error creating xpto.ok file.");
    }
    let ini = Ini::new("xpto.ok").unwrap();
    remove_file("xpto.ok").unwrap();
    assert_eq!("teste de string".to_string(), ini.get("STRING").unwrap());
}

#[test]
fn test_get_i64() {
    if let Ok(mut file) = File::create("xpto.ok") {
        writeln!(file, "{}", host_config()).unwrap();
    } else {
        panic!("Error creating xpto.ok file.");
    }
    let ini = Ini::new("xpto.ok").unwrap();
    remove_file("xpto.ok").unwrap();
    assert_eq!(10, ini.get_i64("NUM_INTEGER").unwrap());
}

#[test]
fn test_get_f64() {
    if let Ok(mut file) = File::create("xpto.ok") {
        writeln!(file, "{}", host_config()).unwrap();
    } else {
        panic!("Error creating xpto.ok file.");
    }
    let ini = Ini::new("xpto.ok").unwrap();
    remove_file("xpto.ok").unwrap();
    assert_eq!(10.5, ini.get_f64("NUM_FLOAT").unwrap());
}

#[test]
fn test_get_list() {
    if let Ok(mut file) = File::create("xpto.ok") {
        writeln!(file, "{}", host_config()).unwrap();
    } else {
        panic!("Error creating xpto.ok file.");
    }
    let ini = Ini::new("xpto.ok").unwrap();
    remove_file("xpto.ok").unwrap();
    let mut l = LinkedList::<String>::new();
    l.push_back("element1".to_string());
    l.push_back("element2".to_string());
    l.push_back("element3".to_string());
    assert_eq!(l, ini.get_list("LIST").unwrap());
}

#[test]
fn test_get_map() {
    if let Ok(mut file) = File::create("xpto.ok") {
        writeln!(file, "{}", host_config()).unwrap();
    } else {
        panic!("Error creating xpto.ok file.");
    }
    let ini = Ini::new("xpto.ok").unwrap();
    remove_file("xpto.ok").unwrap();
    let mut m = HashMap::<String, String>::new();
    m.insert("key1".to_string(), "element1".to_string());
    m.insert("key2".to_string(), "element2".to_string());
    m.insert("key3".to_string(), "element3".to_string());
    assert_eq!(m, ini.get_map("MAP").unwrap());
}
