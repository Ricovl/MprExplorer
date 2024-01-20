use bson::Document;
use convert_case::Casing;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use rusqlite::Connection;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{Seek, Write},
    str::FromStr,
};
use uuid::Uuid;

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
        bson::Bson::Double(_) => "f64",
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

enum SpecialTypes {
    Normal(String),
    Array(i32, Vec<String>),
    Reference(String),
}

fn explore<'a>(
    doc: &'a Document,
    types: &mut HashMap<String, Box<HashMap<String, Vec<(String, SpecialTypes)>>>>,
    depth: usize,
) -> &'a str {
    let mut attributes: Vec<(String, SpecialTypes)> = Vec::new();
    let mut dociter = doc.iter();
    dociter.next();
    let _type = dociter.next().unwrap().1.as_str().unwrap();
    let (modname, structname) = _type.split_once("$").unwrap();

    for (key, obj) in dociter {
        let typestr: SpecialTypes;
        match obj {
            bson::Bson::Array(arr) => {
                let mut array_attrs: Vec<String> = Vec::new();
                let mut iter = arr.iter();
                let first = iter.next().unwrap().as_i32().unwrap(); // 1 = basic type (String, i64), 2 = ?, 3 = ?
                for el in iter {
                    let array_attr = match el {
                        bson::Bson::Document(idoc) => explore(&idoc, types, depth + 1),
                        bson => bson_to_string(bson),
                    };
                    if !array_attrs.contains(&array_attr.to_string()) {
                        array_attrs.push(array_attr.to_string());
                    }
                }
                typestr = SpecialTypes::Array(first, array_attrs);
            }
            bson::Bson::Document(idoc) => {
                typestr = SpecialTypes::Reference(explore(&idoc, types, depth + 1).to_string());
            }
            bson => typestr = SpecialTypes::Normal(bson_to_string(bson).to_string()),
        }

        attributes.push((key.to_string(), typestr));
    }

    types
        .entry(modname.to_string())
        .or_insert(Box::new(HashMap::new()))
        .insert(structname.to_string(), attributes);
    _type
}

fn main() {
    let mpr = "resources/plarge.mpr";
    let conn = Connection::open(mpr).unwrap();

    println!("Starting mpr read.");
    let units = get_all_units(&conn).unwrap();

    println!("Finished reading mpr.");
    println!("Starting mpr file exploring.");
    let mut types: HashMap<String, Box<HashMap<String, Vec<(String, SpecialTypes)>>>> =
        HashMap::new();
    for child in units {
        if let Some(doc) = &child.doc {
            explore(doc, &mut types, 0);
        }
    }

    let path = "src/structs";
    std::fs::remove_dir_all(path).unwrap();
    std::fs::create_dir(path).unwrap();

    println!("Finished exploring mpr file.");
    println!("Starting struct write.");

    let mut modfile = OpenOptions::new()
        .write(true)
        .create(true)
        .open("src/structs.rs")
        .unwrap();

    for (_module_name, structs) in types {
        let module_name = _module_name.to_case(convert_case::Case::Snake);
        writeln!(modfile, "pub mod {};", module_name).unwrap();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("src/structs/{}.rs", module_name))
            .unwrap();

        writeln!(file, "use serde::{{Deserialize, Serialize}};").unwrap();
        writeln!(file, "use super::*;\n").unwrap();

        for (struct_name, attributes) in structs.iter() {
            writeln!(file, "#[derive(Serialize, Deserialize)]").unwrap();
            writeln!(file, "pub struct {} {{", struct_name).unwrap();

            for (_attr_name, attr_type) in attributes {
                let attr_name = _attr_name.to_case(convert_case::Case::Snake);
                writeln!(file, "\t#[serde(rename = \"{}\")]", attr_name).unwrap();
                match attr_type {
                    SpecialTypes::Normal(_type) => {
                        writeln!(file, "\t{}: {},", attr_name, _type).unwrap();
                    }
                    SpecialTypes::Array(_, _) => {
                        writeln!(file, "\t{}: Vec<>,", attr_name).unwrap();
                    },
                    SpecialTypes::Reference(_type) => {
                        let (_module, structname) = _type.split_once("$").unwrap();
                        let module = _module.to_case(convert_case::Case::Snake);
                        writeln!(file, "\t{}: {}::{},", attr_name, module, structname).unwrap();
                    },
                }
            }

            writeln!(file, "}}\n").unwrap();
        }
    }
    println!("Finished struct write.");
}
