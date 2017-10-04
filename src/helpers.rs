
use select::document::Document;
use reqwest::Url;
use reqwest;
use std::io::Read;

use social::WEBSITE;


pub(crate) fn remove_clutter(toshorten: String) -> String {
	let toshorten = toshorten.replace("\t", "");
	let toshorten = toshorten.replace("\n", "");
	let toshorten = toshorten.replace("\"", "");
	let toshorten = toshorten.trim();

	return toshorten.to_string();
}



pub(crate) fn getcontent(url: &str) -> Result<Document, String> {
	let url = Url::parse(&url).map_err(|e| e.to_string())?;
	let mut resp = reqwest::get(url).map_err(|e| e.to_string() + "Could not connect to over.gg")?;

	let mut content = String::new();
	let _ = resp.read_to_string(&mut content);    

    Ok(Document::from(content.as_ref()))
}











pub(crate) fn fix_link(link: &str) -> String {
	let link = WEBSITE.to_string() + link;
	return link.to_string();
}