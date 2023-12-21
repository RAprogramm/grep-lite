fn main() {
    let search_term = "picture";
    let qoute = "\
Every face, every shop, bedroom window, puЬlic-house, and 
dark square is а picture feverishly turned--in search of what? 
It is the same with books. 
What do we seek through millions of pages?";

    for line in qoute.lines() {
        if line.contains(search_term) {
            println!("{}", line);
        }
    }
}
