
// Use nom to parse a blast file
use nom::{
    character::complete::{char, digit1, multispace0, multispace1, newline},
    combinator::{map, map_res, opt},
    sequence::{pair, preceded,separated_pair},
    bytes::complete::{tag,take_until, take},
    number::complete::float,
    error::Error,
    IResult,
};
use clap::{Parser};


#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file: String,
}

enum BlastError {
    ParseError,
}

#[derive(Debug, PartialEq)]
struct BlastFile {
    header: Header,
    alignments: Vec<Alignment>,
}

impl BlastFile {
    fn parse(input: &str) -> IResult<&str, BlastFile, Error<&str>> {

        // Parse header
        let (input, header) = Header::parse(input)?;

        // Parse alignments
        let mut alignments = Vec::new();
        let mut result = Alignment::parse(input);
        // run the parser multiple times until there is no more input
        while result.is_ok() {
            let (input, alignment) = result.unwrap();
            alignments.push(alignment);
            result = Alignment::parse(input);
        };

        Ok(("", BlastFile {
            header,
            alignments,
        }))
    }
}

#[derive(Debug, PartialEq)]
struct Header {
    program: String,
    length: i32
}


#[derive(Debug, PartialEq)]
struct Alignment {
    name: String,
    seq_id: String,
    length: i32,
    range: Option<(i64, i64)>,
    score: Option<f64>,
    e_value: Option<f64>,
}

impl Header {
    fn new(program: String, length: i32) -> Self {
        Header {
            program,
            length,
        }
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        // Program: BLASTX
        // Query: None ID: lcl|Query_60974(dna) Length: 2030
        let (input, _) = take_until("Program: ")(input)?;
        let (input, program) = preceded(tag("Program: "), take_until("\n"))(input)?;
        let (input, _) = newline(input)?;
        let (input, _) = tag("Query: None ID: lcl|Query_60974(dna) Length: ")(input)?;
        let (input, length) = map_res(digit1, |s: &str| s.parse::<i32>())(input)?;
        Ok((input, Header::new(program.to_string(), length)))
    }
}

impl Alignment {
    /// Example entry:
    /// >DNA-directed RNA polymerase II subunit RPB1 [Eschrichtius robustus]
    /// Sequence ID: MBV99095.1 Length: 1457
    /// Range 1: 160 to 195
    /// Score:78.6 bits(192), Expect:1e-10,
    /// ---Or if there are no alignments---
    /// >DNA-directed RNA polymerase II subunit RPB1 [Eschrichtius robustus]
    /// Sequence ID: MBV99095.1 Length: 1457
    /// ------
    fn parse(input: &str) -> IResult<&str, Alignment> {

        // Parse the name, where it starts with a '>', ignore newlines
        let (input, _) = take_until(">")(input)?; // Skip until > i.e discard up until that point
        let (input, name) = take_until("Sequence ID:")(input)?; // Take all up until Sequence ID:


        // Parse the sequence ID, which is a mix of letters and numbers i.e MBV99095.1
        let (input, _) = tag("Sequence ID: ")(input)?;
        let (input, seq_id) = take_until(" ")(input)?;

        // Parse the length
        let (input, length) = preceded(
            pair(tag(" Length:"), multispace0),
            map_res(digit1, |s: &str| s.parse::<i32>()),
        )(input)?;

        // FIXME: Early return if '>' is the next character, since no alignments were found
        // Maybe theres a better way to do this?
        if input.starts_with("\n>") {
            return Ok((input, Alignment {
                name: name.trim().to_string(),
                seq_id: seq_id.to_string(),
                length,
                range: None,
                score: None,
                e_value: None,
            }));
        }


        // Parse the range
        let (input, range) = opt(preceded(
            pair(tag("\nRange 1: "), multispace0),
            separated_pair(
                map_res(digit1, |s: &str| s.parse::<i64>()),
                pair(tag(" to "), multispace0),
                map_res(digit1, |s: &str| s.parse::<i64>()),
            ),
        ))(input)?;


        // Parse the score
        // there might be multiple newlines until the score, so we need to use take_until
        let (input, score) = opt(preceded(
            pair(take_until("Score:"), tag("Score:")),
            map_res(take_until(" bits"), |s: &str| s.parse::<f64>()),
        ))(input)?;


        // Parse the e_value
        let (input, _) = opt(take_until("Expect:"))(input)?;
        let (input, e_value) = opt(preceded(
            pair(tag("Expect:"), multispace0),
            map_res(take_until(","), |s: &str| s.parse::<f64>()),
        ))(input)?;


        return Ok((input, Alignment {
            name: name.trim().to_string(),
            seq_id: seq_id.to_string(),
            length,
            range,
            score,
            e_value,
        }))

    }
}

fn main() {
    // We load the entire file into memory, since it's usually not gonna be that large
    let args = Args::parse();
    let input = std::fs::read_to_string(args.file).unwrap();
    let result = BlastFile::parse(&input).unwrap().1;
    // println!("{:#?}", result.alignments);
}
