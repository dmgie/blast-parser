# blast-parser
A Rust-based blast text-output parser, using the `nom` crate 


This is a relatively small scale, and minute quickly-made parser for the text-based output of BLAST (in this case BLASTN). It uses the `nom` crate to actually build the parser.

In general, it tries to obtain all the alignments that were given, or some other information for ones which do not have a significant alignment.
