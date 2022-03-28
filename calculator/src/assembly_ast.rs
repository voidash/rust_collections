#[derive(Debug,PartialEq, Eq)]
pub enum Statement<'a> {
    Opcode(Option<&'a str>,&'a str, Option<u16>),

}

