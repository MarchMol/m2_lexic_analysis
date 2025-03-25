fn act(toks: Vec<String>){
    let tk_list:Vec<String>= Vec::new();
    for t in toks{if t=="5" { tk_list.push(INT); }
if t=="3" { tk_list.push(FLOAT); }
if t=="4" { tk_list.push(SIGNED); }
if t=="6" { tk_list.push(WHILE); }
if t=="11" { tk_list.push(GT); }
if t=="8" { tk_list.push(L_BRACE); }
if t=="12" { tk_list.push(GTE); }
if t=="14" { tk_list.push(LTE); }
if t=="10" { tk_list.push(ASIGN); }
if t=="15" {  }
if t=="16" { tk_list.push(RETURN); }
if t=="7" { tk_list.push(ID); }
if t=="13" { tk_list.push(LT); }
if t=="9" { tk_list.push(R_BRACE); }
   }
     println!("{:?}",tk_list); 
}
fn main() {
let reg = r"(((-?)[0-9]+.[0-9]*)({3}))|((-[0-9]+)({4}))|(([0-9]+)({5}))|((while)({6}))|(([a-z]+)({7}))|((\{)({8}))|((\})({9}))|((=)({10}))|((>)({11}))|((>=)({12}))|((<)({13}))|((<=)({14}))|((( |\n|\t|\s)+)({15}))|((return)({16}))";
let (minimized_map, minimized_accept_states,minimized_start, token_list) = generate(reg);
    let toks = simulate(input.to_string(), minimized_map, minimized_accept_states, minimized_start, token_list);
    let mut reslut: Vec<String> = Vec::new();
    for t in toks{
        act(t);
    }
}