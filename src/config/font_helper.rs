use crate::assets::Asset;
use tmui::font::mgr::FontManager;

pub fn load_fonts() {
    Asset::iter()
        .filter(|file| file.ends_with(".ttf"))
        .for_each(|f| {
            if let Some(file) = Asset::get(&f) {
                FontManager::load_data(&file.data);
            }
        });
}
