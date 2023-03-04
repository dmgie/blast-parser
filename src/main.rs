// Use nom to parse a blast file
use nom::{
    character::complete::{char, digit1, multispace0, multispace1, newline},
    combinator::{map, map_res, opt},
    sequence::{pair, preceded,separated_pair},
    bytes::complete::{tag,take_until, take},
    number::complete::float,
    IResult,
};
use clap::{Parser};

#[derive(Debug, PartialEq)]
struct Alignment {
    name: String,
    seq_id: String,
    length: i32,
    range: Option<(i64, i64)>,
    score: Option<f64>,
    e_value: Option<f64>,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file: String,
}

impl Alignment {
    fn parse(input: &str) -> IResult<&str, Alignment> {
        // Example entry:
        // >DNA-directed RNA polymerase II subunit RPB1 [Eschrichtius robustus]
        // Sequence ID: MBV99095.1 Length: 1457
        // Range 1: 160 to 195
        // Score:78.6 bits(192), Expect:1e-10,
        // ---Or if there are no alignments---
        // >DNA-directed RNA polymerase II subunit RPB1 [Eschrichtius robustus]
        // Sequence ID: MBV99095.1 Length: 1457
        // ------

        // Parse the name, where it starts with a '>', ignore newlines
        let (input, _) = take_until(">")(input)?; // Skip until > i.e discard up until that point
        let (input, name) = take_until("Sequence ID:")(input)?; // Take all up until Sequence ID:
        // println!("name: {name}");


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

        // println!("range: {range:?}");

        // Parse the score
        // there might be multiple newlines until the score, so we need to use take_until
        let (input, score) = opt(preceded(
            pair(take_until("Score:"), tag("Score:")),
            map_res(take_until(" bits"), |s: &str| s.parse::<f64>()),
        ))(input)?;

        // println!("score: {score}");

        // Parse the e_value
        let (input, _) = opt(take_until("Expect:"))(input)?;
        let (input, e_value) = opt(preceded(
            pair(tag("Expect:"), multispace0),
            map_res(take_until(","), |s: &str| s.parse::<f64>()),
        ))(input)?;

        // println!("e_value: {e_value}");

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
    let input = "
>DNA-directed RNA polymerase II subunit RPB1 [Eschrichtius robustus]
Sequence ID: MBV99095.1 Length: 1457
Range 1: 160 to 195

Score:78.6 bits(192), Expect:1e-10,
Method:Compositional matrix adjust.,
Identities:36/36(100%), Positives:36/36(100%), Gaps:0/36(0%)

Query  1736  APASAMHGGGPPSGDSACPLRTIKRVQFGVLSPDEL  1843
             APASAMHGGGPPSGDSACPLRTIKRVQFGVLSPDEL
Sbjct  160   APASAMHGGGPPSGDSACPLRTIKRVQFGVLSPDEL  195




>hypothetical protein GH733_018666 [Mirounga leonina]
Sequence ID: KAF3813513.1 Length: 2107
Range 1: 6 to 41

Score:78.2 bits(191), Expect:2e-10,
Method:Compositional matrix adjust.,
Identities:36/36(100%), Positives:36/36(100%), Gaps:0/36(0%)

Query  1736  APASAMHGGGPPSGDSACPLRTIKRVQFGVLSPDEL  1843
             APASAMHGGGPPSGDSACPLRTIKRVQFGVLSPDEL
Sbjct  6     APASAMHGGGPPSGDSACPLRTIKRVQFGVLSPDEL  41


>DNA-directed RNA polymerase II subunit RPB1 [Macaca mulatta]
Sequence ID: EHH24473.1 Length: 1629
>DNA-directed RNA polymerase II subunit RPB1 [Macaca fascicularis]
Sequence ID: EHH57677.1 Length: 1655
Range 1: 1 to 31

Score:70.5 bits(171), Expect:5e-08,
Method:Composition-based stats.,
Identities:31/31(100%), Positives:31/31(100%), Gaps:0/31(0%)

Query  1751  MHGGGPPSGDSACPLRTIKRVQFGVLSPDEL  1843
             MHGGGPPSGDSACPLRTIKRVQFGVLSPDEL
Sbjct  1     MHGGGPPSGDSACPLRTIKRVQFGVLSPDEL  31

";

    // run the parser multiple times until there is no more input
    let args = Args::parse();
    let input = std::fs::read_to_string(args.file).unwrap();
    let mut result = Alignment::parse(&*input);
    let mut alignments = Vec::new();
    while result.is_ok() {
        let (input, alignment) = result.unwrap();
        alignments.push(alignment);
        result = Alignment::parse(input);
    }
    for alignment in alignments {
        println!("{alignment:?}");
    }
}
