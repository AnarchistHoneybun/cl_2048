#![allow(dead_code)]
#![allow(unused_assignments)]

mod colors;
mod movement;
mod state_handler;




// ANCHOR: imports
use std::fs;
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::{stdout, Result};
use std::rc::Rc;
// ANCHOR_END: imports


fn is_power_of_two(value: u16) -> bool {
    if value == 0 {
        return false;
    }
    return value & (value - 1) == 0;
}


//ANCHOR: structs
struct App {
    nodes: Vec<Vec<Node>>,
}
#[derive(Clone)]
pub struct Node {
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
    app.nodes = state_handler::startup(app.nodes);

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
                        .style(Style::default().fg(colors::color_setter(block_value)))
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
                    app.nodes = movement::merge_up(app.nodes);
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Down {
                    app.nodes = movement::merge_down(app.nodes);
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Left {
                    app.nodes = movement::merge_left(app.nodes);
                } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Right {
                    app.nodes = movement::merge_right(app.nodes);
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