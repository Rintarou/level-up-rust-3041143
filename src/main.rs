#[derive(Debug, PartialEq)]
enum Pulse {
    Short,
    Long,
}

/// Represents a single character
type Letter = Vec<Pulse>;

/// Represents a string of characters
type Message = Vec<Letter>;

trait MorseCode {
    fn to_morse_code(&self) -> Message;
}

impl MorseCode for String {
    fn to_morse_code(&self) -> Message {
        self.chars()
            .map(|char| match char {
                'a' | 'A' => vec![Pulse::Short, Pulse::Long],
                'b' | 'B' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Short],
                'c' | 'C' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Short],
                'd' | 'D' => vec![Pulse::Long, Pulse::Short, Pulse::Short],
                'e' | 'E' => vec![Pulse::Short],
                'f' | 'F' => vec![Pulse::Short, Pulse::Short, Pulse::Long, Pulse::Short],
                'g' | 'G' => vec![Pulse::Long, Pulse::Long, Pulse::Short],
                'h' | 'H' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Short],
                'i' | 'I' => vec![Pulse::Short, Pulse::Short],
                'j' | 'J' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Long],
                'k' | 'K' => vec![Pulse::Long, Pulse::Short, Pulse::Long],
                'l' | 'L' => vec![Pulse::Short, Pulse::Long, Pulse::Short, Pulse::Short],
                'm' | 'M' => vec![Pulse::Long, Pulse::Long],
                'n' | 'N' => vec![Pulse::Long, Pulse::Short],
                'o' | 'O' => vec![Pulse::Long, Pulse::Long, Pulse::Long],
                'p' | 'P' => vec![Pulse::Short, Pulse::Long, Pulse::Long, Pulse::Short],
                'q' | 'Q' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Long],
                'r' | 'R' => vec![Pulse::Short, Pulse::Long, Pulse::Short],
                's' | 'S' => vec![Pulse::Short, Pulse::Short, Pulse::Short],
                't' | 'T' => vec![Pulse::Long],
                'u' | 'U' => vec![Pulse::Short, Pulse::Short, Pulse::Long],
                'v' | 'V' => vec![Pulse::Short, Pulse::Short, Pulse::Short, Pulse::Long],
                'w' | 'W' => vec![Pulse::Short, Pulse::Long, Pulse::Long],
                'x' | 'X' => vec![Pulse::Long, Pulse::Short, Pulse::Short, Pulse::Long],
                'y' | 'Y' => vec![Pulse::Long, Pulse::Short, Pulse::Long, Pulse::Long],
                'z' | 'Z' => vec![Pulse::Long, Pulse::Long, Pulse::Short, Pulse::Short],

                '1' => vec![
                    Pulse::Short,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '2' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '3' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Long,
                    Pulse::Long,
                ],
                '4' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Long,
                ],
                '5' => vec![
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '6' => vec![
                    Pulse::Long,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '7' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Short,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '8' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Short,
                    Pulse::Short,
                ],
                '9' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Short,
                ],
                '0' => vec![
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                    Pulse::Long,
                ],

                _ => vec![],
            })
            .filter(|vec| !vec.is_empty())
            .collect()
    }
}

impl std::fmt::Display for Pulse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Short => write!(f, "."),
            Pulse::Long => write!(f, "_"),
        }
    }
}

fn print_morse_code(code: &Message) {
    for letter in code.iter() {
        for pulse in letter.iter() {
            print!("{}", pulse);
        }
        print!(" ");
    }
    println!();
}

fn main() {
    let greeting = "Hello, world".to_string().to_morse_code();

    print_morse_code(&greeting);
}

#[test]
fn hello_world() {
    use Pulse::*;

    let expected = vec![
        vec![Short, Short, Short, Short],
        vec![Short],
        vec![Short, Long, Short, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Long, Long],
        vec![Short, Long, Long],
        vec![Long, Long, Long],
        vec![Short, Long, Short],
        vec![Short, Long, Short, Short],
        vec![Long, Short, Short],
    ];

    let actual = "Hello, world".to_string().to_morse_code();
    assert_eq!(actual, expected);
}

#[test]
fn whole_alphabet() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz1234567890".to_string();

    alphabet.to_morse_code();
    alphabet.to_uppercase().to_morse_code();
}
