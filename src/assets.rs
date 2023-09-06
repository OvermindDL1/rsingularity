#[derive(rust_embed::RustEmbed)]
#[folder = "assets/"]
#[exclude = "*.ogg"]
#[exclude = "*.mp3"]
pub struct Assets;

impl Assets {
	pub fn get_as_data_uri(path: &str) -> Result<String, String> {
		use base64::Engine;
		let file = Self::get(path).ok_or_else(|| format!("Failed to get asset: {path}"))?;
		let mimetype = file.metadata.mimetype();
		let data64 = base64::prelude::BASE64_STANDARD.encode(file.data);
		Ok(format!("data:{mimetype};base64,{data64}"))
	}

	// pub fn get_as_object_uri_or_path(path: &str) -> web_sys::Url {
	// 	web_sys::Url::create_object_url_with_blob(todo!()).unwrap()
	// }
}

#[cfg(feature = "embed-music")]
#[derive(rust_embed::RustEmbed)]
#[folder = "assets/music"]
pub struct AssetsMusic;

#[cfg(not(feature = "embed-music"))]
pub struct AssetsMusic;

impl AssetsMusic {
	// pub fn
}
