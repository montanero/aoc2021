use std::cmp::max;
use std::iter::Peekable;
use std::path::Path;
use std::str::Chars;

use crate::aoc::file;

pub(crate) fn solve() -> i32 {
    solve_file(&file::input("input18.txt"))
}

fn solve_file(f: &Path) -> i32 {
    let trees = read_file(f);
    let mut mx = 0;
    for (i1, t1) in trees.iter().enumerate() {
        for (i2, t2) in trees.iter().enumerate() {
            if i1 != i2 {
                let mut b = t1.clone();
                b.add(&t2);
                let mag = b.magnitude();
                mx = max(mx, mag);
            }
        }
    }

    mx
}

fn read_file(f: &Path) -> Vec<Tree> {
    let mut res: Vec<Tree> = Vec::new();
    let lines = file::read_lines(f).unwrap();
    for line in lines {
        let line = line.unwrap();
        let tree = Tree::parse(&line);
        res.push(tree);
    }
    res
}

#[derive(Clone, Debug)]
enum Operator {
    Val(i32),
    Add,
}

type NodeIndex = usize;

#[derive(Clone, Debug)]
struct Node {
    parent: Option<NodeIndex>,
    left: Option<NodeIndex>,
    rite: Option<NodeIndex>,
    op: Operator,
}

#[derive(Clone, Debug)]
struct Tree {
    node: Vec<Node>,
    root: Option<NodeIndex>,
}

impl Tree {
    fn new_val_node(&mut self, val: i32) -> NodeIndex {
        self.node.push(Node {
            parent: None,
            left: None,
            rite: None,
            op: Operator::Val(val),
        });
        self.node.len() - 1
    }

    fn new_add_node(&mut self, left: NodeIndex, rite: NodeIndex) -> NodeIndex {
        let new = Node {
            parent: None,
            left: Some(left),
            rite: Some(rite),
            op: Operator::Add,
        };
        self.node.push(new);
        let idx = self.node.len() - 1;
        self.node[left].parent = Some(idx);
        self.node[rite].parent = Some(idx);
        idx
    }

    fn parse(inp: &str) -> Tree {
        let mut tree = Tree {
            node: vec![],
            root: None,
        };
        let mut p = inp.chars().peekable();
        tree.root = Some(tree.parse_iter(&mut p));
        tree
    }

    fn parse_iter(&mut self, input: &mut Peekable<Chars>) -> NodeIndex {
        let c = input.peek().unwrap();
        if c.is_digit(10) {
            let mut val = 0;
            loop {
                let c = input.next().unwrap();
                let dig = c.to_digit(10).unwrap();
                val = val * 10 + dig as i32;
                let cnext = input.peek().unwrap();
                if !cnext.is_digit(10) {
                    break;
                }
            }
            self.new_val_node(val)
        } else {
            let c = input.next().unwrap();
            assert_eq!(c, '[');
            let left = self.parse_iter(input);
            let c = input.next().unwrap();
            assert_eq!(c, ',');
            let rite = self.parse_iter(input);
            let c = input.next().unwrap();
            assert_eq!(c, ']');
            let new = self.new_add_node(left, rite);
            self.node[left].parent = Some(new);
            self.node[rite].parent = Some(new);
            new
        }
    }

    fn reduce(&mut self) {
        loop {
            let mut action = false;
            let ex = self.find_explodable(self.root.unwrap());
            match ex {
                Some(node) => {
                    self.explode(node);
                    action = true;
                }
                _ => {}
            }
            if !action {
                let sp = self.find_splittable(self.root.unwrap());
                match sp {
                    Some(node) => {
                        self.split(node);
                        action = true;
                    }
                    _ => {}
                }
            }
            if !action {
                break;
            }
        }
    }

    fn number_left_of(&self, node: NodeIndex) -> Option<NodeIndex> {
        let n = &self.node[node];
        match n.op {
            Operator::Val(_) => Some(node),
            Operator::Add => {
                match n.parent {
                    None => None,
                    Some(parent_index) => {
                        let parent_node = &self.node[parent_index];
                        if parent_node.left.unwrap() == node {
                            // we are in the left subtree of parent
                            self.number_left_of(parent_index)
                        } else {
                            // we are in the right subtree of parent
                            self.rightmost_number(parent_node.left.unwrap())
                        }
                    }
                }
            }
        }
    }

    fn rightmost_number(&self, node_index: NodeIndex) -> Option<NodeIndex> {
        let node = &self.node[node_index];
        match node.op {
            Operator::Val(_) => Some(node_index),
            Operator::Add => self.rightmost_number(node.rite.unwrap()),
        }
    }

    fn number_right_of(&self, node: NodeIndex) -> Option<NodeIndex> {
        let n = &self.node[node];
        match n.op {
            Operator::Val(_) => Some(node),
            Operator::Add => {
                match n.parent {
                    None => None,
                    Some(parent_index) => {
                        let parent_node = &self.node[parent_index];
                        if parent_node.rite.unwrap() == node {
                            // we are in the rite subtree of parent
                            self.number_right_of(parent_index)
                        } else {
                            // we are in the left subtree of parent
                            self.leftmost_number(parent_node.rite.unwrap())
                        }
                    }
                }
            }
        }
    }

    fn leftmost_number(&self, node_index: NodeIndex) -> Option<NodeIndex> {
        let node = &self.node[node_index];
        match node.op {
            Operator::Val(_) => Some(node_index),
            Operator::Add => self.leftmost_number(node.left.unwrap()),
        }
    }

    fn find_explodable(&self, node_index: NodeIndex) -> Option<NodeIndex> {
        let node = &self.node[node_index];
        match node.op {
            Operator::Add => {
                if self.get_depth(node_index) == 4 {
                    Some(node_index)
                } else {
                    let l = self.find_explodable(node.left.unwrap());
                    if l.is_some() {
                        l
                    } else {
                        self.find_explodable(node.rite.unwrap())
                    }
                }
            }
            _ => None,
        }
    }
    fn get_depth(&self, node_index: NodeIndex) -> usize {
        let node = &self.node[node_index];
        match node.parent {
            None => 0,
            Some(parent_index) => 1 + self.get_depth(parent_index),
        }
    }
    fn explode(&mut self, exploding_node_index: NodeIndex) {
        let left_next_number = self.number_left_of(exploding_node_index);
        match left_next_number {
            Some(target_index) => {
                let node = &self.node[exploding_node_index];
                let left_val = self.get_val(node.left.unwrap());
                self.set_val(target_index, left_val + self.get_val(target_index));
            }
            _ => {}
        }
        let rite_next_number = self.number_right_of(exploding_node_index);
        match rite_next_number {
            Some(target_index) => {
                let exploding_node = &self.node[exploding_node_index];
                let rite_val = self.get_val(exploding_node.rite.unwrap());
                let oldval = self.get_val(target_index);
                self.set_val(target_index, rite_val + oldval);
            }
            _ => {}
        }
        let node = &mut self.node[exploding_node_index];
        node.left = None;
        node.rite = None;
        node.op = Operator::Val(0)
    }

    fn find_splittable(&self, node_index: NodeIndex) -> Option<NodeIndex> {
        let node = &self.node[node_index];
        match node.op {
            Operator::Val(x) => {
                if x >= 10 {
                    Some(node_index)
                } else {
                    None
                }
            }
            Operator::Add => {
                let l = self.find_splittable(node.left.unwrap());
                if l.is_some() {
                    l
                } else {
                    self.find_splittable(node.rite.unwrap())
                }
            }
        }
    }

    fn split(&mut self, node_index: NodeIndex) {
        let node = &self.node[node_index];
        match node.op {
            Operator::Val(v) => {
                let ln = self.new_val_node(((v as f32) / 2.0).floor() as i32);
                self.node[ln].parent = Some(node_index);
                let l = Some(ln);
                let rn = self.new_val_node(((v as f32) / 2.0).ceil() as i32);
                self.node[rn].parent = Some(node_index);
                let r = Some(rn);
                let node = &mut self.node[node_index];
                node.left = l;
                node.rite = r;
                node.op = Operator::Add
            }
            _ => {
                panic!("!")
            }
        }
    }

    fn get_val(&self, node_index: NodeIndex) -> i32 {
        let node = &self.node[node_index];
        match node.op {
            Operator::Val(x) => x,
            _ => panic!("?"),
        }
    }

    fn set_val(&mut self, node_index: NodeIndex, val: i32) {
        let node = &mut self.node[node_index];
        node.op = Operator::Val(val);
    }

    fn magnitude(&self) -> i32 {
        self.magnitude_sub(self.root.unwrap())
    }
    fn magnitude_sub(&self, idx: NodeIndex) -> i32 {
        let node = &self.node[idx];
        match node.op {
            Operator::Val(x) => x,
            Operator::Add => {
                3 * self.magnitude_sub(node.left.unwrap())
                    + 2 * self.magnitude_sub(node.rite.unwrap())
            }
        }
    }

    fn add(&mut self, o: &Tree) -> &Tree {
        let rite = self.add_sub(o, o.root.unwrap());
        self.root = Some(self.new_add_node(self.root.unwrap(), rite));
        self.reduce();
        self
    }

    fn add_sub(&mut self, o: &Tree, ni: NodeIndex) -> NodeIndex {
        let onode = &o.node[ni];
        match onode.op {
            Operator::Val(x) => self.new_val_node(x),
            Operator::Add => {
                let left = self.add_sub(o, onode.left.unwrap());
                let rite = self.add_sub(o, onode.rite.unwrap());
                let new = self.new_add_node(left, rite);
                self.node[left].parent = Some(new);
                new
            }
        }
    }

    fn to_string(&self) -> String {
        self.to_string_node(self.root.unwrap())
    }

    fn to_string_node(&self, ni: NodeIndex) -> String {
        let onode = &self.node[ni];
        match onode.op {
            Operator::Val(x) => format!("{}", x),
            Operator::Add => format!(
                "[{},{}]",
                self.to_string_node(onode.left.unwrap()),
                self.to_string_node(onode.rite.unwrap())
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn result() {
        let result = solve();
        println!("result : {}", result);
        assert_eq!(result, 3665);
    }
}
