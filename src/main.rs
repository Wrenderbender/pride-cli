use std::collections::HashMap;

use clap::Parser;
use lazy_static::lazy_static;
use terminal_size::terminal_size;



static CLEAR:&str  = "\x1b[0m";
static PRIDE_RBG:[&str; 6] = [
    "\x1b[38;2;254;0;0m", "\x1b[38;2;254;141;0m", "\x1b[38;2;255;238;0m", "\x1b[38;2;1;129;20m", "\x1b[38;2;1;76;255m",
"\x1b[38;2;138;1;140m"
];
static TRANS_RBG:[&str; 5] = [
    "\x1b[38;2;91;207;250m", "\x1b[38;2;245;171;185m", "\x1b[38;2;255;255;255m", "\x1b[38;2;245;171;185m", "\x1b[38;2;91;207;250m"
];
static BLOCK:&str = "█";

lazy_static! {
    static ref HASHMAP: HashMap<String, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("pride".to_string(), PRIDE_RBG.to_vec());
        m.insert("trans".to_string(), TRANS_RBG.to_vec());
        m.insert("transgender".to_string(), TRANS_RBG.to_vec());
        m
    };
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    flag: String,

    #[arg(long, default_value_t = 0)]
    width: u32,

    #[arg(long, default_value_t = 0)]
    height: u32,
}

fn main() {
    bar_flag(HASHMAP.get(&Args::parse().flag).unwrap());
}

fn bar_flag(flag: &Vec<&str>){
    //Takes a flag which has only bars and parses args, outputs it. Default height/width fills entire terminal screen
    let args = Args::parse();
    let mut width = args.width;
    let mut height = args.height;

    if height<(flag.len() as u32) && height!=0{
        eprintln!("Flag height is less then set height!");
        return;
    }
    if args.width == 0{
       width = terminal_size().unwrap().0.0 as u32;
    } 
    if args.height == 0{
        height = terminal_size().unwrap().1.0 as u32/(flag.len() as u32);

    }
    let mut blocks = "".to_string();
    for _ in 0..width{
        blocks.push_str(BLOCK);
    }
    for i in flag{
        for _ in 0..height{
            println!("{}{}{}",i,blocks,CLEAR);
        }
    }
}
