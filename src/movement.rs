use crate::{game_prog, Node};

pub fn merge_up(nodes: Vec<Vec<Node>>) -> Vec<Vec<Node>> {
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

    new_nodes = game_prog(new_nodes, state_changed);

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

    new_nodes = game_prog(new_nodes, state_changed);

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

    new_nodes = game_prog(new_nodes, state_changed);

    //return the new_nodes vector
    return new_nodes;
}