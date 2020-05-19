use std::io::BufRead;
use std::io::Write;
use std::{cmp::Ordering, env, io};

fn main() {
    let input = std::io::stdin();
    let mut out = std::io::stdout();
    let mut input_lock = input.lock();
    guess_game(&mut input_lock, &mut out, env::args());
}

fn guess_game<TIn: BufRead, TOut: Write, TArgs: Iterator<Item = String>>(
    input: &mut TIn,
    output: &mut TOut,
    args: TArgs,
) {
    writeln!(output, "Guess the number!");

    let secret_number = get_secret_number(args);
    loop {
        writeln!(output, "Please input your guess.");
        let guess = match get_guess_number(input) {
            Some(n) => n,
            _ => continue,
        };

        writeln!(output, "You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                writeln!(output, "Too small!");
            }
            Ordering::Greater => {
                writeln!(output, "Too big!");
            }
            Ordering::Equal => {
                writeln!(output, "You win!");
                break;
            }
        }
    }
}

fn get_secret_number<T: Iterator<Item = String>>(args: T) -> u32 {
    let secret_number = args
        .skip(1)
        .take(1)
        .last()
        .expect("No secret number is specified");
    secret_number
        .trim()
        .parse()
        .ok()
        .expect("Secret number is not a number")
}

fn get_guess_number<TIn: BufRead>(input: &mut TIn) -> Option<u32> {
    let mut guess = String::new();
    input.read_line(&mut guess).expect("Failed to read line");
    guess.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let mut input = b"10\n20\n15";
        let mut out: Vec<u8> = Vec::new();
        let args = vec!["guess_who".to_owned(), "15".to_owned()];
        guess_game(&mut &input[..], &mut out, args.into_iter());
        let s = match std::str::from_utf8(&out) {
            Ok(v) => v,
            Err(e) => panic!(e),
        };
        assert!(s.ends_with("You win!\n"));
    }

    #[test]
    fn test_get_guess_number() {
        let mut input = b"10";
        assert_eq!(
            Some(10),
            get_guess_number(&mut &input[..]),
            "failed to convert \"10\" to u32"
        );

        let input = b"-10";
        assert_eq!(
            None,
            get_guess_number(&mut &input[..]),
            "incorect behaviour with negative"
        );

        let input = b"4294967295";
        assert_eq!(
            Some(u32::max_value()),
            get_guess_number(&mut &input[..]),
            "can't process max of u32"
        );

        let input = b"4294967296";
        assert_eq!(
            None,
            get_guess_number(&mut &input[..]),
            "should return None for values greter then u32::max_value()"
        );

        let input = b"aaaaa";
        assert_eq!(
            None,
            get_guess_number(&mut &input[..]),
            "should return None for not digit input"
        );

        let input = b"";
        assert_eq!(
            None,
            get_guess_number(&mut &input[..]),
            "should return None for empty io"
        );
    }

    #[test]
    fn test_get_secret_number() {
        let args = vec!["".to_owned(), "10".to_owned(), "bb".to_owned()];
        assert_eq!(
            10,
            get_secret_number(args.into_iter()),
            "failed to convert \"10\" to u32"
        );

        let args = vec!["".to_owned(), "4294967295".to_owned(), "bb".to_owned()];
        assert_eq!(
            u32::max_value(),
            get_secret_number(args.into_iter()),
            "failed to convert \"10\" to u32"
        );
    }

    #[test]
    #[should_panic]
    fn test_get_secret_number_panics_on_non_digits_input() {
        let args = vec!["".to_owned(), "aaa".to_owned(), "bb".to_owned()];
        let number = get_secret_number(args.into_iter());
    }

    #[test]
    #[should_panic]
    fn test_get_secret_number_panics_without_env_arg() {
        let args = vec!["guess_who!".to_owned()];
        let number = get_secret_number(args.into_iter());
    }

    #[test]
    #[should_panic]
    fn test_get_secret_number_panics_on_negative() {
        let args = vec!["guess_who!".to_owned(), "-10".to_owned()];
        let number = get_secret_number(args.into_iter());
    }

    #[test]
    #[should_panic]
    fn test_get_secret_number_panics_on_greater_of_u32_max_value() {
        let args = vec!["guess_who!".to_owned(), "4294967296".to_owned()];
        let number = get_secret_number(args.into_iter());
    }

    #[test]
    #[should_panic]
    fn test_get_secret_number_panics_on_empty_arg() {
        let args = vec!["guess_who!".to_owned(), "".to_owned()];
        let number = get_secret_number(args.into_iter());
    }
}
