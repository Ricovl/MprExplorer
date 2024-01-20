#[derive(Serialize, Deserialize)]
struct ConditionSettings {
	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "NewConditions")]
	new_conditions: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
struct Table {
	#[serde(rename = "CellPadding")]
	cell_padding: i64,
	#[serde(rename = "CellSpacing")]
	cell_spacing: i64,
	#[serde(rename = "ColumnWeights")]
	column_weights: Vec<1, ["i64"]>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Rows")]
	rows: Vec<2, ["DocumentTemplates$TableRow"]>,
	#[serde(rename = "Style")]
	style: DocumentTemplates$Style,
}

#[derive(Serialize, Deserialize)]
struct Style {
	#[serde(rename = "BackgroundColor")]
	background_color: String,
	#[serde(rename = "Bold")]
	bold: Bool,
	#[serde(rename = "BorderColorBottom")]
	border_color_bottom: String,
	#[serde(rename = "BorderColorLeft")]
	border_color_left: String,
	#[serde(rename = "BorderColorRight")]
	border_color_right: String,
	#[serde(rename = "BorderColorTop")]
	border_color_top: String,
	#[serde(rename = "BorderStyleBottom")]
	border_style_bottom: String,
	#[serde(rename = "BorderStyleLeft")]
	border_style_left: String,
	#[serde(rename = "BorderStyleRight")]
	border_style_right: String,
	#[serde(rename = "BorderStyleTop")]
	border_style_top: String,
	#[serde(rename = "BorderWidthBottom")]
	border_width_bottom: i64,
	#[serde(rename = "BorderWidthLeft")]
	border_width_left: i64,
	#[serde(rename = "BorderWidthRight")]
	border_width_right: i64,
	#[serde(rename = "BorderWidthTop")]
	border_width_top: i64,
	#[serde(rename = "CustomCss")]
	custom_css: String,
	#[serde(rename = "FontColor")]
	font_color: String,
	#[serde(rename = "FontFamily")]
	font_family: String,
	#[serde(rename = "FontSize")]
	font_size: i64,
	#[serde(rename = "Italic")]
	italic: Bool,
	#[serde(rename = "OverrideBackgroundColor")]
	override_background_color: Bool,
	#[serde(rename = "OverrideFontColor")]
	override_font_color: Bool,
	#[serde(rename = "OverrideFontFamily")]
	override_font_family: Bool,
	#[serde(rename = "OverrideFontSize")]
	override_font_size: Bool,
	#[serde(rename = "OverrideFontStyle")]
	override_font_style: Bool,
	#[serde(rename = "OverrideFontWeight")]
	override_font_weight: Bool,
	#[serde(rename = "TextAlign")]
	text_align: String,
}

#[derive(Serialize, Deserialize)]
struct TableRow {
	#[serde(rename = "Cells")]
	cells: Vec<2, ["DocumentTemplates$TableCell"]>,
	#[serde(rename = "ConditionSettings")]
	condition_settings: DocumentTemplates$ConditionSettings,
}

#[derive(Serialize, Deserialize)]
struct TableCell {
	#[serde(rename = "ColSpan")]
	col_span: i64,
	#[serde(rename = "IsPartOfSpan")]
	is_part_of_span: Bool,
	#[serde(rename = "RowSpan")]
	row_span: i64,
	#[serde(rename = "Style")]
	style: DocumentTemplates$Style,
	#[serde(rename = "Widget")]
	widget: DocumentTemplates$DynamicImageViewer,
}

#[derive(Serialize, Deserialize)]
struct DataView {
	#[serde(rename = "Contents")]
	contents: DocumentTemplates$DataViewDropZone,
	#[serde(rename = "EntityRef")]
	entity_ref: DomainModels$DirectEntityRef,
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
struct DocumentTemplate {
	#[serde(rename = "CanvasWidth")]
	canvas_width: i64,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Footer")]
	footer: Null,
	#[serde(rename = "Header")]
	header: Null,
	#[serde(rename = "MarginBottomInInch")]
	margin_bottom_in_inch: Double,
	#[serde(rename = "MarginLeftInInch")]
	margin_left_in_inch: Double,
	#[serde(rename = "MarginRightInInch")]
	margin_right_in_inch: Double,
	#[serde(rename = "MarginTopInInch")]
	margin_top_in_inch: Double,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "PageHeight")]
	page_height: String,
	#[serde(rename = "PageWidth")]
	page_width: String,
	#[serde(rename = "PPI")]
	ppi: i64,
	#[serde(rename = "ShowHeaderFooterFirstPage")]
	show_header_footer_first_page: Bool,
	#[serde(rename = "Style")]
	style: DocumentTemplates$Style,
	#[serde(rename = "Toplevels")]
	toplevels: Vec<3, ["DocumentTemplates$DataView"]>,
}

#[derive(Serialize, Deserialize)]
struct DynamicImageViewer {
	#[serde(rename = "DefaultImageName")]
	default_image_name: String,
	#[serde(rename = "EntityRef")]
	entity_ref: DomainModels$DirectEntityRef,
	#[serde(rename = "Height")]
	height: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "UseThumbnail")]
	use_thumbnail: Bool,
	#[serde(rename = "Width")]
	width: i64,
}

#[derive(Serialize, Deserialize)]
struct DataViewDropZone {
	#[serde(rename = "Widget")]
	widget: DocumentTemplates$Table,
}

