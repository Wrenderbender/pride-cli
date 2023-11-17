fn main() {
    const CLEAR:&str  = "\x1b[0m";
    const PRIDE_RBG:[&str; 6] = 
    ["\x1b[38;2;254;0;0m", "\x1b[38;2;254;141;0m", "\x1b[38;2;255;238;0m", "\x1b[38;2;1;129;20m", "\x1b[38;2;1;76;255m",
    "\x1b[38;2;138;1;140m"];//This is incorrect rbg fix
    const block:&str = "█";
    for i in PRIDE_RBG{
        println!("{} {} {}",i,block,CLEAR);
    }
}
