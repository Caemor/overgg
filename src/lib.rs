
extern crate select;
extern crate reqwest;
#[macro_use]
extern crate log;


pub mod upcoming_matches;
pub mod social;
pub mod search;
pub mod news;
mod helpers;









#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    	
    }
}








/*
Upcoming Match
-Link to Matchpage
-Name Team 1 + Flagge
-Score Team 1
-ETA
-Name Team 2 + Flagge
-Score Team 2



Team
-Link to Teampage
-Teamname
-Region
-Logo
-Roster (of multiple Players)
-Upcoming Matches
-Recent Matches

-Pricemoney?

-events

command!(matches(_ctx, message) {
	let urlstr = MessageBuilder::new()
    	.push("https://www.over.gg/matches")
    	.build();

    let channel_id = message.channel_id;



	let url = Url::parse(urlstr.clone().as_ref()).unwrap();
	let mut resp = reqwest::get(url).unwrap();
	//assert!(resp.status().is_success());

	// when 404 error comes
	if !resp.status().is_success() {
		embed_error(channel_id, "Could not connect to over.gg");
		return Ok(());
	}

	let mut content = String::new();
	let _ = resp.read_to_string(&mut content);
    

    let document = Document::from(content.as_ref());

   	let mut answer = String::new();


	answer.push_str(" __***Upcoming Matches:***__ \n\n"); 
	let mut counter = 0;

	for game in document.find(Class("match-item")) {
		println!("{:?}", remove_clutter(game.html()));
		let eventname = game.find(Class("match-item-event-name")).next().unwrap().text();
		let eventsub = game.find(Class("match-item-event-sub")).next().unwrap().text();
		let mut one = true;
		let mut eventopponent1 = String::new();
		let mut eventopponent2 = String::new();
		for team in game.find(Class("match-item-vs-team-name")).take(2) {
			if one {
				eventopponent1 = team.text();
				one = false;
			} else {
				eventopponent2 = team.text();
			}

		}
		let eventtime = game.find(Class("match-item-eta")).next().unwrap().text();

		let eventname = remove_clutter(eventname);
		let eventsub = remove_clutter(eventsub);
		let eventopponent1 = remove_clutter(eventopponent1);
		let eventopponent2 = remove_clutter(eventopponent2);
		let eventtime = remove_clutter(eventtime.to_lowercase());



		let response = MessageBuilder::new()
			.push(" ")
        	.push("**")
        	.push(eventopponent1)
        	.push("** vs. **")
        	.push(eventopponent2)
        	.push("**\n\t ")
        	.push(eventtime)
        	.push(" ")
        	.push("\n\t at *")
    		.push(eventname)
    		.push("* - ")
    		.push(eventsub)
    		.push("\n\n")
    		.build();
    	answer.push_str(response.clone().as_ref());	

    	counter += 1;

				//games.push(Game::new(eventname, eventsub, eventopponent, eventtime));
		if counter == 3 {//TODO Change value
			break;
		}
	}
	
	if counter == 0 {
		answer.push_str(" No upcoming matches at the moment. \n\n");
	}
	
	


	let response = MessageBuilder::new()
       	.push("\n")
    	.push("via ")
    	.push(urlstr.clone()) //TODO oder url.clone() 
    	.build();
    answer.push_str(response.as_ref());

    //Ok(Team::new(teamname, teamlogo, response, urlstr))
    embed_small_message(channel_id, &answer);
    
});


























*/
