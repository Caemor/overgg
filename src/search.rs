//! Search for Players, Teams and Events on over.gg
//use select;
//use select::predicate::{Class};
use select::predicate::{Attr, Class, Name};
use helpers::{remove_clutter, getcontent};

#[derive(Debug)]
pub enum SearchResult {
    Player(Player),
    Team(Team),
    Event(Event)
}

#[derive(Debug)]
pub struct Player {
	id: u32,
	url: String,
	name: String,
	real_name: Option<String>
}

#[derive(Debug)]
pub struct Team {
   	id: u32,
	url: String,
	name: String
}

#[derive(Debug)]
pub struct Event {
    id: u32,
	url: String,
	name: String,
	add_info: Option<String>,
}

pub fn get(search_term: &str) -> Result<Vec<SearchResult>, String> {
	search(search_term, None)
}

/// Returns the first x upcoming matches from over.gg
pub fn get_x(search_term: &str, number_of_results: u32) -> Result<Vec<SearchResult>, String> {
	search(search_term, Some(number_of_results))
}





fn search(search_term: &str, number_of_results: Option<u32>) -> Result<Vec<SearchResult>, String> {
	let mut search_results: Vec<SearchResult> = Vec::new();

	let document = getcontent(&("https://www.over.gg/search/?q=".to_string() + search_term))?;

	//Sets the limit to the amount of search results (if there is none give it is set to 10)
	let limit = number_of_results.unwrap_or(10);

	for node in document.find(Attr("id", "content")).take(1) {
		
		let number_of_results = get_number_of_results(node.find(Name("div")).next().unwrap().text());
		if number_of_results == 0 {
			return Ok(search_results);
		}


		println!("{:?}", number_of_results);

		for element in node.find(Class("wf-module-item")).take(limit as usize) {

			let name = element.find(Class("search-item-title")).next().map(|n| remove_clutter(n.text())).ok_or("Could not find name")?;

			let add_info = element.find(Class("search-item-desc")).next().map(|n| remove_clutter(n.text()));

			let url = element.attr("href").ok_or("Could not the url in search")?;

			let v: Vec<&str> = url.split("/").collect();

			let url = "https://over.gg".to_string() + url;


			if v.len() < 3 {
				continue;
			} else {
				let id = v[2].parse::<u32>().map_err(|e| e.to_string())?;

				match v[1] {
					"team" => {
						let team = Team {
							id, url, name
						};
						search_results.push(SearchResult::Team(team));
					},
					"event" => {
						let add_info = add_info.map(|s| s.replace("event– ", ""));
						let event = Event {
							id, url, name, add_info
						};
						search_results.push(SearchResult::Event(event));
					},
					"player" => {
						let real_name = add_info;
						let player = Player {
							id, url, name, real_name
						};
						search_results.push(SearchResult::Player(player));
					},
					x => {
						return Err(format!("New Searchterm: {:?}. Search needs to be updated. Please message Bot developer.", x));
					},
				}
			}
		}

	}
	Ok(search_results)
}


fn get_number_of_results(toshorten: String) -> i32 {
	let toshorten = toshorten.replace("\t", "");
	let toshorten = toshorten.replace("\n", "");
	let toshorten = toshorten.replace("\"", "");
	let toshorten = toshorten.replace("Found ", "");
	let toshorten = toshorten.replace(" results", "");
	let toshorten = toshorten.replace(" result", "");
	let toshorten = toshorten.trim();

	return toshorten.to_string().parse::<i32>().unwrap();
}

fn teamname_previously(toshorten: &str) -> String {
	toshorten.replace("previously", " previously ")
}

/*

command!(search(_ctx, msg, args) {	




    let mut answer = String::new();


	for node in document.find(Attr("id", "content")).take(1) {
		let number_of_results = get_number_of_results(node.find(Name("div")).next().unwrap().text());

		if number_of_results == 0 {
			embed_error(msg.channel_id, "No result found for this searchterm");
			return Ok(());
		} else {

			let response = MessageBuilder::new()
								.push(number_of_results)
								.push(" Results found.")
								.build();
			answer.push_str(response.as_ref());

			

			let mut show_number = 10 as usize;
			if number_of_results < 10 {
				show_number = number_of_results as usize;
			} else if number_of_results > 10 {
				answer.push_str("Output is limited to the first 10 results.")
			}

			answer.push_str("\n\n");

			for element in node.find(Class("wf-module-item")).take(show_number) {
				let name = element.text();
				let name = name.trim();
				let name = teamname_previously(name.to_string());
				let name = remove_clutter(name.to_string());
				let url = element.attr("href").unwrap();
				let v: Vec<&str> = url.split("/").collect();


				if v.len() < 3 {
					continue;
				} else {
					match v[1] {
						"team" => {
							let response = MessageBuilder::new()
								.push("**Team: ")
								.push(name.clone())
								.push("**\n")
								.push("\tTeamID for over.gg is: *")
								.push(v[2])
								.push("*\n\n")
								.build();
							answer.push_str(response.as_ref());
						},
						"event" => {
							let a = name.clone();//.replace("event", "");
							let splitname: Vec<&str> = a.split("event–").collect();


							if splitname.len() < 2 {
								let response = MessageBuilder::new()
									.push("**Event: ")
									.push(name.clone())
									.push("**\n")
									.push("\tEvent ID for over.gg is: *")
									.push(v[2])
									.push("*\n\n")
									.build();
								answer.push_str(response.as_ref());
							} else {
								let response = MessageBuilder::new()
									.push("**Event: ")
									.push(splitname[0])
									.push("**\n\t")
									.push(splitname[1].trim())
									.push("\n\tEvent ID for over.gg is: *")
									.push(v[2])
									.push("*\n\n")
									.build();
								answer.push_str(response.as_ref());
							}
							
						},
						_ => {
							let response = MessageBuilder::new()
								.push("Error for: ")
								.push(name.clone())
								.push("\n")
								.push("Please message Bot developer\n\n")
								.build();
							answer.push_str(response.as_ref());
						}
					}
				}
			}


			answer.push_str("\nUse ~team <id> or ~event <id> for more information about a specific event or team");
			embed_succes(msg.channel_id, answer.as_ref());
		}
	}

	//String search Found XXX Results

	//if XXX == 0

		//No Results found for this searchterm

	//else

		//for XXX > 10 show first 10 elements

		//else show all elements

		//

		// CLASS: wf-module-item
		// ID from href
		// Logo from img src
		// search-item-title (team name)
		// search-item-desc (previous/current team)
	
});
*/