use bson::Document;
use convert_case::Casing;
use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use rusqlite::Connection;
use std::{collections::HashMap, fs::OpenOptions, io::Write, str::FromStr};
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
        bson::Bson::Double(_) => "Double",
        bson::Bson::String(_) => "String",
        bson::Bson::Boolean(_) => "Bool",
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

fn explore<'a>(doc: &'a Document, types: &mut HashMap<String, Vec<(String, String)>>, depth: usize) -> &'a str {
    
    let mut attributes: Vec<(String, String)> = Vec::new();
    let mut dociter = doc.iter();
    dociter.next();
    dociter.next();

    for (key, obj) in dociter {
        let string;
        let typestr: &str;
        match obj {
            bson::Bson::Array(arr) => {
                let mut array_attrs: Vec<&str> = Vec::new();
                let mut iter = arr.iter();
                let first = iter.next().unwrap().as_i32().unwrap(); // 1 = basic type (String, i64), 2 = ?, 3 = ?
                for el in iter {
                    let array_attr = match el {
                        bson::Bson::Document(idoc) => {
                            explore(&idoc, types, depth + 1)
                        },
                        bson => {bson_to_string(bson)}
                    };
                    if !array_attrs.contains(&array_attr) {
                        array_attrs.push(array_attr);
                    }
                }
                string = format!("Vec<{}, {:?}>", first, array_attrs);
                typestr = string.as_str();
            },
            bson::Bson::Document(idoc) => {
                typestr = explore(&idoc, types, depth + 1);
            },
            bson => {typestr = bson_to_string(bson)},
        }

        attributes.push((key.to_string(), typestr.to_string()));

    }

    // firstly get the type
    let typename = doc.get_str("$Type").unwrap().to_string();
    if !types.contains_key(&typename) {
        types.insert(typename, attributes);
    }
    doc.get_str("$Type").unwrap()
}

fn main() {
    let mpr = "resources/plarge.mpr";
    let conn = Connection::open(mpr).unwrap();

    let units = get_all_units(&conn).unwrap();

    let mut types: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for child in units {
        if let Some(doc) = &child.doc {
            explore(doc, &mut types, 0);
        }
    }


    for (typename, attributes) in types {
        let split: Vec<&str> = typename.split("$").collect();

        let mut file = OpenOptions::new().write(true).append(true).create(true).open(format!("structs/{}.rs", split[0])).unwrap();

        writeln!(file, "#[derive(Serialize, Deserialize)]").unwrap();
        writeln!(file, "struct {} {{", split[1]).unwrap();

        for (attr_name, attr_type) in attributes {
            writeln!(file, "\t#[serde(rename = \"{}\")]", attr_name).unwrap();
            writeln!(file, "\t{}: {},", attr_name.to_case(convert_case::Case::Snake), attr_type).unwrap();
        }

        writeln!(file, "}}\n").unwrap();
    }


}
