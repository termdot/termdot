use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/resources/"]
#[include = "*.svg"]
pub struct Asset;
