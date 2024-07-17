use bson::Document;
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use rusqlite::Connection;
use serde::Serialize;
use std::{collections::{HashMap, HashSet}, fs::{self, OpenOptions}, io::{Seek, Write}, path::Path, str::FromStr};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize)]
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

#[derive(Clone, Serialize)]
struct Unit {
    unit_id: Uuid,
    container_id: Uuid, // unit_id of parent container (same as unit_id if unit is the root)
    containment_name: UnitType,
    tree_conflicts: u64,
    contents_hash: String,
    contents_conflicts: String,
    #[serde(skip)]
    contents: Vec<u8>,
    doc: Option<Document>,
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

    units
        .par_iter_mut()
        .for_each(|unit| unit.doc = Some(Document::from_reader(unit.contents.as_slice()).unwrap()));

    Ok(units)
}


fn bson_to_string(bson: &bson::Bson) -> &'static str {
    match bson {
        bson::Bson::Double(_) => "Double",
        bson::Bson::String(_) => "String",
        bson::Bson::Boolean(_) => "bool",
        bson::Bson::Null => "Null",
        bson::Bson::RegularExpression(_) => "RegEx",
        bson::Bson::JavaScriptCode(_) => "JavaScript",
        bson::Bson::JavaScriptCodeWithScope(_) => "JavaScriptScope",
        bson::Bson::Int32(_) => "i32",
        bson::Bson::Int64(_) => "i64",
        bson::Bson::Timestamp(_) => "Timestamp",
        bson::Bson::Binary(_) => "Binary",
        bson::Bson::ObjectId(_) => "ObjectId",
        bson::Bson::DateTime(_) => "DateTime",
        bson::Bson::Symbol(_) => "Symbol",
        bson::Bson::Decimal128(_) => "Decimal128",
        bson::Bson::Undefined => "Undefined",
        bson::Bson::MaxKey => "MinKey",
        bson::Bson::MinKey => "MaxKey",
        bson::Bson::DbPointer(_) => "DbPointer",
        _ => "unknown",
    }
}


fn generate_structure(units: &[Unit], root_folder: &str) -> std::io::Result<()> {
    let root_unit = units.iter().find(|u| u.unit_id == u.container_id).expect("Root unit not found");

    fs::create_dir_all(root_folder)?;

    // Recursive function to create structure
    fn create_structure(unit: &Unit, units: &[Unit], current_path: &Path) -> std::io::Result<()> {
        if let Some(doc) = &unit.doc {
            let name = if let Ok(name) = doc.get_str("Name") {
                name
            } else {
                "Unnamed"
            };
 
            let new_path = match unit.containment_name {
                UnitType::Folders | UnitType::Modules => {
                    let folder_path = current_path.join(name.to_string());
                    fs::create_dir_all(&folder_path)?;
                    folder_path
                },
                _ => {
                    current_path.to_path_buf()
                }
            };
            
            let children: Vec<_> = units.iter().filter(|u| u.container_id == unit.unit_id && u.unit_id != u.container_id).collect();
            children.par_iter().try_for_each(|child| {
                create_structure(child, units, &new_path)
            })?;
            
            match unit.containment_name {
                UnitType::Folders | UnitType::Modules => {
                },
                _ => {
                    let file_name = format!("{:?}_{}_{}.json", unit.containment_name, name, unit.unit_id);
                    let file_path = current_path.join(file_name);
                    let mut file = fs::File::create(&file_path)?;
                    // writeln!(file, "Unit ID: {}\nContainer ID: {}\nType: {:?}\n", 
                    // unit.unit_id, unit.container_id, unit.containment_name)?;

                    // if let Ok(pretty_json) = serde_json::to_string_pretty(doc) {
                    //     writeln!(file, "{}", pretty_json)?;
                    // }
                    // serde_json::to_writer_pretty(&mut file, doc)?;
                    let json = serde_json::to_string_pretty(unit)?;
                    file.write_all(json.as_bytes())?;
                }
            };
            
        }

        Ok(())
    }

    create_structure(root_unit, units, Path::new(root_folder))?;

    Ok(())
}

fn main() {
    let mpr = "resources/plarge.mpr";
    let conn = Connection::open(mpr).unwrap();

    println!("Starting mpr read.");
    let units = get_all_units(&conn).unwrap();
    println!("Finished mpr read.");

    let root_folder = "/home/rico/MX";
    generate_structure(&units, root_folder).unwrap();

    println!("Folder structure generated successfully!");
    // let mut types = HashSet::new();

    // for unit in units {
    //     if let Some(doc) = &unit.doc {
    //         let name = doc.get_str("$Type").unwrap();
    //         types.insert(name.to_string());
    //     }
    // }
    

    // for _type in types {
    //     println!("type: {}", _type);
    // }
}
