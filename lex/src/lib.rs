 use regex::RegexSet;
 use std::{fs, process};

 // 词素类别
#[derive(Debug,Clone,PartialEq)]
 pub enum Class {
    Whitespace,
    // KeyWords
    Begin,
    End,
    Call,
    Do,
    While,
    If,
    Then,
    Odd,
    Procedure,
    Const,
    Var,
    Read,
    Write,
    // Identifies
    Identifies,
    // Operations
    Plus, // +
    Minus, // -
    Times, // *
    Slash, // /
    Equal, // =
    Neq,  // <>
    Leq, // <=
    Lss, // <
    Geq, // >=
    Gtr, // >
    Becomes, // :=
    Lparen, // (
    Rparen, // )
    Comma,  // ,
    Semicolon, // ;
    Period,  // .
    Number,
}

 #[macro_use]
 extern crate lazy_static;

 lazy_static! {
     // 符号表
     static ref GET_CLASS: Vec<Class> = vec![
         Class::Whitespace,
         // KeyWords
         Class::Begin,
         Class::End,
         Class::Call,
         Class::Do,
         Class::While,
         Class::If,
         Class::Then,
         Class::Odd,
         Class::Procedure,
         Class::Const,
         Class::Var,
         Class::Read,
         Class::Write,
         // Identifies
         Class::Identifies,
         // Operations
         Class::Plus, // +
         Class::Minus, // -
         Class::Times, // *
         Class::Slash, // /
         Class::Equal, // =
         Class::Neq,  // <>
         Class::Leq, // <=
         Class::Lss, // <
         Class::Geq, // >=
         Class::Gtr, // >
         Class::Becomes, // :=
         Class::Lparen, // (
         Class::Rparen, // )
         Class::Comma,  // ,
         Class::Semicolon, // ;
         Class::Period,  // .
         Class::Number,
     ];

     // 词法规范
     static ref SPECIFICATION : RegexSet = RegexSet::new(&[
         r"^\s$",
         r"^begin$",
         r"^end$",
         r"^call$",
         r"^do$",
         r"^while$",
         r"^if$",
         r"^then$",
         r"^odd$",
         r"^procedure$",
         r"^const$",
         r"^var$",
         r"^read$",
         r"^write$",
         r"^([a-z]|_)([a-z]|_|[0-9])*$",
         r"^\+$",
         r"^-$",
         r"^\*$",
         r"^/$",
         r"^=$",
         r"^<>$",
         r"^<=$",
         r"^<$",
         r"^>=$",
         r"^>$",
         r"^:=$",
         r"^\($",
         r"^\)$",
         r"^,$",
         r"^;$",
         r"^\.$",
         r"^[0-9][0-9]*$"
     ]).unwrap();
 }

// 词法单元
#[derive(Debug)]
pub struct Token{
    pub class : Class,
    pub lexeme : String,
}

// 读入参数
 pub struct Config{
     pub filename: String,
 }

 // 初始化参数
 impl Config{
     pub fn new(mut args : std::env::Args) -> Result<Config,&'static str>{
         if args.len() < 2{
             return Err("not enough arguments");
         }
         args.next();
         let filename = match args.next(){
             Some(s) => s,
             None => return Err("Didn't get a filename"),
         };
         Ok(
             Config{
                 filename,
             }
         )
     }
 }


 pub fn get_tokens(config :Config) ->  Vec<Token>{
     let code = fs::read_to_string(&config.filename).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments : {}",err);
        process::exit(1)
     });
        lexical_analysis(&code)
 }


 // 词法分析函数
 pub fn lexical_analysis(code:&str) -> Vec<Token>{

     // 存放词法单元的容器
     let mut res: Vec<Token> = Vec::new();
     let mut beg_pos = 0;
     let mut end_pos = 0;
     let mut i = 0;
     let mut last_time_matched = false;
     // 词法分析
     while i!=code.len()+1{
         if i!= code.len() && SPECIFICATION.is_match(&code[beg_pos..i+1].to_lowercase()) {
             end_pos = i;
             last_time_matched = true
         }else if last_time_matched  {
             let indexs : Vec<usize>= SPECIFICATION.matches(&code[beg_pos..end_pos+1].to_lowercase()).into_iter().collect();
             // 如果不是空白字符则加入
             if GET_CLASS[indexs[0]] != Class::Whitespace{
                 res.push(Token{
                     class : GET_CLASS[indexs[0]].clone(),
                     lexeme : code[beg_pos..end_pos+1].to_string(),
                 });
             }
             beg_pos = i;
             i = i-1;
             last_time_matched = false;
         }
         i=i+1;
     }
     // 如果有非法字符,产生恐慌
     if i == code.len()+1  && beg_pos+1 != i {
         eprintln!("Error: the code have illegal  characters!!!!");
         let mut pos = beg_pos;
         for c in code[beg_pos..].chars(){
             if  !c.is_ascii_whitespace(){
                 pos+=1;
             }else{
                 break;
             }
         }
         eprintln!("Illegal character is {} on line {}.",&code[beg_pos..pos],&code[0..beg_pos].chars().into_iter().filter(|x| *x=='\n').collect::<Vec<char>>().len()+1);
         process::exit(1);
     }
     res
 }