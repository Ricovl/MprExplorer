use bson::Document;
use convert_case::Casing;
use indexmap::IndexMap;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use rusqlite::Connection;
use std::{
    io::{Write}, collections::{BTreeMap, HashSet}, fmt::{self, Debug, Formatter}, fs::{OpenOptions, File}, str::FromStr
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


struct MendixType {
    name: String,
    indexed: bool,
    is_dead_end: bool,
    values: IndexMap<String, SpecialTypes>,
}

impl Debug for MendixType {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "pub struct(")?;
        write!(fmt, ")")
    }
}


impl MendixType {
    fn add_attribute(&mut self, attr_name: &str, attr_type: SpecialTypes) {
        match attr_type {
            SpecialTypes::Array(_, _) | SpecialTypes::Empty() => {
                self.is_dead_end = false;
            }
            _ => {},
        };
        self.values.insert(attr_name.to_string(), attr_type);
    }

    fn eat_other(&mut self, other: MendixType) {
        let mut self_values = self.values.iter_mut();
        let mut other_values = other.values.iter();

        loop {
            let other_value = other_values.next();

            if let Some((_other_key, other_type)) = other_value {
                let (_self_key, mut self_type) = self_values.next().unwrap();

                match self_type {
                    SpecialTypes::Array(_, map) => {
                        if let SpecialTypes::Array(_, other_vec) = other_type {
                            for val in other_vec {
                                map.insert(val.to_string());
                            }
                        }
                    },
                    SpecialTypes::Empty() => {
                        // self_type = &mut SpecialTypes::Normal("()".to_string());
                        // self_type = other_type.clone();
                        match other_type {
                            SpecialTypes::Array(val, map) => self_type = &mut SpecialTypes::Array(val.clone(), map.clone()),
                            SpecialTypes::Reference(refe) => self_type = &mut SpecialTypes::Reference(refe.clone()),
                            _ => (),
                        }
                    },
                    _ => ()
                }
            }
            else {
                break;
            }
        }
    }

    fn write(&self, file: &mut File) {
        writeln!(file, "#[derive(Serialize, Deserialize)]").unwrap();
        writeln!(file, "pub struct {} {{", self.name).unwrap();

        for (_attr_name, attr_type) in self.values.iter() {
            let attr_name = if _attr_name.eq("Type") { "_type".to_string() } else { _attr_name.to_case(convert_case::Case::Snake) };

            writeln!(file, "\t#[serde(rename = \"{}\")]", attr_name).unwrap();
            writeln!(file, "\t{}: {},", attr_name, attr_type).unwrap();
        }
        writeln!(file, "}}\n").unwrap();
    }

}

struct Module {
    name: String,
    types: BTreeMap<String, MendixType>,
}

impl Module {
    fn get_or_create<'a>(&'a mut self, name: &str) -> &'a mut MendixType {
        if !self.types.contains_key(name) { // https://users.rust-lang.org/t/fast-way-to-insert-or-update-for-hashmap/76355
            self.types.insert(name.to_string(), MendixType { name: name.to_string(),indexed: false, is_dead_end: true, values: IndexMap::new() });
        }
        self.types.get_mut(name).unwrap()
    }

    fn process_mendixtype(&mut self, mendix_type: MendixType) {
        if !self.types.contains_key(&mendix_type.name) {
            self.types.insert(mendix_type.name.clone(), mendix_type);
        }
        else {
            self.types.get_mut(&mendix_type.name).unwrap().eat_other(mendix_type);
        }
    }

    fn write(&self, modfile: &mut File) {
        let module_name = self.name.to_case(convert_case::Case::Snake);
        writeln!(modfile, "pub mod {};", module_name).unwrap();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(format!("src/structs/{}.rs", module_name))
            .unwrap();

        writeln!(file, "use serde::{{Deserialize, Serialize}};").unwrap();
        writeln!(file, "use super::*;\n").unwrap();

        for (_, mendix_type) in self.types.iter() {
            mendix_type.write(&mut file);
        }
    }
}

struct ModuleExplorer {
    modules: BTreeMap<String, Module>,
}

impl ModuleExplorer {
    fn get_or_create(&mut self, name: &str) -> &mut Module {
        // if self.modules.contains_key(name) { // https://users.rust-lang.org/t/fast-way-to-insert-or-update-for-hashmap/76355
        //     &self.modules[name]
        // }
        // else {
        //    let _name = name.to_string();
        //    &self.modules.insert(_name, Module { name: _name, types: Box::new(BTreeMap::new()) }).unwrap()
        // }
        if !self.modules.contains_key(name) { // https://users.rust-lang.org/t/fast-way-to-insert-or-update-for-hashmap/76355
            self.modules.insert(name.to_string(), Module { name: name.to_string(), types: BTreeMap::new() });
        }
        self.modules.get_mut(name).unwrap()
    }
    
    fn explore_mpr(&mut self, doc: & Document) -> String {
        let mut iter_doc = doc.iter();
        iter_doc.next();
        let _type = iter_doc.next().unwrap().1.as_str().unwrap();
        let (_modname, _structname) = _type.split_once("$").unwrap();

        let mut mendixtype = MendixType { name: _structname.to_string(), indexed: false, is_dead_end: true, values: IndexMap::new() };
    
        for (key, obj) in iter_doc {
            match obj {
                bson::Bson::Array(bson_array) => {
                    let mut set = HashSet::new();
                    
                    let mut iter = bson_array.iter();
                    let first = iter.next().unwrap().as_i32().unwrap(); // 1 = basic type (String, i64), 2 = ?, 3 = ?

                    for bson in iter {
                        let array_attr = match bson {
                            bson::Bson::Document(idoc) => self.explore_mpr(&idoc),
                            bson => bson_to_string(bson).to_string(),
                        };
                        set.insert(array_attr);
                    }
                    mendixtype.add_attribute(key, SpecialTypes::Array(first, set));
                }
                bson::Bson::Document(_doc) => {
                    let arg = self.explore_mpr(&_doc);
                    mendixtype.add_attribute(key, SpecialTypes::Reference(arg));
                }
                bson::Bson::Null => { 
                    mendixtype.add_attribute(key, SpecialTypes::Empty());
                }
                _bson => {
                    mendixtype.add_attribute(key, SpecialTypes::Normal(bson_to_string(_bson).to_string()));
                }
            };
        }

        let module = self.get_or_create(_modname);
        module.process_mendixtype(mendixtype);
    
        _type.to_string()
    }


    fn write_to_folder(&self, dir: &str) {
        let mut modfile: File = OpenOptions::new()
            .write(true)
            .create(true)
            .open("src/structs.rs")
            .unwrap();

        for (_, module) in self.modules.iter() {
            module.write(&mut modfile);
        }
    }

}

enum SpecialTypes {
    Normal(String),
    Array(i32, HashSet<String>),
    Reference(String),
    Empty(),
}

impl std::fmt::Display for SpecialTypes {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
        match self {
            SpecialTypes::Normal(value) => write!(f, "{}", value),
            SpecialTypes::Array(arraytype, _attributes) => {
                if _attributes.is_empty() {
                    write!(f, "Vec<UnknownType>")
                }
                else if arraytype.to_owned() == 1 {
                    let val = _attributes.iter().next().unwrap();
                    write!(f, "Vec<{}>", val)
                }
                else {
                    let mut attributes = String::new();
                    for attr in _attributes {
                        let (_module, structname) = attr.split_once("$").unwrap();
                        let module = _module.to_case(convert_case::Case::Snake);
                        attributes.push_str(format!("{}::{}, ", module, structname).as_str());
                        
                    }
                    write!(f, "Vec<{}>", attributes)
                }
            },
            SpecialTypes::Reference(_type) => {
                let (_module, structname) = _type.split_once("$").unwrap();
                let module = _module.to_case(convert_case::Case::Snake);
                write!(f, "{}::{}", module, structname)
            },
            SpecialTypes::Empty() => write!(f, "Empty"),
        }
    }
}

fn main() {
    let mpr = "resources/plarge.mpr";
    let conn = Connection::open(mpr).unwrap();

    println!("Starting mpr read.");
    let units = get_all_units(&conn).unwrap();

    println!("Finished reading mpr.");
    println!("Starting mpr file exploring.");

    let mut module_explr = ModuleExplorer { modules: BTreeMap::new() };
    for child in units {
        if let Some(doc) = &child.doc {
            module_explr.explore_mpr(&doc);
        }
    }

    
    let path = "src/structs";
    std::fs::remove_dir_all(path).unwrap();
    std::fs::create_dir(path).unwrap();
    
    println!("Finished exploring mpr file.");
    println!("Starting struct write.");
    

    module_explr.write_to_folder("test");
}
