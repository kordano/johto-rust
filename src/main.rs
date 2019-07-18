fn accum(s:&str)->String {
    let mut cypher = "".to_owned();
    for (i, c) in s.chars().enumerate() {
        for n in 0..i+1 {
            if i == 0 {
                cypher = cypher + &c.to_string().to_uppercase();
            } else if n == 0 {
                cypher = cypher + "-" + &c.to_string().to_uppercase();
            } else {
                cypher = cypher + &c.to_string().to_lowercase();
            }
        }
    }
    return cypher;
}

fn main() {
    println!("{}", accum("Zgglmgaer"));
}
