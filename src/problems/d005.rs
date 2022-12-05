use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, tuple},
};

pub struct Day {
    p1: String,
    p2: String,
    /// input string
    input: String,
    /// Native representation of the crate stacks, shifted to zero-indexed.
    /// Stack 1 is self.stacks[0], etc
    /// self.stacks[X][0] is the bottom item of Stack X+1
    stacks: Vec<Vec<char>>,
    /// List (X, Y-1, Z-1) corresponding to each "move X from Y to Z" instruction
    /// created by p1 and used in p2 so that we don't have to parse input twice
    instructions: Vec<(usize, usize, usize)>,
}



impl crate::Problem for Day {
    const YEAR: u32 = crate::YEAR;
    const DAY: u32 = 5;

    fn new(input: String) -> Self {
        Self {
            p1: String::new(),
            p2: String::new(),
            input,
            stacks: Vec::new(),
            instructions: Vec::new(),
        }
    }

    fn do_p1(&mut self) {
        // construct the parser that will parse the list of crates to move
        // split input into lines, applying a parser to each line (in this case, split using \n as separator)
        let mut crate_parse = separated_list0::<&str, _, _, nom::error::Error<_>, _, _>(
            // specifying \n as the character to split upon
            complete::char('\n'),
            // this parser is applied to each line
            // split input into chunks separated by spaces, apply a parser to chunk
            separated_list1(
                // specifying space as the separator
                complete::char(' '),
                // "pick one of the following parsers, take only the output of the first one that matches"
                alt((
                    // match an opening [, then any character, then a closing ]
                    delimited(complete::char('['), anychar, complete::char(']')),
                    // match three spaces in a row, in this case it will return the matched middle space.
                    // essentially, abuse of the delimited() parser so that we always consume the correct
                    // amount of whitespace
                    delimited(
                        complete::char(' '),
                        complete::char(' '),
                        complete::char(' '),
                    ),
                )),
            ),
        );

        // match the phrase "move X from Y to Z", parsing X, Y and Z into u64 values.
        // This step doesn't actually do any parsing, it just constructs the parser that we
        // apply repeatedly to the input
        let mut instr_parse = tuple::<&str, _, nom::error::Error<_>, _>((
            tag("move "),
            complete::u64,
            tag(" from "),
            complete::u64,
            tag(" to "),
            complete::u64,
        ));

        // the crate-parser will continue until it doesn't recognise the input.
        // `remaining` is returned by the parser as if to say "this is the rest of the
        // string that I didn't recognise". This gives us an input with the start removed,
        // to pass to the next parser.
        let (remaining, mut p_crates) = crate_parse(&self.input).unwrap();

        // resize our stacks vector for the number of crates
        self.stacks.resize(p_crates[0].len(), Vec::new());

        // transpose the parsed crate output into the native stack representation
        for mut layer in p_crates.drain(..).rev() {
            for (idx, chr) in layer.drain(..).enumerate() {
                // parser uses whitespace to indicate lack of crate
                if !chr.is_whitespace() {
                    self.stacks[idx].push(chr)
                }
            }
        }
        // clone it so that we can use self.stacks for p2
        let mut stacks = self.stacks.clone();

        // do stacking for the remaining input
        for (_, (_, count, _, from, _, to)) in remaining
            .lines()
            .skip(3)
            .map_while(|s| Some(instr_parse(s).unwrap()))
        {
            self.instructions
                .push((count as usize, from as usize - 1, to as usize - 1));
            for _ in 0..count {
                let c = stacks[from as usize - 1].pop().unwrap();
                stacks[to as usize - 1].push(c)
            }
        }
        self.p1 = stacks.iter().map(|v| v.last().unwrap_or(&' ')).collect();
    }

    fn do_p2(&mut self) {
        for (count, from, to) in self.instructions.iter() {
            let at = self.stacks[*from].len() - count;
            let mut mv = self.stacks[*from].split_off(at);
            self.stacks[*to].append(&mut mv);
        }
        self.p2 = self
            .stacks
            .iter()
            .map(|v| v.last().unwrap_or(&' '))
            .collect();
    }

    fn p1_result(&self) -> String {
        format!("{}", self.p1)
    }

    fn p2_result(&self) -> String {
        format!("{}", self.p2)
    }
}
