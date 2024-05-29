use std::sync::Arc;

fn main() {
    let (font, i) = find_system_font("Apple Color Emoji");
    let face = harfbuzz_rs::Face::from_bytes(&font, i);
    let _ = face.table_with_tag(b"GSUB").is_some();
}

fn find_system_font(name: &str) -> (Arc<Vec<u8>>, u32) {
    let font = font_kit::source::SystemSource::new()
        .select_best_match(
            &[font_kit::family_name::FamilyName::Title(name.to_owned())],
            &font_kit::properties::Properties::new(),
        )
        .unwrap();
    match font {
        font_kit::handle::Handle::Path { .. } => panic!("did not expect path"),
        font_kit::handle::Handle::Memory { bytes, font_index } => (bytes, font_index),
    }
}
