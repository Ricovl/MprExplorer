#[derive(Serialize, Deserialize)]
struct MenuDocument {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "ItemCollection")]
	item_collection: Menus$MenuItemCollection,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
struct MenuItemCollection {
	#[serde(rename = "Items")]
	items: Vec<3, ["Menus$MenuItem"]>,
}

#[derive(Serialize, Deserialize)]
struct MenuItem {
	#[serde(rename = "Action")]
	action: Forms$MicroflowAction,
	#[serde(rename = "AlternativeText")]
	alternative_text: Null,
	#[serde(rename = "Caption")]
	caption: Texts$Text,
	#[serde(rename = "Icon")]
	icon: Null,
	#[serde(rename = "Items")]
	items: Vec<3, []>,
}

