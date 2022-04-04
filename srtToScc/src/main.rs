use regex::Regex;

#[derive(debug, Copy)]
struct Caption {
    pub time_start: String,
    pub time_stop: String,
    pub text : String,
}

impl Caption {
    fn new(time_start: String, time_stop: String, text: String)  -> Caption {
        Caption {
            time_start, 
            time_stop,
            text
        }
    }
    fn is_valid(&self) -> bool {
        if self.time_start > self.time_stop { return false;}
        true
    }
}


#[allow(unused_mut, unused_variables)]
pub fn parse(input: Vec<String>) {

    let mut result: Vec<Caption> = Vec::new();
    let number: Regex = Regex::new(r"^[0-9]+$").unwrap();
    let time = Regex::new(r"(?P<time_start>[0-9:,.]+) --> (?P<time_stop>[0-9:,.]+)").unwrap();

    let mut buffer_caption: Caption = Caption{
        time_start: String::new(), time_stop: String::new(), text:String::from("")
    };

    for line in &input {
        if !number.is_match(line) && time.is_match(line) {
            //start of buffer
             match number.captures(line) {
                 Some(captured) => {
                     buffer_caption.time_start = captured.name("time_start").map(|ts| ts.as_str().to_string()).unwrap();
                     buffer_caption.time_stop = captured.name("time_stop").map(|ts| ts.as_str().to_string()).unwrap();
                 }
                None => {} 
                }
        }
        if !number.is_match(line) && !time.is_match(line) {
            buffer_caption.text = format!("{}{}\n",buffer_caption.text, line);
        }
        
        if number.is_match(line) && !time.is_match(line) {
           result.push(buffer_caption);
        }
    }
}

pub fn odd_parity(data: u8) -> u8{
    let mut result = data;
    let mut bit_count = 0;
    while data > 0 {
        bit_count += if data & 0x01 == 0x01 {1} else {0};
        data = data >> 1;
    }
    result = if bit_count % 2 == 0 {result | 0x80} else {result};

    result
}


pub fn scc_writer(srt: Vec<Caption>) {
    let mut writer = vec!["Scenarist_SCC V1.0\n\n"];
    
}

fn main() {
    println!("Hello, world!");
}

