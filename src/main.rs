
use  csv::{ReaderBuilder, StringRecord ,Reader, StringRecordsIter, Error};
use std::{fs, collections::HashMap};

mod history;
use history::history_data::{HistoryStruct};


mod game_data;

const HITORY_FILE_PATH : &str = "./src/history/history.csv";


fn csv_to_string(path:&str ) -> String {

    let content = fs::read_to_string(path).unwrap();
    return content
}

fn read_data_history( content : String ) -> HashMap<String, HistoryStruct> {
    

    let bytes = content.as_bytes();
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(bytes);

    let mut data_history_directo :HashMap<String, HistoryStruct> = HashMap::new(); 
    let mut last_record: String = "".to_string();

    for result in rdr.records() {
        let result  = result.unwrap();

        let new_data = HistoryStruct::new(result);


        if  new_data.type_data == "SITUACION" {
            last_record = new_data.tag.clone();
            data_history_directo.insert( new_data.tag.clone(), new_data);
        } else if new_data.type_data == "OPCION" {
            if let Some(data) =  data_history_directo.get_mut(&last_record)  {
                (*data).option.push(new_data)
            }
        }

    }

    return data_history_directo

}

fn main() {
    let cont = csv_to_string(HITORY_FILE_PATH);

    let history_info = read_data_history(cont); 

    HistoryStruct::play_game(history_info)

}
