use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use clap::Parser;
use rand::prelude::*;

/// Generate a scramble for Hyperspeedcube
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the definition file
    #[arg(short, long)]
    input: Box<Path>,

    /// Where to put the scramble, or else simply print to stdout
    #[arg(short, long)]
    output: Option<Box<Path>>,
}

#[derive(Debug, PartialEq, Eq)]
struct Twist(String);

#[derive(Debug, PartialEq, Eq)]
struct Generator(Vec<Twist>);

#[derive(Debug, PartialEq, Eq)]
struct Definition {
    n: u8,
    d: u8,
    depth: u32,
    prefix: Generator,
    postfix: Generator,
    generators: Vec<Generator>,
}
impl Definition {
    fn parse(input: &str) -> Result<Self, &'static str> {
        // let re = Regex::new(
        //     r"n:\s*(?<n>\d+)\s*(//.*)?\n\
        //          d:\s*(?<d>\d+)\s*(//.*)?\n\
        //          depth:\s*(?<depth>\d+)\s*(//.*)?\n\
        //          prefix:\s*(?<prefix>[^/])\s*(//.*)?\n\
        //          postfix:\s*(?<postfix>[^/])\s*(//.*)?\n\
        //          generators:\s*(?<generators>[^/])\s*(//.*)?\n\
        //          ",
        // )
        // .unwrap();
        // let caps = re.captures(s).expect("invalid grammar");
        // println!("{:?}", caps);
        // println!("{:?}", caps.name("n").expect("no n").as_str().parse::<u8>());

        // remove empty lines and comments
        let mut lines = input.lines().filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line[0..line.find("//").unwrap_or(line.len())].trim())
            }
        });

        let Some(Ok(n)) = lines
            .next()
            .expect("ran out of tokens for n")
            .strip_prefix("n:")
            .map(|s| s.trim_start().parse())
        else {
            return Err("failed to parse n");
        };
        let Some(Ok(d)) = lines
            .next()
            .expect("ran out of tokens for d")
            .strip_prefix("d:")
            .map(|s| s.trim_start().parse())
        else {
            return Err("failed to parse d");
        };
        let Some(Ok(depth)) = lines
            .next()
            .expect("ran out of tokens for depth")
            .strip_prefix("depth:")
            .map(|s| s.trim_start().parse())
        else {
            return Err("failed to parse depth");
        };
        let Some(prefix) = lines
            .next()
            .expect("ran out of tokens for prefix")
            .strip_prefix("prefix:")
            .map(|s| {
                Generator(
                    s.split_whitespace()
                        .map(|twist| Twist(twist.to_string()))
                        .collect(),
                )
            })
        else {
            return Err("failed to parse prefix");
        };
        let Some(postfix) = lines
            .next()
            .expect("ran out of tokens for postfix")
            .strip_prefix("postfix:")
            .map(|s| {
                Generator(
                    s.split_whitespace()
                        .map(|twist| Twist(twist.to_string()))
                        .collect(),
                )
            })
        else {
            return Err("failed to parse postfix");
        };
        if lines.next().expect("ran out of tokens for generators") != "generators:" {
            return Err("failed to parse generators");
        }
        let generators = lines
            .map(|line| {
                Generator(
                    line.split_whitespace()
                        .map(|twist| Twist(twist.to_string()))
                        .collect(),
                )
            })
            .collect();

        Ok(Definition {
            n,
            d,
            depth,
            prefix,
            postfix,
            generators,
        })
    }
}

// yoinked from hsc
fn wrap_words<S: AsRef<str>>(words: impl Iterator<Item = S>) -> String {
    const WORD_WRAP_WIDTH: usize = 70;
    let mut ret = String::new();
    let mut column = 0;
    for word in words {
        let word = word.as_ref();
        if column == 0 {
            column += word.len();
            ret += word;
        } else {
            column += word.len() + 1;
            if column <= WORD_WRAP_WIDTH {
                ret += " ";
            } else {
                column = word.len();
                ret += "\n";
            }
            ret += word;
        }
    }
    ret
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let def = Definition::parse(&fs::read_to_string(args.input)?)?;

    let mut writer: Box<dyn Write> = match args.output {
        Some(path) => Box::new(io::BufWriter::new(fs::File::create(path)?)),
        None => Box::new(io::stdout().lock()),
    };

    write!(
        writer,
        "# Hyperspeedcube puzzle log
---
version: 1
puzzle:
  Rubiks{d}D:
    layer_count: {n}
state: 1
twists: >
",
        d = def.d,
        n = def.n
    )?;

    let mut rng = rand::rng();
    for line in wrap_words(
        def.prefix
            .0
            .iter()
            .chain(
                (0..def.depth)
                    .filter_map(|_| def.generators.choose(&mut rng).map(|g| g.0.iter()))
                    .flatten()
                    .chain(def.postfix.0.iter()),
            )
            .map(|twist| &twist.0),
    )
    .lines()
    {
        writeln!(writer, "  {line}")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "n:3
d:  4
depth: 100
prefix: zy 
postfix:  
generators: 
IU//
 IU//a
IU IU // a 
";
        let def = Definition::parse(input).unwrap();
        assert_eq!(def.n, 3);
        assert_eq!(def.d, 4);
        assert_eq!(def.depth, 100);
        assert_eq!(def.prefix, Generator(vec![Twist("zy".to_string())]));
        assert_eq!(def.postfix, Generator(vec![]));
        assert_eq!(
            def.generators,
            vec![
                Generator(vec![Twist("IU".to_string())]),
                Generator(vec![Twist("IU".to_string())]),
                Generator(vec![Twist("IU".to_string()), Twist("IU".to_string())])
            ]
        );
    }
}
