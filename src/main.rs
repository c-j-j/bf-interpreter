use std::{env, io::{BufReader, Read}, process::exit};
use std::fs::File;
use std::str;

struct Lexer <'a> {
    reader: &'a mut dyn Read,
}

#[derive(Debug, Clone)]
enum Op {
    Left,
    Right,
    IfStart {end: Option<usize> },
    IfEnd { start: Option<usize> },
    Output,
    Inc,
    Dec,
}

impl <'a> Lexer <'a> {
    fn new(reader: &'a mut dyn Read) -> Self {
        Self { reader }
    }

    fn next(&mut self) -> Option<Op> {
        while let Some(Ok(next_byte)) = self.reader.bytes().next() {
            match next_byte {
                b'>' => {
                    return Some(Op::Right)
                },
                b'<' => {
                    return Some(Op::Left)
                }
                b'[' => {
                    return Some(Op::IfStart { end: None } )
                }
                b']' => {
                    return Some(Op::IfEnd { start: None })
                }
                b'.' => {
                    return Some(Op::Output)
                }
                b'+' => {
                    return Some(Op::Inc)
                }
                b'-' => {
                    return Some(Op::Dec)
                }
                _ => {
                }
            }
        }
        None
    }

    fn parse(&mut self) -> Result<Vec<Op>, &str> {
        let mut ops = Vec::new();
        let mut stack = Vec::new();

        while let Some(op) = self.next() {
            match op {
                Op::IfStart { end: _ } => {
                    stack.push(ops.len());
                    ops.push(op);
                },
                Op::IfEnd { start: _ } => {
                    let addr = stack.pop().expect("ERROR: Stack underflow");
                    let start_if_op = ops[addr].clone();

                    match start_if_op {
                        Op::IfStart { end: _end } => {
                            ops[addr]  = Op::IfStart { end: Some(ops.len()) };
                        },
                        _ => {
                            return Err("ERROR: Stack in corrupt state ");
                        }
                    }
                    let end_if_op = Op::IfEnd { start: Some(addr) };

                    ops.push(end_if_op);
                },
                _ => {
                    ops.push(op)
                }
            }
        }

        Ok(ops)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print!("USAGE:
  ./bf filepath
               ");
        exit(1)
    }

    let filepath = args[1].as_str();
    let file = File::open(filepath).expect("ERROR: Unable to open file");

    let mut reader = BufReader::new(file);

    let mut lexer = Lexer::new(&mut reader);
    let ops = lexer.parse().expect("ERROR: Unable to parse input");

    let mut ip = 0;
    let mut memory: Vec<u8> = vec![0; 100];
    let mut head = 0;

    while ip < ops.len() {
        let curr_op = &ops[ip];

        match curr_op {
            Op::Inc => {
                memory[head] += 1;
                ip += 1;
            }
            Op::Dec => {
                memory[head] -= 1;
                ip += 1;
            }
            Op::Right => {
                head = head + 1;
                ip += 1;
            }
            Op::Left => {
                head = head - 1;
                ip += 1;
            }
            Op::IfStart {end} => {
                let curr_memory = memory[head];
                if curr_memory == 0 {
                    ip = end.expect("If statement has no end");
                } else {
                    ip = ip + 1;
                }
            }
            Op::IfEnd {start} => {
                let curr_memory = memory[head];
                
                if curr_memory != 0 {
                    ip = start.expect("If statement has no start");
                } else {
                    ip = ip + 1;
                }
            }
            Op::Output => {
                print!("{}", str::from_utf8(&[memory[head]]).unwrap());
                ip += 1;
            }
        }
    }
}
