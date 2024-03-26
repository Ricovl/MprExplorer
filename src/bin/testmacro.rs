use proc_macros::{IdentifiableImpl};


#[derive(IdentifiableImpl)]
struct Microflow {
    name: String,
    #[identifiable_field]
    actions: Vec<MicroflowAction>,
}

// impl Identifiable for Microflow {
//     fn identifiers(&self) -> Vec<Identifier> {
//         let mut identifiers = Vec::new();
//         for item in &self.actions {
//             identifiers.extend(item.identifiers());
//         }
//         identifiers
//     }
// }


enum MicroflowAction {
    MicroflowCallAction(MicroflowCallAction),
    // Other action types...
}

impl Identifiable for MicroflowAction {
    fn identifiers(&self) -> Vec<Identifier> {
        match self {
            MicroflowAction::MicroflowCallAction(microflow_call) => {
                microflow_call.identifiers()
            }
            // Other action types...
        }
    }
}

#[derive(IdentifiableImpl)]
struct MicroflowCallAction {
    #[identifiable_field]
    microflow_call: MicroflowCall,
    user_return_variable: bool,
}

// impl Identifiable for MicroflowCallAction {
//     fn identifiers(&self) -> Vec<Identifier> {
//         let identifiers = self.microflow_call.identifiers();
//         // Other identifiers...
//         identifiers
//     }
// }

struct MicroflowCall {
    queue: QueueIdentifier,
    microflow: Option<MicroflowIdentifier>,
}

impl Identifiable for MicroflowCall {
    fn identifiers(&self) -> Vec<Identifier> {
        let mut identifiers = Vec::new();
        identifiers.push(Identifier::QueueIdentifier(self.queue.clone()));
        if let Some(microflow) = &self.microflow {
            identifiers.push(Identifier::MicroflowIdentifier(microflow.clone()));
        }
        identifiers
    }
}



#[derive(Clone, Debug)]
struct MicroflowIdentifier {
    value: String,

}

#[derive(Clone, Debug)]
struct QueueIdentifier {
    value: String,
}

#[derive(Debug)]
enum Identifier {
    MicroflowIdentifier(MicroflowIdentifier),
    QueueIdentifier(QueueIdentifier),
    // Other identifier types...
}

trait Identifiable {
    fn identifiers(&self) -> Vec<Identifier>;
    fn identyfiers_of_type(&self, identifier_type: IdentifierType) -> Vec<Identifier> {
        self.identifiers().into_iter().filter(|identifier| match identifier {
            Identifier::MicroflowIdentifier(_) => identifier_type == IdentifierType::Microflow,
            Identifier::QueueIdentifier(_) => identifier_type == IdentifierType::Queue,
        }).collect()
    }
}

enum IdentifyBy {
    Identifier,
    Value,
}

#[derive(PartialEq, Eq)]
enum IdentifierType {
    Microflow,
    Queue,
    // Other identifier types...
}


fn main() {
    let microflow_identifier = MicroflowIdentifier {value: "Onboarding.Microflow".to_string()};
    let queue_identifier = QueueIdentifier {value: "Onboarding.Queue".to_string()};

    let microflow_call = MicroflowCall {
        queue: queue_identifier,
        microflow: Some(microflow_identifier),
    };

    let microflow_call_action = MicroflowCallAction {
        microflow_call: microflow_call,
        user_return_variable: false,
    };

    let microflow = Microflow {
        name: "Onboarding.Microflow".to_string(),
        actions: vec![MicroflowAction::MicroflowCallAction(microflow_call_action)],
    };

    let identifiers = microflow.identifiers();

    println!("identifiers: {:?}", identifiers);
}