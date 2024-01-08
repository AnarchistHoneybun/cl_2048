use crate::{state_handler, Node};

/// The movement functions:
/// these take a reference to the nodes vector of the game
/// and return a modified vector with changes based on the direction
/// of movement selected.
///
/// Merging rules:
/// - Iterate over the column or row, whichever is required
/// - start from the last value found in the particular column or row
/// - for a certain node, find the destination node
/// - a destination node is the closest node in that direction which is
/// either empty or has the same value as the current node
/// - merge the nodes
/// - continue iteration over the remaining nodes

pub fn merge_up(nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {

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

    /// After movement is done, the game_prog function is called
    /// which takes the vector of nodes created above and returns
    /// a modified vector which will be returned to the actual runtime.

    new_nodes = state_handler::game_prog(new_nodes, state_changed);

    //return the new_nodes vector
    return new_nodes;
}

pub fn merge_down(nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
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

    new_nodes = state_handler::game_prog(new_nodes, state_changed);

    //return the new_nodes vector
    return new_nodes;
}
pub fn merge_left(nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
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

    new_nodes = state_handler::game_prog(new_nodes, state_changed);

    //return the new_nodes vector
    return new_nodes;
}
pub fn merge_right(nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
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

    new_nodes = state_handler::game_prog(new_nodes, state_changed);

    //return the new_nodes vector
    return new_nodes;
}
