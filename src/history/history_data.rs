use std::collections::HashMap;

use csv::StringRecord;

#[derive(Debug)]
pub struct HistoryStruct {
	pub type_data: String,
	pub tag : String,
	pub text: String,
	pub life : i32,
	pub option : Vec<HistoryStruct> 
}


impl HistoryStruct {

	pub fn new ( row : StringRecord ) -> HistoryStruct {

	let data = HistoryStruct {
		type_data : row.get(0).unwrap().trim().to_string() ,
		tag : row.get(1).unwrap().trim().to_string(),
		text : row.get(2).unwrap().trim().to_string(),
		life :  row.get(3).unwrap().trim().parse().unwrap_or(0),
		option : vec![]
	};
	return data
}

	pub fn get_option_selected () -> usize {

		let mut select_option = String::new();
		std::io::stdin().read_line(&mut select_option).unwrap();
		let select_option = select_option.trim().parse().unwrap_or(99);
		return select_option;
	}

	pub fn play_game( data : HashMap<String , HistoryStruct> ) {
		let mut live = 100;		
		let mut last_save_tag = "INICIO";

		loop {
			println!("Game");
			println!("You have {} life", live);

			if let Some(data) = data.get(last_save_tag) {
				println!("{:?}",data);
				println!("{}",data.text);

				for (index, option) in data.option.iter().enumerate() {
					println!("[{}] - {}", index, option.text)
				}

				let option = Self::get_option_selected();

				if let Some(option_selected) = &data.option.get(option) {
					last_save_tag = &option_selected.tag;
				} else {
					println!(" Option is not correct ")
				}

				live += data.life;
			} else {
				break;
			}

			println!("live opt = {}",live);
			if live <= 0 {
				println!("--------- GAME OVER ----------");
				break;
			}

		}
	}
}
