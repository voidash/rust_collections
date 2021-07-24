#[derive(Debug)]
pub enum TelephoneType {
   Default,
   Landline,
   Handset
}

#[derive(Debug)]
pub struct Directory{
    pub name : String,
    pub number : usize,
    pub address : String,
    pub t_type: TelephoneType 
}

impl Directory{
    pub fn set_name(&mut self, new_name: &str){
        self.name = String::from(new_name);
    }

    pub fn set_number(&mut self, number: usize){
        self.number = number;
    }

    pub fn set_address(&mut self, address: &str){
        self.address = String::from(address);
    }

    pub fn set_t_type(&mut self){
        if self.number / 100000_0000 > 1{ 
            self.t_type = TelephoneType::Handset
        }else {
            self.t_type = TelephoneType::Landline
        }
    }
}
