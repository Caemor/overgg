//! Look for the latest news on over.gg
use select::predicate::{Class, Name};
use helpers::*;
use social::WEBSITE;



#[derive(Default, Debug, PartialEq)]
pub struct News {
	pub url: String,
	pub text: String,
	pub author: String,
	pub published: String
}

impl News {
	fn new(url: String, text: String, author: String, published: String) -> News {
		News {
			url, text, author, published
		}
	}
}

/// Returns all upcoming matches on the first site from over.gg
pub fn get_default() -> Result<Vec<News>, String> {
	news(10)
}

/// Returns the first x upcoming matches from over.gg
pub fn get_x(number_of_news: u32) -> Result<Vec<News>, String> {
	news(number_of_news)
}

lazy_static! {
	pub static ref URL: String = {
		let s = WEBSITE.to_string() + "/news";
		s
	};
}





fn news(number_of_news: u32) -> Result<Vec<News>, String> {
	let document = getcontent(&URL)?;

    let mut news_list: Vec<News> = Vec::new();

	for game in document.find(Class("list-table")).take(1) {
		let mut firstrow = true;
		for tablerow in game.find(Name("tr")).take((number_of_news + 1) as usize) {
			if firstrow {
				firstrow = false;
				continue;
			}
			let news = tablerow.find(Name("a")).next().ok_or("news.rs: Could not get Tablerow of url")?;
			let link = news.attr("href").ok_or("news.rs: Could not get url")?;
			let link: String = fix_link(link);
			let text = news.text();

			let mut counter = 0;
			let mut author = String::new();
			let mut published = String::new();
			for th in tablerow.find(Name("td")) {
				counter += 1;
				match counter {
					2 => {
						author = th.text();
					},
					3 => published = th.text(),
					_ => {},

				}

				trace!("{:?}", th.text());
			}
			author = remove_clutter(author).replace("by ", "");
			published = remove_clutter(published);

			let news = News::new(link, text, author, published);
			news_list.push(news);
		}
	}

	Ok(news_list)
}