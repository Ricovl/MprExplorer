use bson::{document, Document};
use rayon::{iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator}, ThreadPoolBuilder};
use rusqlite::Connection;
use std::{any::Any, collections::{HashMap, HashSet}, fs::OpenOptions, io::{Seek, Write}, num::NonZeroU64, str::FromStr, sync::mpsc::channel, thread, time::Instant};
use uuid::Uuid;

use crate::structs::MendixThings;

mod structs;

#[derive(Debug, PartialEq)]
enum UnitType {
    Root,
    Documents,
    DomainModel,
    Folders,
    ModuleSecurity,
    ModuleSettings,
    Modules,
    ProjectConversion,
    ProjectDocuments,
}

impl FromStr for UnitType {
    type Err = ();

    fn from_str(input: &str) -> Result<UnitType, Self::Err> {
        match input {
            "" => Ok(UnitType::Root),
            "Documents" => Ok(UnitType::Documents),
            "DomainModel" => Ok(UnitType::DomainModel),
            "Folders" => Ok(UnitType::Folders),
            "ModuleSecurity" => Ok(UnitType::ModuleSecurity),
            "ModuleSettings" => Ok(UnitType::ModuleSettings),
            "Modules" => Ok(UnitType::Modules),
            "ProjectConversion" => Ok(UnitType::ProjectConversion),
            "ProjectDocuments" => Ok(UnitType::ProjectDocuments),
            _ => Err(()),
        }
    }
}

struct Unit {
    unit_id: Uuid,
    container_id: Uuid, // unit_id of parent container (same as unit_id if unit is the root)
    containment_name: UnitType,
    tree_conflicts: u64,
    contents_hash: String,
    contents_conflicts: String,
    contents: Vec<u8>,
    doc: Option<structs::MendixThings>,
}

impl Unit {
    fn new(
        unit_id: Uuid,
        container_id: Uuid,
        containment_name: String,
        tree_conflicts: u64,
        contents_hash: String,
        contents_conflicts: String,
        contents: &Vec<u8>,
    ) -> Unit {
        Unit {
            unit_id: unit_id,
            container_id: container_id,
            containment_name: UnitType::from_str(containment_name.as_str()).unwrap(),
            tree_conflicts: tree_conflicts,
            contents_hash: contents_hash,
            contents_conflicts: contents_conflicts,
            contents: contents.to_vec(),
            // doc: Document::from_reader(contents.as_slice()).unwrap(),
            doc: None,
        }
    }
}

fn get_all_units(conn: &Connection) -> Result<Vec<Unit>, &str> {
    let query = "SELECT * FROM Unit";
    let mut stmt = conn.prepare(query).unwrap();

    let start = Instant::now();
    
    let mut units: Vec<Unit> = stmt
        .query_map([], |row| {
            Ok(Unit::new(
                Uuid::from_u128(row.get::<_, i128>(0).unwrap() as u128),
                Uuid::from_u128(row.get::<_, i128>(1).unwrap() as u128),
                row.get(2).unwrap(),
                row.get(3).unwrap(),
                row.get(4).unwrap(),
                row.get(5).unwrap(),
                &row.get(6).unwrap(),
            ))
        })
        .unwrap()
        .map(|unit| unit.unwrap())
        .collect();

    let duration = start.elapsed();
    println!("elapsed: {duration:?}");


    units.par_iter_mut().for_each( |unit| {
        let slice = unit.contents.as_ref();

            let _result: Result<MendixThings, _> = bson::from_slice(slice);

            match _result {
                Ok(obj) => {
                    unit.doc = Some(obj);
                },
                Err(e) => {
                    let test = Document::from_reader(slice).unwrap();
                    println!("==Document==\ndoc: {:?}\n==End Doc==", test);
                    println!("failed to process: {:?}", e);
                },
            }
    });
    let duration = start.elapsed();
    println!("elapsed: {duration:?}");
    // println!("failed {} out of {}", failed, i);
    println!("done processing all {} documents!", units.len());

    Ok(units)
}


fn main() {
    let res = ThreadPoolBuilder::new().stack_size(4*1024*1024).build_global();
    let mpr = "resources/plarge.mpr";
    let conn = Connection::open(mpr).unwrap();

    println!("Starting mpr read.");
    
    let units = get_all_units(&conn).unwrap();

    let start = Instant::now();
    let modules = units.par_iter().filter(|unit| {
        if let Some(thing) = &unit.doc {
            match thing {
                MendixThings::ModuleImpl(_) => true,
                _ => false,
            }
        }
        else {
            false
        }
    });

    let modules: Vec<_> = modules.collect();
    let duration = start.elapsed();
    println!("elapsed: {duration:?}");
    
    for module in modules {
        if let Some(MendixThings::ModuleImpl(imp)) = &module.doc {
            println!("module: {}", imp.name);
        }
    }

    // let mut types = HashSet::new();

    // let start = Instant::now();

    // for unit in units {
    //     if let Some(mendix_thing) = &unit.doc {
    //         match mendix_thing {
    //             MendixThings::Rule(rule) => {
    //                 println!("rule thing: {}", rule.name);
    //             },
    //             _ => (),
    //         }
    //     }
    // }

    // let duration = start.elapsed();
    // println!("elapsed: {duration:?}");

    

    // for _type in types {
    //     println!("type: {}", _type);
    // }
}
