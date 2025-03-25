use crate::lex_reader::get_line_array;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;


pub fn gen_reg()->(String, HashMap<u8,String>){
    let lex = "./test.yal";
    let mut map: HashMap<u8,String> = HashMap::new();
    let acts = get_line_array(lex);
    let mut reg_array : Vec<String> = Vec::new();
    for ac in acts{
        reg_array.push(format!("(({})({{{}}}))",ac.0, ac.2));
        map.insert(ac.2, ac.1);
    }
    let result = reg_array.join("|");
    (String::from(result),map)
}


pub fn creat_file(cont: String)->std::io::Result<()>{
    let mut file = File::create("./src/bin/compiler.rs")?; // Creates or truncates
    let byte_slice: &[u8] = cont.as_bytes();
    file.write_all(byte_slice)?; // Writes to the file
    Ok(())
}