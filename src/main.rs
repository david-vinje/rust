use std::{io, collections::HashMap};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    let numpad  = HashMap::from([
        ('a', "2"),
        ('b', "22"),
        ('c', "222"),
        ('d', "3"),
        ('e', "33"),
        ('f', "333"),
        ('g', "4"),
        ('h', "44"),
        ('i', "444"),
        ('j', "5"),
        ('k', "55"),
        ('l', "555"),
        ('m', "6"),
        ('n', "66"),
        ('o', "666"),
        ('p', "7"),
        ('q', "77"),
        ('r', "777"),
        ('s', "7777"),
        ('t', "8"),
        ('u', "88"),
        ('v', "888"),
        ('w', "9"),
        ('x', "99"),
        ('y', "999"),
        ('z', "9999"),
        (' ', "0"),
    ]);
    for i in 1..=n {
        let mut msg = String::new();
        io::stdin().read_line(&mut msg).unwrap();
        let msg = msg.trim_end_matches("\n").chars();
        let mut prev = "X";
        let mut res = String::from(format!("Case #{}: ", i));
        for curr in msg {
            let nums = numpad.get(&curr).unwrap();
            if prev.contains(nums) || nums.contains(prev) {
                res.push_str(" ");
            }
            res.push_str(nums);
            prev = nums;
        }
        println!("{}", res)
    }
}
