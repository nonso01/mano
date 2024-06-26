#[derive(Debug)]
pub enum Auto {}

#[derive(Debug)]
pub enum Color {
    AUTO(Auto),
    RGB(i8, i8, i8, i8),
    HSL(i8, i8, i8, i8),
}

#[derive(Debug)]
pub enum Size {
    PX(isize),
    REM(isize),
    CM(isize),
    MM(isize),
    OTHERS(Todo),
}

#[derive(Debug)]
pub enum Time {
    SEC(isize),
    MIN(isize),
    HR(isize),
}

#[derive(Debug)]
pub enum Todo {}

#[derive(Debug)]
pub struct CSSStyleDeclaration {
    pub accent_color: Color,
    pub additive_symbols: Todo,
    pub align_content: Todo,
    pub align_items: Todo,
    pub align_self: Todo,
    pub alignment_baseline: Todo,
    pub all: Todo,
    pub animation: Todo,
    pub animation_composition: Todo,
    pub animation_delay: Todo,
    pub animation_direction: Todo,
    pub animation_fill_mode: Todo,
    pub animation_iteration_count: Todo,
    pub animation_name: Todo,
    pub animation_play_state: Todo,
    pub animation_range: Todo,
    pub animation_range_end: Todo,
    pub animation_range_start: Todo,
    pub animation_timeline: Todo,
    pub animation_timing_function: Todo,
    pub app_range: Todo,
    pub appearance: Todo,
    pub ascent_override: Todo,
    pub backdrop_filter: Todo,
    pub backface_visibility: Todo,
    pub background: Todo,
    pub background_attachement: Todo,
    pub background_blend_mode: Todo,
    pub background_clip: Todo,
    pub background_color: Color,
    pub background_image: Todo,
    pub background_origin: Todo,
    pub background_position: Todo,
    pub background_position_x: Todo,
    pub background_position_y: Todo,
    pub background_repeat: Todo,
    pub background_size: Todo,
    pub base_pallete: Todo,
    pub baseline_shift: Todo,
    pub baseline_source: Todo,
    pub block_size: Todo,
    pub border: Todo,
    pub border_block: Todo,
    pub border_block_color: Todo,
    pub border_block_end: Todo,
    pub border_block_end_color: Todo,
    pub border_block_end_style: Todo,
    pub border_block_end_width: Todo,
    pub border_block_style: Todo,
    pub border_block_width: Todo,
    pub border_bottom: Todo,
    pub border_bottom_color: Color,
    pub border_bottom_left_radius: Size,
    pub border_bottom_style: Todo,
    pub border_bottom_width: Todo,
    pub border_collapse: Todo,
    pub border_color: Todo,
    pub border_end_end_radius: Size,
    pub border_end_start_radius: Size,
    pub border_image: Todo,
    pub border_image_source: Todo,
    pub border_image_width: Size,
    pub border_inline: Todo,
    pub border_inline_color: Color,
    pub border_inline_end: Todo,
    pub border_inline_end_color: Color,
    pub border_inline_end_style: Todo,
    pub border_inline_end_width: Size,
    pub border_inline_start: Todo,
    pub border_inline_start_color: Color,
    pub border_inline_start_style: Todo,
    pub border_inline_start_width: Size,
    pub border_inline_style: Todo,
    pub border_inline_width: Size,
    pub border_left: Todo,
    pub border_left_color: Color,
    pub border_left_style: Todo,
    pub border_left_width: Size,
    pub border_radius: Size,
    pub border_right: Todo,
    pub border_right_color: Color,
    pub border_right_style: Todo,
    pub border_right_width: Size,
    pub border_spacing: Size,
    pub border_start_end_radius: Size,
    pub border_start_start_radius: Size,
    pub border_style: Todo,
    pub border_top: Todo,
    pub border_top_color: Color,
    pub border_top_left_radius: Todo,
    pub border_top_right_radius: Todo,
    pub border_top_style: Todo,
    pub border_top_width: Size,
    pub border_width: Size,
    pub bottom: Todo,
    pub box_shadow: Todo,
    pub box_sizing: Todo,
    pub break_after: Todo,
    pub beak_before: Todo,
    pub break_inside: Todo,
    pub buffered_rendering: Todo,
    pub caption_side: Todo,
    pub caret_color: Color,
    pub clear: Todo,
    pub clip: Todo,
    pub clip_path: Todo,
    pub clip_rule: Todo,
    pub color: Color,
    pub color_interpolation: Todo,
    pub color_interpolation_filters: Todo,
    pub color_rendering: Todo,
    pub color_scheme: Todo,
    pub column_count: Todo,
    pub column_fill: Todo,
    pub column_gap: Todo,
    pub column_rule: Todo,
    pub column_rule_color: Color,
    pub column_rule_style: Todo,
    pub column_rule_width: Todo,
    pub column_span: Todo,
    pub column_width: Size,
    pub columns: Todo,
    pub contain: Todo,
    pub contain_intrinsic_block_size: Todo,
    pub contain_intrinsic_height: Size,
    pub contain_intrinsic_inline_size: Todo,
    pub contain_intrinsic_size: Todo,
    pub contain_intrinsic_width: Size,
    pub container: Todo,
    pub container_name: Todo,
    pub container_type: Todo,
    pub content: Todo,
    pub content_visibility: Todo,
    pub counter_increment: Todo,
    pub counter_reset: Todo,
    pub counter_set: Todo,
    pub cursor: Todo,
    pub cx: Size,
    pub cy: Size,
    pub d: Todo,
    pub descent: Todo,
    pub direction: Todo,
    pub display: Todo,
    pub dominant_baseline: Todo,
    pub empty_cells: Todo,
    pub fallback: Todo,
    pub field_sizing: Todo,
    pub fill: Todo,
    pub fill_opacity: Todo,
    pub fill_rule: Todo,
    pub filter: Todo,
    pub flex: Todo,
    pub flex_basis: Todo,
    pub flex_direction: Todo,
    pub flex_flow: Todo,
    pub flex_grow: Todo,
    pub flex_shrink: Todo,
    pub flex_wrap: Todo,
    pub float: Todo,
    pub flood_color: Color,
    pub flood_opacity: Todo,
    pub font: Todo,
    pub font_display: Todo,
    pub font_family: Todo,
    pub font_feature_settings: Todo,
    pub font_kerning: Todo,
    pub font_optical_sizing: Todo,
    pub font_palette: Todo,
    pub font_sizing: Todo,
    pub font_stretch: Todo,
    pub font_style: Todo,
    pub font_synthesis: Todo,
    pub font_synthesis_small_caps: Todo,
    pub font_synthesis_style: Todo,
    pub font_synthesis_weight: Size,
    pub font_variant: Todo,
    pub font_variant_alternates: Todo,
    pub font_variant_caps: Todo,
    pub font_variant_east_asians: Todo,
    pub font_variant_ligatures: Todo,
    pub font_variant_numeric: Todo,
    pub font_variant_position: Todo,
    pub font_variation_settings: Todo,
    pub font_weight: Todo,
    pub forced_color_adjust: Todo,
    pub gap: Todo,
    pub grid: Todo,
    pub grid_area: Todo,
    pub grid_auto_columns: Todo,
    pub grid_auto_flow: Todo,
    pub grid_auto_rows: Todo,
    pub grid_column: Todo,
    pub grid_column_end: Todo,
    pub grid_column_gap: Todo,
    pub grid_column_start: Todo,
    pub grid_gap: Todo,
    pub grid_row: Todo,
    pub grid_row_end: Todo,
    pub grid_row_gap: Todo,
    pub grid_row_start: Todo,
    pub grid_template: Todo,
    pub grid_template_areas: Todo,
    pub grid_template_columns: Todo,
    pub grid_template_rows: Todo,
    pub height: Size,
    pub hyphenate_character: Todo,
    pub hyphenate_limit_chars: Todo,
    pub hyphens: Todo,
    pub image_orientation: Todo,
    pub image_rendering: Todo,
    pub inherits: Todo,
    pub initial_letter: Todo,
    pub initial_value: Todo,
    pub inline_size: Todo,
    pub inset: Todo,
    pub inset_area: Todo,
    pub inset_block: Todo,
    pub inset_block_start: Todo,
    pub inset_inline: Todo,
    pub inset_inline_end: Todo,
    pub inset_inline_start: Todo,
    pub isolation: Todo,
    pub justify_content: Todo,
    pub justify_items: Todo,
    pub justify_self: Todo,
    pub left: Todo,
    pub letter_spacing: Todo,
    pub lighting_color: Todo,
    pub line_break: Todo,
    pub line_gap_override: Todo,
    pub line_height: Size,
    pub list_style: Todo,
    pub list_style_image: Todo,
    pub list_style_position: Todo,
    pub list_style_type: Todo,
    pub margin: Todo,
    pub margin_block: Todo,
    pub margin_block_end: Todo,
    pub margin_block_start: Todo,
    pub margin_bottom: Todo,
    pub margin_inline: Todo,
    pub margin_inline_end: Todo,
    pub margin_inline_start: Todo,
    pub margin_left: Todo,
    pub margin_right: Todo,
    pub margin_top: Todo,
    pub marker: Todo,
    pub marker_end: Todo,
    pub marker_mid: Todo,
    pub marker_start: Todo,
    pub mask: Todo,
    pub mask_clip: Todo,
    pub mask_composite: Todo,
    pub mask_image: Todo,
    pub mask_mode: Todo,
    pub mask_origin: Todo,
    pub mask_position: Todo,
    pub mask_repeat: Todo,
    pub mask_size: Todo,
    pub mask_type: Todo,
    pub math_depth: Todo,
    pub math_shift: Todo,
    pub math_style: Todo,
    pub max_block_size: Todo,
    pub max_height: Size,
}
