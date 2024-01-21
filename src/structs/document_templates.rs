use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ConditionSettings {
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "new_conditions")]
	new_conditions: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct DataView {
	#[serde(rename = "contents")]
	contents: document_templates::DataViewDropZone,
	#[serde(rename = "entity_ref")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataViewDropZone {
	#[serde(rename = "widget")]
	widget: document_templates::Table,
}

#[derive(Serialize, Deserialize)]
pub struct DocumentTemplate {
	#[serde(rename = "canvas_width")]
	canvas_width: i64,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "footer")]
	footer: NULL,
	#[serde(rename = "header")]
	header: NULL,
	#[serde(rename = "margin_bottom_in_inch")]
	margin_bottom_in_inch: f64,
	#[serde(rename = "margin_left_in_inch")]
	margin_left_in_inch: f64,
	#[serde(rename = "margin_right_in_inch")]
	margin_right_in_inch: f64,
	#[serde(rename = "margin_top_in_inch")]
	margin_top_in_inch: f64,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "page_height")]
	page_height: String,
	#[serde(rename = "page_width")]
	page_width: String,
	#[serde(rename = "ppi")]
	ppi: i64,
	#[serde(rename = "show_header_footer_first_page")]
	show_header_footer_first_page: bool,
	#[serde(rename = "style")]
	style: document_templates::Style,
	#[serde(rename = "toplevels")]
	toplevels: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct DynamicImageViewer {
	#[serde(rename = "default_image_name")]
	default_image_name: String,
	#[serde(rename = "entity_ref")]
	entity_ref: domain_models::DirectEntityRef,
	#[serde(rename = "height")]
	height: i64,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "use_thumbnail")]
	use_thumbnail: bool,
	#[serde(rename = "width")]
	width: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Style {
	#[serde(rename = "background_color")]
	background_color: String,
	#[serde(rename = "bold")]
	bold: bool,
	#[serde(rename = "border_color_bottom")]
	border_color_bottom: String,
	#[serde(rename = "border_color_left")]
	border_color_left: String,
	#[serde(rename = "border_color_right")]
	border_color_right: String,
	#[serde(rename = "border_color_top")]
	border_color_top: String,
	#[serde(rename = "border_style_bottom")]
	border_style_bottom: String,
	#[serde(rename = "border_style_left")]
	border_style_left: String,
	#[serde(rename = "border_style_right")]
	border_style_right: String,
	#[serde(rename = "border_style_top")]
	border_style_top: String,
	#[serde(rename = "border_width_bottom")]
	border_width_bottom: i64,
	#[serde(rename = "border_width_left")]
	border_width_left: i64,
	#[serde(rename = "border_width_right")]
	border_width_right: i64,
	#[serde(rename = "border_width_top")]
	border_width_top: i64,
	#[serde(rename = "custom_css")]
	custom_css: String,
	#[serde(rename = "font_color")]
	font_color: String,
	#[serde(rename = "font_family")]
	font_family: String,
	#[serde(rename = "font_size")]
	font_size: i64,
	#[serde(rename = "italic")]
	italic: bool,
	#[serde(rename = "override_background_color")]
	override_background_color: bool,
	#[serde(rename = "override_font_color")]
	override_font_color: bool,
	#[serde(rename = "override_font_family")]
	override_font_family: bool,
	#[serde(rename = "override_font_size")]
	override_font_size: bool,
	#[serde(rename = "override_font_style")]
	override_font_style: bool,
	#[serde(rename = "override_font_weight")]
	override_font_weight: bool,
	#[serde(rename = "text_align")]
	text_align: String,
}

#[derive(Serialize, Deserialize)]
pub struct Table {
	#[serde(rename = "cell_padding")]
	cell_padding: i64,
	#[serde(rename = "cell_spacing")]
	cell_spacing: i64,
	#[serde(rename = "column_weights")]
	column_weights: Vec<i64>,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "rows")]
	rows: Vec<document_templates::TableRow>,
	#[serde(rename = "style")]
	style: document_templates::Style,
}

#[derive(Serialize, Deserialize)]
pub struct TableCell {
	#[serde(rename = "col_span")]
	col_span: i64,
	#[serde(rename = "is_part_of_span")]
	is_part_of_span: bool,
	#[serde(rename = "row_span")]
	row_span: i64,
	#[serde(rename = "style")]
	style: document_templates::Style,
	#[serde(rename = "widget")]
	widget: document_templates::DynamicImageViewer,
}

#[derive(Serialize, Deserialize)]
pub struct TableRow {
	#[serde(rename = "cells")]
	cells: Vec<document_templates::TableCell>,
	#[serde(rename = "condition_settings")]
	condition_settings: document_templates::ConditionSettings,
}

