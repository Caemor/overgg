extern crate overgg;

use overgg::social::*;
use overgg::upcoming_matches;

#[test]
fn social() {
	assert_eq!(WEBSITE, "https://over.gg");
	assert_eq!(YOUTUBE, "https://www.youtube.com/c/overgg");
	assert_eq!(TWITTER, "https://twitter.com/overdotgg");
	assert_eq!(DISCORD, "https://discord.gg/yU7crSy");
	assert_eq!(TWITTER_HANDLE, "overdotgg");
}

#[test]
fn upcoming_matches() {
	assert_eq!(upcoming_matches::get_x(3).unwrap().len(), 3);
}

