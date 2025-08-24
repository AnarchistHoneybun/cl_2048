use crate::Node;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};

/// Function startup:
/// takes the initial node vector and populates a node in it
/// with a value of 2 to serve as the first game tile. Returns the modified vector.
pub fn startup(nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
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

/// Function shutdown:
/// called if the game is over, and closes the alternate screen created
/// for the game. Then it disabled the raw mode and returns Ok(()) if
/// the above processes are successful. Basically clearing up the terminal
/// state so the formatting is not weird for subsequent interactions.
pub fn shutdown() -> std::io::Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/// Function game_prog:
/// takes the 2d vector of nodes and a boolean value passed by the
/// movement functions as an argument, and returns a modified vector.
/// The boolean value is used to check if there was any change in the game's
/// state after pressing the movement keys.
pub fn game_prog(nodes: Vec<Vec<Node>>, state_changed: bool) -> Vec<Vec<Node>> {
    //make a copy of the nodes vector
    let mut new_nodes = nodes.clone();

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

    // Check if the state is unchanged and there are no empty nodes.
    // If that is the case, check for game over. If that returns
    // true, execute the shutdown function and exit the program.
    // If however the game is not over, return the as yet unmodified
    // nodes vector.

    if !state_changed && empty_nodes.is_empty() {
        //if the state has not changed and there are no empty nodes, the game is over

        if check_game_over(&new_nodes) {
            shutdown().expect("Could not shutdown terminal");
            println!("Game Over! No moves left");
            std::process::exit(0);
        } else {
            //if the game is not over, return the nodes vector
            return new_nodes;
        }
    }

    //If only the state hasn't changed and there are empty nodes,
    // return the nodes vector.

    if !state_changed {
        //if the state has not changed, return the nodes vector
        return new_nodes;
    }

    // If none of the sentinel checks above return true, then their
    // must be some change in state and empty nodes. In that case,
    // find a random node from the empty nodes vector and give it a
    // value of 2. Then return the vector.

    //make a random number between 0 and the length of the empty_nodes vector
    let random_index = rand::random::<usize>() % empty_nodes.len();

    //add a 2 in the node at the random index
    new_nodes[empty_nodes[random_index][0]][empty_nodes[random_index][1]].value = 2;
    new_nodes[empty_nodes[random_index][0]][empty_nodes[random_index][1]].empty = false;

    //return the new_nodes vector
    return new_nodes;
}

/// Function check_game_over:
/// called before progressing the game state, this function takes the
/// 2d vector of nodes as an argument and returns a boolean value.
/// Iterates over the vector and checks all adjacent nodes. If none of
/// them are equal to each other, it means there is no possibility of
/// merging them, and the game is over.
pub fn check_game_over(nodes: &Vec<Vec<Node>>) -> bool {
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
