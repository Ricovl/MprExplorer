use bson::Document;
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use rusqlite::Connection;
use serde::Serialize;
use serde_json::Value;
use std::{fs::{self}, io::Write, path::Path, str::FromStr};
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
            unit_id,
            container_id,
            containment_name: UnitType::from_str(containment_name.as_str()).unwrap(),
            tree_conflicts,
            contents_hash,
            contents_conflicts,
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

fn process_uuids(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for val in map.values_mut() {
                process_uuids(val);
            }
        }
        Value::Array(arr) => {
            if arr.len() == 16 && arr.iter().all(|n| n.is_u64()) {
                let bytes: Vec<u8> = arr
                    .iter()
                    .filter_map(|n| n.as_u64().and_then(|n| {
                        if n <= 255 {
                            Some(n as u8)
                        } else {
                            None
                        }
                    }))
                    .collect();
                if bytes.len() == 16 {
                    if let Ok(uuid) = Uuid::from_slice(&bytes) {
                        *value = Value::String(uuid.to_string());
                    }
                }
            } else {
                for item in arr.iter_mut() {
                    process_uuids(item);
                }
            }
        }
        _ => {}
    }
}

fn generate_structure(units: &[Unit], root_folder: &str) -> std::io::Result<()> {
    let root_unit = units.iter().find(|u| u.unit_id == u.container_id).expect("Root unit not found");

    let root_folder = Path::new(root_folder);

    if root_folder.exists() {
        fs::remove_dir_all(&root_folder)?;
    }
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
                    let file_name = match unit.containment_name {
                        UnitType::Documents => {
                            format!("{}.json", name)
                        },
                        UnitType::DomainModel | UnitType::ModuleSettings | UnitType::ModuleSecurity => {
                            format!("{:?}.json", unit.containment_name)
                        },
                        _ => {
                            format!("{:?}_{}.json", unit.containment_name, unit.unit_id)
                        }
                    };
                    let file_path = current_path.join(file_name);
                    let mut file = fs::File::create(&file_path)?;

                    let mut value = serde_json::to_value(&unit)?;
                    process_uuids(&mut value);
                    let json = serde_json::to_string_pretty(&value)?;
                    file.write_all(json.as_bytes())?;
                }
            };
            
        }

        Ok::<(), std::io::Error>(())
    }

    create_structure(root_unit, units, Path::new(root_folder))?;

    Ok(())
}


fn generate_structurev2(units: &[Unit], root_folder: &str) -> std::io::Result<()> {

    fs::create_dir_all(root_folder)?;

    units.par_iter().try_for_each(|unit| {
        let first_folder = &unit.unit_id.to_string()[0..2];
        let second_folder = &unit.unit_id.to_string()[2..4];
        println!("{}/{}/{}.mxunit", first_folder, second_folder, unit.unit_id);

        fs::create_dir_all(format!("{}/{}/{}/", root_folder, first_folder, second_folder))?;
        let mut file = fs::File::create(format!("{}/{}/{}/{}.mxunit", root_folder, first_folder, second_folder, unit.unit_id))?;
        file.write_all(&unit.contents)?;
        Ok::<(), std::io::Error>(())
    })?;

    Ok(())
}

fn main() {
    let mpr = "resources/plarge.mpr";
    let conn = Connection::open(mpr).unwrap();

    println!("Starting mpr read.");
    let units = get_all_units(&conn).unwrap();
    println!("Finished mpr read.");

    // let root_folder = "resources/mprcontents";
    // generate_structurev2(&units, root_folder).unwrap();
    let root_folder = "/home/rico/MX/src";
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
