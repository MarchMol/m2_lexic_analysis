use crate::lex_reader::get_line_array;
use std::fs::File;
use std::io::Write;


pub fn gen_reg()->String{
    let lex = "./test2.lex";
    let acts = get_line_array(lex);
    let mut reg_array : Vec<String> = Vec::new();
    for ac in acts{
        reg_array.push(format!("(({})({{{}}}))",ac.0, ac.1));
    }
    let result = reg_array.join("|");
    result
}

pub fn compile()->String{
    let lex = "./test.lex";
    let acts = get_line_array(lex);
    let mut rslt = String::new();
    for ac in acts{
        rslt+=&format!("(({})({{{}}}))",ac.0, ac.1);
    }
    rslt
}

// fn creat_file(cont: String)->std::io::Result<()>{
//     let mut file = File::create("example.rs")?; // Creates or truncates
//     let byte_slice: &[u8] = cont.as_bytes();
//     file.write_all(byte_slice)?; // Writes to the file
//     Ok(())
// }