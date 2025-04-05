use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/resources/"]
#[include = "*.svg"]
#[include = "*.png"]
#[include = "*.json"]
#[include = "*.ttf"]
pub struct Asset;
