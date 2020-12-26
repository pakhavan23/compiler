#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use crate::lexer::SyntaxKind;
use crate::lexer::SyntaxToken;

pub fn parse(tokens: Vec<SyntaxToken>) {
    let mut state: i32 = 0;
    let mut v_operators: Vec<String> = vec![];
    let mut v_numbers: Vec<String> = vec![];
    let mut v_nodes: Vec<Tree> = vec![];

    for token in tokens {
        match state {
            0 => match token.kind {
                SyntaxKind::WhitespaceToken => state = 0, // if our token is whitespace , we will stay in that state
                SyntaxKind::WordlyToken => state = 17,    // assigning value to a variable
                _ => println!("Error in postion {}", token.position),
            },
            17 => match token.kind {
                SyntaxKind::WhitespaceToken => state = 17,
                SyntaxKind::AssignToken => state = 18,
                _ => println!("Error in postion {}", token.position),
            },
            18 => match token.kind {
                SyntaxKind::WhitespaceToken => state = 18,
                SyntaxKind::NumberToken | SyntaxKind::WordlyToken => {
                    state = 19;
                    v_numbers.push(token.text);
                }
                SyntaxKind::StringToken => state = 19,
                SyntaxKind::ParenthesesOpenToken => {
                    v_operators.push(token.text);
                    state = 22;
                }
                _ => println!("Error in postion {}", token.position),
            },
            19 => match token.kind {
                SyntaxKind::WhitespaceToken => state = 19,
                SyntaxKind::AdditionToken
                | SyntaxKind::SubstractionToken
                | SyntaxKind::DivisionToken
                | SyntaxKind::MultiplicationToken => {
                    v_operators.push(token.text);
                    state = 20;
                }
                SyntaxKind::CaretToken => {
                    v_numbers = vec![];
                    state = 0
                } // assigning finished
                _ => println!(
                    "Error in postion {} , {} , {}",
                    token.position, token.text, state
                ),
            },
            20 => match token.kind {
                SyntaxKind::WhitespaceToken => state = 20,
                SyntaxKind::NumberToken | SyntaxKind::WordlyToken => {
                    state = 21;
                    v_numbers.push(token.text);
                }
                SyntaxKind::StringToken => state = 21,
                SyntaxKind::ParenthesesOpenToken => {
                    v_operators.push(token.text);
                    state = 22;
                }
                _ => println!("Error in postion {}", token.position),
            },
            21 => match token.kind {
                SyntaxKind::WhitespaceToken => state = 21,
                SyntaxKind::AdditionToken
                | SyntaxKind::SubstractionToken
                | SyntaxKind::DivisionToken
                | SyntaxKind::MultiplicationToken => {
                    precedence(&mut v_operators, &mut v_numbers, &mut v_nodes, token.text);
                    state = 20;
                }
                SyntaxKind::ParenthesesCloseToken => {
                    creat_tree(&mut v_operators, &mut v_numbers, &mut v_nodes, false)
                }
                SyntaxKind::CaretToken => {
                    if v_operators.len() != 0 {
                        creat_tree(&mut v_operators, &mut v_numbers, &mut v_nodes, false)
                    }
                    state = 0
                }
                _ => println!("Error in postion {} , {}", token.position, token.text),
            },
            22 => match token.kind {
                SyntaxKind::WhitespaceToken => state = 22,
                SyntaxKind::NumberToken | SyntaxKind::WordlyToken => {
                    v_numbers.push(token.text);
                    state = 23;
                }
                SyntaxKind::ParenthesesOpenToken => {
                    v_operators.push(token.text);
                    state = 22
                }
                _ => println!("Error in postion {}", token.position),
            },
            23 => match token.kind {
                SyntaxKind::WhitespaceToken => state = 23,
                SyntaxKind::AdditionToken
                | SyntaxKind::SubstractionToken
                | SyntaxKind::DivisionToken
                | SyntaxKind::MultiplicationToken => {
                    precedence(&mut v_operators, &mut v_numbers, &mut v_nodes, token.text);
                    state = 22;
                }
                SyntaxKind::ParenthesesCloseToken => {
                    creat_tree(&mut v_operators, &mut v_numbers, &mut v_nodes, false);
                    state = 21;
                }
                _ => println!("Error in postion {} , {}", token.position, token.text),
            },
            _ => println!("1"),
        }
    }
    println!("{:?}", v_nodes);
    println!("{}", v_nodes.len());
}

fn creat_tree(
    operators: &mut Vec<String>,
    numbers: &mut Vec<String>,
    nodes: &mut Vec<Tree>,
    flag: bool,
) {
    loop {
        match operators.pop() {
            Some(z) => {
                if z == "(" {
                    break;
                }
                let node = Node { value: z };
                match numbers.pop() {
                    Some(num2) => {
                        let node_right = Node { value: num2 };
                        let mut new_tree = Tree {
                            main: node,
                            left_child: Node {
                                value: "nothing".to_string(),
                            },
                            right_child: node_right,
                        };
                        match numbers.pop() {
                            Some(num1) => {
                                let node_left = Node { value: num1 };
                                new_tree.left_child = node_left;
                            }
                            None => match nodes.pop() {
                                Some(node_pop) => {
                                    if node_pop.main.value == "Zarb" {
                                        new_tree.left_child = Node {
                                            value: "Zarb".to_string(),
                                        };
                                    } else if node_pop.main.value == "Jam" {
                                        new_tree.left_child = Node {
                                            value: "Jam".to_string(),
                                        };
                                    } else if node_pop.main.value == "Kam" {
                                        new_tree.left_child = Node {
                                            value: "Kam".to_string(),
                                        };
                                    } else if node_pop.main.value == "Tagsim" {
                                        new_tree.left_child = Node {
                                            value: "Tagsim".to_string(),
                                        };
                                    }
                                    nodes.push(node_pop);
                                }
                                None => println!("Nothing"),
                            },
                        }
                        nodes.push(new_tree);
                    }
                    None => break,
                }
            }
            None => break,
        }
        if flag {
            break;
        }
    }
}

fn precedence(
    operators: &mut Vec<String>,
    numbers: &mut Vec<String>,
    nodes: &mut Vec<Tree>,
    text: String,
) {
    match operators.pop() {
        Some(z) => {
            if (z == "Zarb" || z == "Tagsim") && (text == "Jam" || text == "Kam") {
                operators.push(z);
                creat_tree(operators, numbers, nodes, true);
                operators.push(text);
            } else {
                operators.push(z);
                operators.push(text);
            }
        }
        None => operators.push(text),
    }
}

#[derive(Debug)]
struct Node {
    value: String,
}

#[derive(Debug)]
struct Tree {
    main: Node,
    right_child: Node,
    left_child: Node,
}
