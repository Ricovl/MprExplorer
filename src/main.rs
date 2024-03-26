use bson::{document, Document};
use indexmap::map;
use rayon::{iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator}, ThreadPoolBuilder};
use rusqlite::Connection;
use std::{any::Any, collections::{HashMap, HashSet}, fs::OpenOptions, io::{Seek, Write}, num::NonZeroU64, path::Path, str::FromStr, sync::mpsc::channel, thread, time::Instant};
use uuid::Uuid;

use crate::structs::{forms::MicroflowAction, microflows::{MicroFlowObject, MicroflowActionType}, ModuleDocuments};

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
    doc: Option<structs::ModuleDocuments>,
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

            let _result: Result<ModuleDocuments, _> = bson::from_slice(slice);

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


struct Project {
    units: Vec<Unit>,
}

impl Project {
    fn open_new(path: &Path) -> Project {
        println!("Starting mpr read.");
        let conn = Connection::open(path).unwrap();
        let units = get_all_units(&conn).unwrap();

        Project { units }
    }



    fn get_module_id_by_name(&self, name: String) -> Uuid {
        let start = Instant::now();
        let modules = self.units.par_iter().filter(|unit| {
            match &unit.doc {
                Some(ModuleDocuments::ModuleImpl(imp)) => {
                    imp.name == name
                },
                _ => false,
                
            }
        });

        let module: Vec<_> = modules.collect();
        let module = module.first();
        module.unwrap().unit_id
    }

    fn list_modules(&self) {
        let start = Instant::now();
        let modules = self.units.par_iter().filter(|unit| {
            matches!(&unit.doc, Some(ModuleDocuments::ModuleImpl(_)))
        });

        let modules: Vec<_> = modules.collect();
        let duration = start.elapsed();
        println!("elapsed: {duration:?}");

        for module in modules {
            if let Some(ModuleDocuments::ModuleImpl(imp)) = &module.doc {
                println!("module: {}", imp.name);
            }
        }
    }

    fn list_commit_flows_withparams(self) {
        let start = Instant::now();
        let flows = self.units.par_iter().filter(|unit| {
            matches!(unit.doc, Some(ModuleDocuments::Microflow(_)))
        });

        let modules: Vec<_> = flows.collect();
        let duration = start.elapsed();
        println!("elapsed: {duration:?}");

        for module in modules {
            if let Some(ModuleDocuments::Microflow(imp)) = &module.doc {
                if imp.name.contains("CD_") && imp.name.contains("_Commit") {
                    println!("flow: {}", imp.name);

                    if let Some(object_collection) = &imp.object_collection {
                        for obj in &object_collection.objects {
                            if let MicroFlowObject::MicroflowParameter(parameter) = obj {
                                println!("\tparameter: {}", parameter.name);
                            }
                        }
                    }
                }
            }
        }
    }

    fn list_commit_usage_bp_flows(self) {
        let start = Instant::now();
        let flows = self.units.par_iter().filter(|unit| {
            matches!(unit.doc, Some(ModuleDocuments::Microflow(_)))
        });

        let modules: Vec<_> = flows.collect();
        let duration = start.elapsed();
        println!("elapsed: {duration:?}");

        for module in modules {
            if let Some(ModuleDocuments::Microflow(imp)) = &module.doc {
                if imp.name.contains("BP_") {
                    
                    if let Some(object_collection) = &imp.object_collection {
                        for obj in &object_collection.objects {
                            if let MicroFlowObject::ActionActivity(action) = obj {
                                if let Some(action) = &action.action {
                                    if let MicroflowActionType::MicroflowCallAction(action) = &action {
                                        if let Some(call) = &action.microflow_call {
                                            if call.microflow.contains("CD_Onboarding_") && call.microflow.contains("_Commit") {
                                                println!("flow: {}", imp.name);
                                                println!("\tcall: {}", call.microflow);
                                                
                                                let mappings = &call.parameter_mappings;
                                                for mapping in mappings {
                                                    if mapping.argument == "empty" {
                                                        println!("\t\tmapping: empty -> {}", mapping.parameter);
                                                    }
                                                    else {
                                                        println!("\t\tmapping: full -> {}", mapping.parameter);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn list_enity_parents(&self, parent: Uuid) {
        let start = Instant::now();
        let model = self.units.par_iter().find_first(|unit| {
            unit.container_id == parent && matches!(&unit.doc, Some(ModuleDocuments::DomainModel(_)))
        });

        let duration = start.elapsed();
        println!("elapsed: {duration:?}");

        if let Some(module) = model {
            if let Some(ModuleDocuments::DomainModel(imp)) = &module.doc {
                let assocations = &imp.associations;
                let entities = &imp.entities;

                for entity in entities {
                    println!("entity: {}", entity.name);

                    let associations = assocations.iter().filter(|assoc| {
                        assoc.parent_pointer == entity._id || assoc.child_pointer == entity._id
                    });
                    for assoc in associations {
                        let child = entities.iter().find(|ent| {
                            ent._id == assoc.child_pointer
                        });
                        // println!("\tassociation: {}", child.unwrap().name);
                        println!("\tp:{} c:{}, assoc: {}, {}, child: {}", assoc.parent_pointer == entity._id, assoc.child_pointer == entity._id ,assoc.name, assoc.owner, child.unwrap().name);
                    }
                }
            }
        }
    }

}


fn main() {
    let res = ThreadPoolBuilder::new().stack_size(4*1024*1024).build_global();
    let mpr = Path::new("resources/Pnew.mpr");

    let project = Project::open_new(mpr);

    let module_id = project.get_module_id_by_name("OnboardingV2".to_string());
    println!("module_id: {}", module_id);
    project.list_enity_parents(module_id);


}
