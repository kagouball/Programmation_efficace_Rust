use std::collections::HashMap;
use std::collections::hash_map::RandomState;

static mut t9: HashMap<char, u8, RandomState> = HashMap::new();
const T9: &str = "222333444555666777888999";
//               abcdefghijklmnopqrstuvwxyz

unsafe fn lettre_chiffre(x:&char) -> Option<&u8> {
    t9.get(x)
}

fn main() {
    initT9();

}

fn initT9(){
    unsafe {
        t9.insert('a',2);
        t9.insert('b',2);
        t9.insert('c',2);
        t9.insert('d',3);
        t9.insert('e',3);
        t9.insert('f',3);
        t9.insert('g',4);
        t9.insert('h',4);
        t9.insert('i',4);
        t9.insert('j',5);
        t9.insert('k',5);
        t9.insert('l',5);
        t9.insert('m',6);
        t9.insert('n',6);
        t9.insert('o',6);
        t9.insert('p',7);
        t9.insert('q',7);
        t9.insert('r',7);
        t9.insert('s',7);
        t9.insert('t',8);
        t9.insert('u',8);
        t9.insert('v',8);
        t9.insert('w',9);
        t9.insert('x',9);
        t9.insert('y',9);
        t9.insert('z',9);
    }
}
