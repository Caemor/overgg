
#[derive(Debug)]
struct Player {
	name: String,
	real_name: String,
	flag: String,
	twitter: Option<String>,
	twitch: Option<String>,

	link: String,
	image: String

	teams: Option<Vec<Team>>,

    total_winnings: u32,
    events: Option<Vec<Events>>,

    recent_results: Vec<Matches>,
    upcoming_matches: Vec<Matches>,

}