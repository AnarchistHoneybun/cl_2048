#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use std::fs::File;
use std::{fs, io};

// ANCHOR: imports
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::{stdout, Read, Result};
use std::num::ParseIntError;
use std::rc::Rc;
use std::u16::MAX;
use enigo::*;
// ANCHOR_END: imports

fn startup(nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
    //make a copy of the nodes vector
    let mut new_nodes = nodes.clone();

    //make a vector of empty nodes
    let mut empty_nodes: Vec<Node> = Vec::new();

    //run a loop that adds all the empty nodes to the empty_nodes vector
    for i in 0..4 {
        for j in 0..4 {
            if new_nodes[i][j].empty == true {
                empty_nodes.push(new_nodes[i][j].clone());
            }
        }
    }

    //make a random number between 0 and 4 as the index of the empty_nodes vector
    let x_cor = (rand::random::<usize>() % 2) + 1;
    let y_cor = (rand::random::<usize>() % 2) + 1;

    new_nodes[x_cor][y_cor].value = 2;
    new_nodes[x_cor][y_cor].empty = false;

    //return the new_nodes vector
    return new_nodes;
}
fn color_setter(value: u16) -> Color {
    // check value and generate rgb values based on the value
    if value == 0 {
        return Color::Black;
    }

    //write a match statement that returns a Color::red based on the value

    match value {
        2 => Color::Rgb {
            0: 238,
            1: 228,
            2: 218,
        },
        4 => Color::Rgb {
            0: 237,
            1: 224,
            2: 200,
        },
        8 => Color::Rgb {
            0: 242,
            1: 177,
            2: 121,
        },
        16 => Color::Rgb {
            0: 245,
            1: 149,
            2: 99,
        },
        32 => Color::Rgb {
            0: 246,
            1: 124,
            2: 96,
        },
        64 => Color::Rgb {
            0: 246,
            1: 94,
            2: 59,
        },
        128 => Color::Rgb {
            0: 237,
            1: 207,
            2: 114,
        },
        256 => Color::Rgb {
            0: 237,
            1: 204,
            2: 97,
        },
        512 => Color::Rgb {
            0: 237,
            1: 200,
            2: 80,
        },
        1024 => Color::Rgb {
            0: 237,
            1: 197,
            2: 63,
        },
        2048 => Color::Rgb {
            0: 237,
            1: 194,
            2: 46,
        },
        _ => Color::Rgb {
            0: 237,
            1: 194,
            2: 46,
        },
    }
}
fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn game_prog(nodes: Vec<Vec<Node>>, state_changed: bool) -> Vec<Vec<Node>> {
    //make a copy of the nodes vector
    let mut new_nodes = nodes.clone();
    let mut enigo = Enigo::new();

    

    //iterate through the new_nodes vector and check if any of the nodes are empty
    //if a node is empty, store it's coordinates to a 2d vector
    let mut empty_nodes: Vec<Vec<usize>> = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            if new_nodes[i][j].empty == true {
                let mut temp_vec: Vec<usize> = Vec::new();
                temp_vec.push(i);
                temp_vec.push(j);
                empty_nodes.push(temp_vec);
            }
        }
    }

    //check if the state has changed
    //dbg!(state_changed);

    if !state_changed && empty_nodes.is_empty() {
        //if the state has not changed and there are no empty nodes, the game is over

        if check_game_over(&new_nodes) {
            enigo.key_down(Key::Layout('q'));
        } else {
            //if the game is not over, return the nodes vector
            return new_nodes;
        }
    }

    if !state_changed {
        //if the state has not changed, return the nodes vector
        return new_nodes;
    }

    //make a random number between 0 and the length of the empty_nodes vector
    let random_index = rand::random::<usize>() % empty_nodes.len();

    //add a 2 in the node at the random index
    new_nodes[empty_nodes[random_index][0]][empty_nodes[random_index][1]].value = 2;
    new_nodes[empty_nodes[random_index][0]][empty_nodes[random_index][1]].empty = false;

    //return the new_nodes vector
    return new_nodes;
}
fn check_game_over(nodes: &Vec<Vec<Node>>) -> bool {
    //iterate through the nodes vector and check if any two adjacent nodes have the same value
    //if they do, return false
    //if they don't, return true

    let mut game_over = true;

    for i in 0..3 {
        for j in 0..3 {
            if nodes[i][j].value == nodes[i][j + 1].value {
                game_over = false;
            }
            if nodes[i][j].value == nodes[i + 1][j].value {
                game_over = false;
            }
        }
    }

    return game_over;
}
fn is_power_of_two(value: u16) -> bool {
    if value == 0 {
        return false;
    }
    return value & (value - 1) == 0;
}

fn merge_up(nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
    // iterate across the columns in the nodes vector
    // for each column, iterate up the rows
    //if a node is not empty, check if the node above it is empty
    //if the node above it is empty, move the node up
    //if the node above it is not empty, check if the node above it is equal to the node
    //if the node above it is equal to the node, merge the nodes
    //if the node above it is not equal to the node, move both the nodes up one row

    let mut state_changed = false;

    //make a copy of the nodes vector
    let mut new_nodes = nodes.clone();

    //iterate across the columns in the nodes vector
    for i in 1..4 {
        for j in 0..4 {
            if new_nodes[i][j].value == 0 {
                continue;
            } else {
                let mut temp = i - 1;
                while temp > 0 && new_nodes[temp][j].value == 0 {
                    temp = temp - 1;
                }
                if new_nodes[temp][j].value == new_nodes[i][j].value {
                    new_nodes[temp][j].value = new_nodes[temp][j].value * 2;
                    new_nodes[temp][j].empty = false;
                    state_changed = true;
                    new_nodes[i][j].value = 0;
                    new_nodes[i][j].empty = true;
                } else {
                    if new_nodes[temp][j].value == 0 {
                        new_nodes[temp][j].value = new_nodes[i][j].value;
                        new_nodes[temp][j].empty = false;
                        state_changed = true;
                        new_nodes[i][j].value = 0;
                        new_nodes[i][j].empty = true;
                    } else {
                        if temp + 1 != i {
                            new_nodes[temp + 1][j].value = new_nodes[i][j].value;
                            new_nodes[temp + 1][j].empty = false;
                            state_changed = true;
                            new_nodes[i][j].value = 0;
                            new_nodes[i][j].empty = true;
                        }
                    }
                }
            }
        }
    }

    new_nodes = game_prog(new_nodes, state_changed);

    //return the new_nodes vector
    return new_nodes;
}

fn merge_down(nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
    //make a copy of the nodes vector
    let mut new_nodes = nodes.clone();
    let mut state_changed = false;

    //iterate across the columns in the nodes vector
    for i in (0..3).rev() {
        for j in 0..4 {
            if new_nodes[i][j].value == 0 {
                continue;
            } else {
                let mut temp = i + 1;
                while temp < 3 && new_nodes[temp][j].value == 0 {
                    temp += 1;
                }
                if new_nodes[temp][j].value == new_nodes[i][j].value {
                    new_nodes[temp][j].value = new_nodes[temp][j].value * 2;
                    new_nodes[temp][j].empty = false;
                    state_changed = true;
                    new_nodes[i][j].value = 0;
                    new_nodes[i][j].empty = true;
                } else {
                    if new_nodes[temp][j].value == 0 {
                        new_nodes[temp][j].value = new_nodes[i][j].value;
                        new_nodes[temp][j].empty = false;
                        state_changed = true;
                        new_nodes[i][j].value = 0;
                        new_nodes[i][j].empty = true;
                    } else {
                        if temp - 1 != i {
                            new_nodes[temp - 1][j].value = new_nodes[i][j].value;
                            new_nodes[temp - 1][j].empty = false;
                            state_changed = true;
                            new_nodes[i][j].value = 0;
                            new_nodes[i][j].empty = true;
                        }
                    }
                }
            }
        }
    }

    new_nodes = game_prog(new_nodes, state_changed);

    //return the new_nodes vector
    return new_nodes;
}
fn merge_left(nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
    let mut state_changed = false;

    //make a copy of the nodes vector
    let mut new_nodes = nodes.clone();

    //iterate across the columns in the nodes vector
    for j in 1..4 {
        for i in 0..4 {
            if new_nodes[i][j].value == 0 {
                continue;
            } else {
                let mut temp = j - 1;
                while temp > 0 && new_nodes[i][temp].value == 0 {
                    temp = temp - 1;
                }
                if new_nodes[i][temp].value == new_nodes[i][j].value {
                    new_nodes[i][temp].value = new_nodes[i][temp].value * 2;
                    new_nodes[i][temp].empty = false;
                    state_changed = true;
                    new_nodes[i][j].value = 0;
                    new_nodes[i][j].empty = true;
                } else {
                    if new_nodes[i][temp].value == 0 {
                        new_nodes[i][temp].value = new_nodes[i][j].value;
                        new_nodes[i][temp].empty = false;
                        state_changed = true;
                        new_nodes[i][j].value = 0;
                        new_nodes[i][j].empty = true;
                    } else {
                        if temp + 1 != j {
                            new_nodes[i][temp + 1].value = new_nodes[i][j].value;
                            new_nodes[i][temp + 1].empty = false;
                            state_changed = true;
                            new_nodes[i][j].value = 0;
                            new_nodes[i][j].empty = true;
                        }
                    }
                }
            }
        }
    }

    new_nodes = game_prog(new_nodes, state_changed);

    //return the new_nodes vector
    return new_nodes;
}
fn merge_right(nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
    let mut state_changed = false;

    //make a copy of the nodes vector
    let mut new_nodes = nodes.clone();

    //iterate across the columns in the nodes vector
    for j in (0..3).rev() {
        for i in 0..4 {
            if new_nodes[i][j].value == 0 {
                continue;
            } else {
                let mut temp = j + 1;
                while temp < 3 && new_nodes[i][temp].value == 0 {
                    temp += 1;
                }
                if new_nodes[i][temp].value == new_nodes[i][j].value {
                    new_nodes[i][temp].value = new_nodes[i][temp].value * 2;
                    new_nodes[i][temp].empty = false;
                    state_changed = true;
                    new_nodes[i][j].value = 0;
                    new_nodes[i][j].empty = true;
                } else {
                    if new_nodes[i][temp].value == 0 {
                        new_nodes[i][temp].value = new_nodes[i][j].value;
                        new_nodes[i][temp].empty = false;
                        state_changed = true;
                        new_nodes[i][j].value = 0;
                        new_nodes[i][j].empty = true;
                    } else {
                        if temp - 1 != j {
                            new_nodes[i][temp - 1].value = new_nodes[i][j].value;
                            new_nodes[i][temp - 1].empty = false;
                            state_changed = true;
                            new_nodes[i][j].value = 0;
                            new_nodes[i][j].empty = true;
                        }
                    }
                }
            }
        }
    }

    new_nodes = game_prog(new_nodes, state_changed);

    //return the new_nodes vector
    return new_nodes;
}

//ANCHOR: structs
struct App {
    nodes: Vec<Vec<Node>>,
}
#[derive(Clone)]
struct Node {
    value: u16,
    empty: bool,
}
//ANCHOR_END: structs

//ANCHOR: setup

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    //fill the 2d vector in the app struct with empty nodes
    let mut app = App {
        nodes: vec![
            vec![
                Node {
                    value: 0,
                    empty: true
                };
                4
            ];
            4
        ],
    };

    //run the startup function to put a 2 in one of the nodes
    app.nodes = startup(app.nodes);

    //access best score value
    let file_path = "src/best_score.txt";
    let mut contents = String::new();
    contents = fs::read_to_string(file_path).expect("Should have a txt file in source");

    let mut best_value = contents.parse::<u16>().unwrap();
    let mut max_value = 0;

    // ANCHOR_END: setup

    //ANCHOR: draw

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            let outer = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Max(area.height * 2 + 6),
                    Constraint::Max((area.height / 2) + 5),
                    Constraint::Min(0),
                ])
                .split(area);

            let outer_subdiv = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Max(3), Constraint::Min(0)])
                .split(outer[0]);

            let outer_lower = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ])
                .split(outer_subdiv[1]);

            let mut cols: [Rc<[Rect]>; 4] = [Rc::new([]), Rc::new([]), Rc::new([]), Rc::new([])];

            for i in 0..4 {
                cols[i] = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(vec![
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ])
                    .split(outer_lower[i]);
            }

            //render a header in outer_subdiv[0]
            let header = Paragraph::new("!2048!")
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Double),
                )
                //center the text vertically and horizontally
                .alignment(Alignment::Center);
            frame.render_widget(header, outer_subdiv[0]);

            for i in 0..4 {
                for j in 0..4 {
                    let block_value = app.nodes[i][j].value;
                    //check if block_value is 0
                    //if it is, make block text empty
                    //if it is not, make block text block_value
                    let mut block_text = String::new();
                    if block_value == 0 {
                        block_text = " ".parse().unwrap();
                    } else {
                        block_text = block_value.to_string();
                    }

                    let game_blocks = Paragraph::new(format!("{}", block_text))
                        .style(Style::default().fg(color_setter(block_value)))
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                                .padding(Padding::new(0, 1, cols[i][j].height / 3, 1)),
                        )
                        //center the text vertically and horizontally
                        .alignment(Alignment::Center);

                    frame.render_widget(game_blocks, cols[i][j]);
                }
            }

            
            for i in 0..4 {
                for j in 0..4 {
                    if app.nodes[i][j].value > max_value {
                        max_value = app.nodes[i][j].value;
                    }
                }
            }

            if max_value > best_value {
                best_value = max_value;
                fs::write(file_path, best_value.to_string()).expect("Unable to write game data")
            }

            let stats_detail = Paragraph::new(format!(
                "Score: {}\nBest: {:?}\n\n\n\n\n\nControls:\n← ↑ ↓ →\n\n\n'q' to quit",
                max_value, best_value
            ))
            .style(Style::default().fg(Color::Cyan))
            .block(
                Block::default()
                    .title("Stats")
                    //center the title
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double)
                    .padding(Padding::new(1, 1, outer[1].height / 3, 1)),
            )
            //center the text vertically and horizontally
            .alignment(Alignment::Left);
            frame.render_widget(stats_detail, outer[1]);
        })?;
        //ANCHOR_END: draw

        // ANCHOR: handle-events
        if event::poll(std::time::Duration::from_millis(20))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Up {
                    app.nodes = merge_up(app.nodes);
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Down {
                    app.nodes = merge_down(app.nodes);
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Left {
                    app.nodes = merge_left(app.nodes);
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Right {
                    app.nodes = merge_right(app.nodes);
                }
            }
        }
        // ANCHOR_END: handle-events
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    println!("Game Over! You reached {}.", max_value);
    Ok(())
}
