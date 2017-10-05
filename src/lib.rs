
extern crate select;
extern crate reqwest;
#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;


pub mod upcoming_matches;
pub mod social;
pub mod search;
pub mod news;
mod helpers;


/*
Search ✓
	- Player X
	- Team X
	- Event X

News ✓
Upcoming_Matches ✓
Over.gg/Social ✓

Livestreams X

Events X
	- Ongoing X
	- Completed X
	- Upcoming X
*/
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    	
    }
}

