use serde_json::map::Entry;

use crate::reader::read_lines;

fn clean_reg(reg: &str)->String{
    let mut reg_chars = reg.chars().into_iter();
    let mut new_reg = String::new();
    while let Some(c) = reg_chars.next(){
        match c{
            '\"'=>{
            }
            _=>{
                new_reg.push(c);
            }
        }
    }
    new_reg
}

fn split_line(line: &str)->(String, String){
    let mut splitted:Vec<&str> = Vec::new();
    let mut line_chars = line.chars().into_iter().peekable();
    let mut last = '#';
    let mut argument = String::new();
    let mut action = String::new();

    let mut in_string = false;
    let mut in_action = false;
    while let Some(c) = line_chars.peek(){
        if *c=='\"' && last!='\\'{
            in_string = !in_string;

        } else if *c=='{'&& last!='\\'{
            in_action=true;
        } else if *c=='}'&& last!='\\'{
            in_action=false;
        }
        else{
            if *c!=' '{
                if in_action{
                    action+=&c.to_string();
                } else{
                    argument+=&c.to_string();

                }
            } else if in_string{
                argument+=&c.to_string();
            }
        
        }
        last = *c;
        line_chars.next();
    }
    // println!("Argument: {}",argument);
    // println!("Action: {}",action);
    splitted = line.split_whitespace().collect();
    (argument, action)
}
fn get_tk_act(line: &str)->(String, String){
    let mut splitted= split_line(line);
    let act = splitted.1;
    let reg = clean_reg(&splitted.0);
    // if splitted.len()==4{ // ideal = [reg, {, action ,} ]
    //     splitted.retain(|word| *word!="{" && *word!="}");
    //     reg = clean_reg(splitted[0]);
    //     act = splitted[1].to_string();
    // } else if splitted.len()==3{ // Hopefully, empty action = [reg, { ,} ]
    //     if splitted[1]=="{" && splitted[2] =="}"{
    //         reg = clean_reg(splitted[0]);            
    //         splitted.retain(|word| *word!="{" && *word!="}");
    //     } 
    // } else{
    //     // TODO raise exception
    // }
    // (reg, act)
    (reg,act)
}
pub fn get_line_array(filename:&str)->Vec<(String, String)>{
    let mut definitions: Vec<String> = Vec::new();
    let mut actions: Vec<(String,String)> = Vec::new();
    let mut def_started = false;
    let mut act_started = false;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(content) = line {
                if content == "%{"{
                    if !def_started && !act_started{ // Start definition stage
                        def_started = true;
                    }     
                } else if content == "%}"{
                    if def_started && !act_started{ // End definition stage
                        def_started = false;
                    }
                } else if content.contains("%%"){
                    if !def_started && !act_started{ // Start definition stage
                        act_started = true;
                    }    
                } else if content.contains("%%"){
                    if !def_started && act_started{ // End definition stage
                        act_started = false;
                    }
                } else{
                    if def_started{
                        definitions.push(content);
                    } else if act_started{
                        let entry = get_tk_act(&content);
                        actions.push(entry);
                    }
                }
            }
        }
    }

    actions
}