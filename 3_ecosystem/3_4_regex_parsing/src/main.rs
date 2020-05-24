#[macro_use]
extern crate lazy_static;
extern crate combine;
use combine::parser::char::{char, digit, spaces};
use combine::*;
use std::iter::FromIterator;

use regex::Regex;

fn main() {}

fn parse_regex(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(
            "^((.)?[<>^])?(\\+|\\-)?(#[?xXbo0]|0)?([0-9]+)?(\\.(([0-9]+)\\$?|\\*))?([a-zA-Z?]+?)?"
        )
        .unwrap();
    }
    let res: Option<regex::Captures> = REGEX.captures(input);
    match res {
        Some(captures) => {
            println!("{:?}", captures);
            let sign = captures.get(3).map(|val| {
                if val.as_str() == "-" {
                    return Sign::Minus;
                } else {
                    return Sign::Plus;
                }
            });
            let width = captures
                .get(5)
                .map(|val| val.as_str().parse::<usize>().unwrap());
            let precision = captures.get(6).map(|_| {
                let value = captures.get(7).unwrap();
                if value.as_str() == "*" {
                    return Precision::Asterisk;
                } else if value.as_str().ends_with('$') {
                    return Precision::Argument(captures[8].parse::<usize>().unwrap());
                } else {
                    return Precision::Integer(captures[8].parse::<usize>().unwrap());
                }
            });

            return (sign, width, precision);
        }
        None => (None, None, None),
    }
}

fn parse_custom(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    let mut skip_fill = optional(attempt(
        one_of("<>^".chars()).map(|x: char| x.to_string()).or(any()
            .and(one_of("<>^".chars()))
            .map(|x: (char, char)| String::from_iter(vec![x.0, x.1]))),
    ));

    let mut sign = optional(one_of("+-".chars()).map(|x: char| match x {
        '+' => Sign::Plus,
        '-' => Sign::Minus,
        _ => panic!("how did you get here?"),
    }));
    let mut h = optional(one_of("#".chars()));
    let mut zero = optional(one_of("0".chars()));
    let mut width = optional(
        many1::<Vec<_>, _, _>(digit()).map(|x| String::from_iter(x).parse::<usize>().unwrap()),
    );
    let mut precision = optional(
        one_of(".".chars()).and(
            one_of("*".chars())
                .map(|x: char| Precision::Asterisk)
                .or(many1::<Vec<_>, _, _>(digit())
                    .and(optional(one_of("$".chars())))
                    .map(|(x, dollar)| {
                        let digits = String::from_iter(x).parse::<usize>().unwrap();
                        match dollar {
                            Some(_) => Precision::Argument(digits),
                            None => Precision::Integer(digits),
                        }
                    })),
        ),
    );

    let fill_result = skip_fill.parse(input).unwrap();
    let sign_result = sign.parse(fill_result.1).unwrap();
    let h_result = h.parse(sign_result.1).unwrap();
    let zero_result = zero.parse(h_result.1).unwrap();
    let width_result = width.parse(zero_result.1).unwrap();
    let precision_result = precision.parse(width_result.1).unwrap();

    let pr = match precision_result.0 {
        Some(v) => Some(v.1),
        None => None,
    };
    (sign_result.0, width_result.0, pr)
}
enum Fill {
    Left(u8),
    Right(u8),
    Center(u8),
}

#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let (sign, ..) = parse_regex(input);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse_regex(input);
            assert_eq!(width, expected);
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse_regex(input);
            assert_eq!(precision, expected);
        }
    }

    #[test]
    fn parses_sign_custom() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let (sign, ..) = parse_custom(input);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width_custom() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse_custom(input);
            assert_eq!(width, expected);
        }
    }

    #[test]
    fn parses_precision_custom() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse_custom(input);
            assert_eq!(precision, expected);
        }
    }
}
