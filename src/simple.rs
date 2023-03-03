// More of a simple parser without using any fancy packages
use std::fs::File;


struct Alignments(Vec<Alignment>);

struct Alignment {
    name: String,
    seq_id: String,
    length: i32,
    range: (i64, i64),
    score: i32,
    e_value: f64,
}


fn main() {
    let mut file = File::open("test.sam").unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut alignments = Alignments(Vec::new());
    let mut current_alignment = Alignment {
        name: String::new(),
        seq_id: String::new(),
        length: 0,
        range: (0, 0),
        score: 0,
        e_value: 0.0,
    };


}
