
struct Team {
	name: String,
	logo: String,
	description: String,
	url: String
}

impl Team {
	fn new(name: String, logo: String, description: String, url: String) -> Team {
		Team {
			name: name,
			logo: logo,
			description: description,
			url: url
		}
	}
}


/*

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

*/




command!(team(_ctx, message, args) {
	let mut id: i64 = 0;
	let channel_id = message.channel_id;
	
	match args.single::<i64>() {
	    Ok(n) => id = n,
	    Err(_) => {
	    	embed_error(channel_id, "You need some numeric id as input! E.g.: `team 15`")
	    },
	}

	match get_team(id) {
		Err(TeamError::IDNotFound) => {
			let errormsg = MessageBuilder::new()
		    	.push("There exists no Team with the ID: ")
		    	.push(id)
		    	.build();
			embed_error(channel_id, errormsg.as_ref());
			},
		Ok(team) => {
			let _ = channel_id.send_message(|m| m
		    	//.content("tset")
		    	.embed(|e| e 
		    		.title(&team.name)
		    		.description(&team.description)
		    		.url(&team.url)
		    		.thumbnail(&team.logo)
		    		//.timestamp(timestamp_to_string(UTC::now))
		            .footer(|f| {
		            	let ref user = message.author;
		            	let mut tmp = String::new();
		            	tmp.push_str("Requested by ");
		            	tmp.push_str(&user.name.clone());
		            	tmp.push_str("#");
		            	tmp.push_str(&user.discriminator.to_string());
		            	let mut f = f.text(tmp);
		                let url = user.avatar_url();
		                if let Some(url) = url {
		                	f = f.icon_url(&url);
		                }
		                f
				})));
		}
	}   
});

command!(liquid(_ctx, msg) {

	match get_team(19i64) {
		Err(TeamError::IDNotFound) => {
			let errormsg = MessageBuilder::new()
		    	.push("There exists no Team with the ID: ")
		    	.push(19)
		    	.push("\n Please write a message to Caemor#9555 to tell him that Team Liquid is updated.")
		    	.build();

			let _ = msg.channel_id.say(errormsg);
			},
		Ok(team) => {
			let _ = msg.channel_id.send_message(|m| m
		    	//.content("tset")
		    	.embed(|e| e 
		    		.title(&team.name)
		    		.description(&team.description)
		    		.url(&team.url)
		    		.thumbnail(&team.logo)
		    		//.timestamp(timestamp_to_string(UTC::now))
		            .footer(|f| {
		            	let ref user = msg.author;
		            	let mut tmp = String::new();
		            	tmp.push_str("Requested by ");
		            	tmp.push_str(&user.name.clone());
		            	tmp.push_str("#");
		            	tmp.push_str(&user.discriminator.to_string());
		            	let mut f = f.text(tmp);
		                let url = user.avatar_url();
		                if let Some(url) = url {
		                	f = f.icon_url(&url);
		                }
		                f
				})));
		}
	}   
});



fn remove_clutter(toshorten: String) -> String {
	let toshorten = toshorten.replace("\t", "");
	let toshorten = toshorten.replace("\n", "");
	let toshorten = toshorten.replace("\"", "");
	let toshorten = toshorten.trim();

	return toshorten.to_string();
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

pub fn teamname_previously(toshorten: String) -> String {
	let toshorten = toshorten.replace("previously", " previously ");
	let toshorten = toshorten.replace("(inactive)", " (inactive) ");
	return toshorten.to_string();
}



fn edit_link(link: String) -> String {
	let link = link.replace("//", "http://");
	return link.to_string();
}







#[derive(Debug)]
enum TeamError {
    IDNotFound,
}

type TeamResult = Result<Team, TeamError>;


/*
	team-header
		team-header-logo
		team-header-name
		team-header-country
		website?
		twitter?

	upcoming matches
		match-item

	team-rating-info	
*/
fn get_team(id: i64) -> TeamResult {

    
    let urlstr = MessageBuilder::new()
    	.push("https://www.over.gg/team/")
    	.push(id)
    	.build();



	let url = Url::parse(urlstr.clone().as_ref()).unwrap();
	let mut resp = reqwest::get(url).unwrap();
	//assert!(resp.status().is_success());

	// when 404 error comes
	if !resp.status().is_success() {
		return Err(TeamError::IDNotFound)
	}

	let mut content = String::new();
	let _ = resp.read_to_string(&mut content);
    
    let document = Document::from(content.as_ref());




   
    //let mut games : Vec<Game> = Vec::new();

	trace!("# Command: Team with ");
	trace!("ID: {}. Teamname found: ", id);
    let node = document.find(Class("team-header")).take(1).next().unwrap();

    let teamname = node.find(Class("team-header-name")).next().unwrap().text();
   	let teamcountry = node.find(Class("team-header-country")).next().unwrap().text();
   	let teamlogo = node.find(Class("team-header-logo").descendant(Name("img"))).next().unwrap();//.child().unwrap();

   	//let teamlogo = teamlogo.find(Attr())
   	//let teamlogo = teamlogo.find(Attr("img", "src")).next().unwrap();
   	let teamname = teamname_previously(remove_clutter(teamname));
   	let teamcountry = remove_clutter(teamcountry);
   	let teamlogo = teamlogo.attr("src").unwrap();
   	let teamlogo = edit_link(teamlogo.to_string());
   	trace!("{}", teamname.clone());
   	//println!("{}", remove_clutter(teamcountry));
   	//println!("{:?}", teamlogo.attr("src").unwrap());

   

   	let mut stringui = String::new();

   	if teamname.contains("inactive") {
   		stringui.push_str(" Team is __***INACTIVE***__ \n\n"); 
   	} else {
   	
	    
    
		let mut nodes = document.find(Class("wf-card")).take(6);
		nodes.next();
		nodes.next();
		nodes.next();
		nodes.next();
		nodes.next();



		if let Some(node) = nodes.next() {
			stringui.push_str(" __***Upcoming Matches:***__ \n\n"); 

			for game in node.find(Class("match-item")) {
				let eventname = game.find(Class("match-item-event-name")).next().unwrap().text();
				let eventsub = game.find(Class("match-item-event-sub")).next().unwrap().text();
				let eventopponent = game.find(Class("match-item-vs-team-name")).next().unwrap().text();
				let eventtime = game.find(Class("match-item-eta")).next().unwrap().text();

				let eventname = remove_clutter(eventname);
				let eventsub = remove_clutter(eventsub);
				let eventopponent = remove_clutter(eventopponent);
				let eventtime = remove_clutter(eventtime.to_lowercase());

				trace!("{:?}", remove_clutter(eventsub.clone()));

				let response = MessageBuilder::new()
					.push(" ")
		        	.push("**")
		        	.push(eventopponent)
		        	.push(" ")
		        	.push(eventtime)
		        	.push("** ")
		        	.push("\n\t at ")
		    		.push(eventname)
		    		.push(" - ")
		    		.push(eventsub)
		    		.push("\n\n")
		    		.build();
		    	stringui.push_str(response.clone().as_ref());	

				//games.push(Game::new(eventname, eventsub, eventopponent, eventtime));
			}
		} else {
			stringui.push_str(" No upcoming matches at the moment. \n\n")
		}
	}
	


	let response = format!(" from {}.\n\n{} via https://over.gg", teamcountry, stringui);
	Ok(Team::new(teamname, teamlogo, response, urlstr))


}