use crate::assets::Asset;
use tmui::font::mgr::FontManager;

#[inline]
pub fn load_fonts() {
    FontManager::register_font_loader(|manager| {
        Asset::iter()
            .filter(|file| file.ends_with(".ttf"))
            .for_each(|f| {
                if let Some(file) = Asset::get(&f) {
                    manager.load_data(&file.data);
                }
            });
    });
}
