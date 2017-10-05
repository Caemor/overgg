
struct Event {
	name: String,
	url: String,

	image: String,

	location: String,
	price_pool: u32,

	start: String,
	end: String,


	description: String,
	url: String
}

impl Event {
	fn new(name: String, description: String, url: String) -> Event {
		Event {
			name: name,
			description: description,
			url: url
		}
	}
}



#[derive(Debug)]
enum EventError {
    IDNotFound,
}

type EventResult = Result<Event, EventError>;


	let id: i64;
	let channel_id = message.channel_id;
	
	match args.single::<i64>() {
	    Ok(n) => id = n,
	    Err(e) => {	    	
	    	embed_error(channel_id, "You need some numeric id as input! E.g.: `event 15`");
	    	return Err(CommandError::from(format!("Event: {:?}", e)));
	    },
	}

  	match get_event(id) {
		Err(EventError::IDNotFound) => {
			let errormsg = MessageBuilder::new()
		    	.push("There exists no Event with the ID: ")
		    	.push(id)
		    	.build();
			embed_error(channel_id, errormsg.as_ref());
			},
		Ok(event) => {			
			check_msg(channel_id.send_message(|m| m
		    	.embed(|e| e 
		    		.title(&event.name)
		    		.description(&event.description)
		    		.url(&event.url)
		    		//.thumbnail(team.logo.as_ref())
		    		//.timestamp(timestamp_to_string(UTC::now))
		            .footer(|_| embed_footer(&message.author))
		            .color(embed_color())
				)));
		}
	} 




fn get_event(id: i64) -> EventResult {
    
    let urlstr = MessageBuilder::new()
    	.push("https://www.over.gg/event/")
    	.push(id)
    	.build();

	let url = Url::parse(urlstr.clone().as_ref()).unwrap();
	let mut resp = reqwest::get(url).unwrap();

	// when 404 error comes
	if !resp.status().is_success() {
		return Err(EventError::IDNotFound)
	}

	let mut content = String::new();
	let _ = resp.read_to_string(&mut content);
    let document = Document::from(content.as_ref());

    let node = document.find(Class("event-desc")).take(1).next().unwrap();
    let mut text = remove_clutter(node.text().clone());

    let eventname = node.find(Class("event-title")).next().unwrap().text();
    
    let mut location = String::new();
    let mut prizepool = String::new();
    let mut start = String::new();
    let mut end = String::new();

    let mut answer = String::new();

    if text.contains("end: "){
    	let clone = text.clone();
    	let v: Vec<&str> = clone.split("end: ").collect();
    	text = v[0].to_string();
    	end = v[1].to_string();
    	let response = MessageBuilder::new()
						.push("**End:** ")
			        	.push(end)
			        	.push("\n\n")
			    		.build();
		end = response;

    }
    if text.contains("start: "){
    	let clone = text.clone();
    	let v: Vec<&str> = clone.split("start: ").collect();
    	text = v[0].to_string();
    	start = v[1].to_string();

    	let response = MessageBuilder::new()
						.push("**Start:** ")
			        	.push(start)
			        	.push("\n")
			    		.build();
		start = response;
    }
    if text.contains("prize pool: "){
    	let clone = text.clone();
    	let v: Vec<&str> = clone.split("prize pool: ").collect();
    	text = v[0].to_string();
    	prizepool = v[1].to_string();

    	let response = MessageBuilder::new()
						.push("**Prize Pool:** ")
			        	.push(prizepool)
			        	.push("\n\n")
			    		.build();
		prizepool = response;
    }
    if text.contains("location: "){
    	let clone = text.clone();
    	let v: Vec<&str> = clone.split("location: ").collect();
    	//text = v[0].to_string();
    	location = v[1].to_string();
    	let response = MessageBuilder::new()
						.push("**Location:** ")
			        	.push(location)
			        	.push("\n\n")
			    		.build();
		location = response;
    }

    let response = MessageBuilder::new()
					.push("\n")
		        	.push(location)
		        	.push(prizepool)
		        	.push(start)
		        	.push(end)
		        	.push("Over.gg event ID is: ")
		        	.push(id)
		    		.build();

	answer.push_str(response.as_ref());
	Ok(Event::new(eventname, answer, urlstr))
}
 