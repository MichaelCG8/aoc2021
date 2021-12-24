use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::iter;
use std::ops::RangeFrom;
use std::str::Chars;
use std::time;


fn main() {
    let start_total = time::Instant::now();
    let data = include_str!("../../inputs/18");
    let start_part1 = time::Instant::now();
    println!("Part 1: {} in {:?}", part1(data), start_part1.elapsed());
    // let start_part2 = time::Instant::now();
    // println!("Part 2: {} in {:?}", part2(data), start_part2.elapsed());

    println!("Total: {:?}", start_total.elapsed())
}


struct Node {
    id: Option<usize>,
    parent: Option<usize>,
    value: Option<isize>,
    children: Vec<usize>,
}

impl Node {
    fn new() -> Self {
        Self { id: None, parent: None, value: None, children: Vec::new() }
    }

    fn add_children_from_chars(&mut self, c_iter: &mut Chars, tree: &mut Tree, id_generator: &mut RangeFrom<usize>) {
        loop {
            match c_iter.next() {
                None => break,
                Some(c) => match c {
                    '[' => {
                        let mut nesting_level = 1;
                        let sub_chars: String = c_iter
                            .take_while(|&c| {
                                match c {
                                    '[' => nesting_level += 1,
                                    ']' => nesting_level -= 1,
                                    _ => {},
                                };
                                nesting_level != 0
                            })
                            .collect();
                        let mut node = Node::new();
                        let id = id_generator.next().unwrap();
                        node.id = Some(id);
                        node.parent = self.id;
                        node.add_children_from_chars(&mut sub_chars.chars(), tree, id_generator);
                        tree.insert(id, node);
                        self.children.push(id);
                    },
                    ',' => {},
                    _ => {
                        let mut node = Node::new();
                        let id = id_generator.next().unwrap();
                        node.id = Some(id);
                        node.parent = self.id;
                        node.value = Some(c.to_string().parse().unwrap());
                        tree.insert(id, node);
                        self.children.push(id);
                    }
                }
            }
        }
    }

    fn iter_descendants<'a>(&'a self, tree: &'a Tree) -> Box<dyn Iterator<Item=&Node> + 'a> {
        let mut iterator: Box<dyn Iterator<Item=&Node>> = Box::new(iter::empty());
        for &child_id in self.children.iter() {
            let child = tree.get(child_id).unwrap();
            iterator = Box::new(iterator.chain(iter::once(child)).chain(child.iter_descendants(tree)));
        }
        iterator
    }

    fn depth(&self, tree: &Tree) -> usize {
        match self.parent {
            None => 1,
            Some(id) => tree.get(id).unwrap().depth(tree) + 1,
        }
    }

    fn add_to_value(&mut self, value: isize) {
        *self.value.as_mut().unwrap() += value;
    }

    fn magnitude(&self, tree: &Tree) -> isize {
        match self.value {
            Some(val) => val,
            None => (
                3 * tree.get(self.children[0]).unwrap().magnitude(tree)
                + 2 * tree.get(self.children[1]).unwrap().magnitude(tree)
            )
        }
    }


    fn to_string(&self, tree: &Tree) -> String {
        match self.value {
            Some(val) => format!("{}", val),
            None => format!(
                "[{},{}]",
                tree.get(self.children[0]).unwrap().to_string(tree),
                tree.get(self.children[1]).unwrap().to_string(tree),
            ),
        }
    }
}


struct Tree {
    arena: HashMap<usize, Node>,
    root_id: usize,
    s: String,
}


impl Tree {
    fn new(root_id: usize) -> Self {
        Tree{ arena: HashMap::new(), root_id, s: "".to_string() }
    }

    fn insert(&mut self, key: usize, node: Node) {
        self.arena.insert(key, node);
    }

    fn get_mut(&mut self, key: usize) -> Option<&mut Node> {
        self.arena.get_mut(&key)
    }

    fn get(&self, key: usize) -> Option<&Node> {
        self.arena.get(&key)
    }

    fn iter_nodes(&self) -> Box<dyn Iterator<Item=&Node> + '_> {
        let root = self.get(self.root_id).unwrap();
        Box::new(iter::once(root).chain(root.iter_descendants(self)))
    }
}

impl Debug for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get(self.root_id).unwrap().to_string(self))
    }
}


fn add(mut a: Tree, mut b: Tree, id_generator: &mut RangeFrom<usize>) -> Tree {
    let root_id = id_generator.next().unwrap();
    let mut tree: Tree = Tree::new(root_id);
    let mut root = Node {
        id: Some(root_id),
        parent: None,
        value: None,
        children: Vec::new(),
    };
    root.children.push(a.root_id);
    root.children.push(b.root_id);

    a.get_mut(a.root_id).unwrap().parent = Some(root_id);
    b.get_mut(b.root_id).unwrap().parent = Some(root_id);

    // TODO: Use tree.arena = a.arena; tree.arena.extend(b.arena);
    tree.arena = a.arena.into_iter().chain(b.arena).collect();
    tree.arena.insert(root_id, root);
    tree.s = tree.to_string();

    println!("{}", tree.get(root_id).unwrap().to_string(&tree));
    reduce(&mut tree, id_generator);
    tree
}

enum Operation {
    Explode{ this: usize, left: Option<usize>, right: Option<usize> },
    Split{ this: usize, value: isize },
}

fn reduce(tree: &mut Tree, id_generator: &mut RangeFrom<usize>) {
    loop {
        println!("{}", tree.get(tree.root_id).unwrap().to_string(tree));
        let mut last_regular_id = None;
        let mut operation = None;
        {
            let mut node_iter = tree.iter_nodes();
            while let Some(node) = node_iter.next() {
                if node.value.is_some() {
                    last_regular_id = node.id;
                    if node.value.unwrap() >= 10 {
                        operation = Some(Operation::Split {
                            this: node.id.unwrap(),
                            value: node.value.unwrap(),
                        });
                        break;
                    }
                }
                if node.depth(tree) == 5 && node.value.is_none() {  // Pair nested inside 4 others.
                    // Next two are the contents of this pair. Find the next regular number AFTER that.
                    node_iter.next();
                    node_iter.next();
                    let next_regular_node = node_iter.find(|n| n.value.is_some());
                    operation = Some(Operation::Explode {
                        this: node.id.unwrap(),
                        left: last_regular_id,
                        right: match next_regular_node { None => None, Some(n) => n.id },
                    });
                    break;
                }
            }
        }
        match operation {
            None => break,
            Some(op) => match op {
                Operation::Explode{ this, left, right} => {
                    let mut remove_list = Vec::new();
                    {
                        let this_node = tree.get(this).unwrap();
                        remove_list.push(this_node.children[0]);
                        remove_list.push(this_node.children[1]);
                        let left_value = tree.get(this_node.children[0]).unwrap().value.unwrap();
                        let right_value = tree.get(this_node.children[1]).unwrap().value.unwrap();
                        if let Some(left_idx) = left {
                            tree.get_mut(left_idx).unwrap().add_to_value(left_value);
                        }
                        if let Some(right_idx) = right {
                            tree.get_mut(right_idx).unwrap().add_to_value(right_value);
                        }
                    }
                    tree.arena.remove(&remove_list[0]);
                    tree.arena.remove(&remove_list[1]);

                    let this_node_mut = tree.get_mut(this).unwrap();
                    this_node_mut.children.truncate(0);
                    this_node_mut.value = Some(0);
                },
                Operation::Split { this, value } => {
                    let mut node = Node::new();
                    let id = id_generator.next().unwrap();
                    node.id = Some(id);
                    node.parent = Some(this);
                    node.value = Some(value / 2);  // value/2 rounded down.
                    tree.insert(id, node);
                    let this_node_mut = tree.get_mut(this).unwrap();
                    this_node_mut.children.push(id);

                    let mut node = Node::new();
                    let id = id_generator.next().unwrap();
                    node.id = Some(id);
                    node.parent = Some(this);
                    node.value = Some(value - value / 2);  // value/2 rounded up.
                    tree.insert(id, node);
                    let this_node_mut = tree.get_mut(this).unwrap();
                    this_node_mut.children.push(id);

                    this_node_mut.value = None;
                },
            }
        }
        tree.s = tree.to_string();
    }
}


fn part1(data: &str) -> isize {
    let mut id_generator = 0..;
    let numbers: Vec<Tree> = data.
        lines()
        .map(|line| {
            let mut c_iter = line[1..line.len()-1].chars();  // strip outer []
            let root_id = id_generator.next().unwrap();
            let mut tree: Tree = Tree::new(root_id);
            let mut root = Node {
                id: Some(root_id),
                parent: None,
                value: None,
                children: Vec::new(),
            };
            root.add_children_from_chars(&mut c_iter, &mut tree, &mut id_generator);
            tree.insert(root_id, root);
            tree
        })
        .collect();


    let sum = numbers
        .into_iter()
        .reduce(|accum, item| add(accum, item, &mut id_generator))
        .unwrap();

    println!("{}", sum.get(sum.root_id).unwrap().to_string(&sum));
    sum.get(sum.root_id).unwrap().magnitude(&sum)
}

// 0: [[5,[2,8]],4],[5,[[9,9],0]]
// 1: [5,[2,8]],4
// 8: 5,[[9,9],0]
// 2: 5,[2,8]
// 7: 4
// 3: 5
// 4: 2, 8
//
//
// 0
// 1                       8
// 2        7              9           10
// 3   4   (4)            (5)          11           14
// (5) 5  6                            12  13       (0)
//    (2)(8)                           (9) (9)
// fn part2(data: &str) -> usize {
//
// }


#[cfg(test)]
mod tests {
    use super::*;
    static DATA : &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn part1_matches_sample() {

        assert_eq!(part1(DATA), 4140);
    }

    #[test]
    fn temp() {
        assert_eq!(sum_helper(vec!["[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"]), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string());
    }

    fn reduce_helper(s: &str) -> String {
        let mut id_generator = 0..;

        let mut c_iter = s[1..s.len()-1].chars();  // strip outer []
        let root_id = id_generator.next().unwrap();
        let mut tree: Tree = Tree::new(root_id);
        let mut root = Node {
            id: Some(root_id),
            parent: None,
            value: None,
            children: Vec::new(),
        };
        root.add_children_from_chars(&mut c_iter, &mut tree, &mut id_generator);
        tree.insert(root_id, root);

        tree.s = tree.to_string();
        reduce(&mut tree, &mut id_generator);
        tree.to_string()
    }

    fn sum_helper(strs: Vec<&str>) -> String {
        let mut id_generator = 0..;

        let numbers: Vec<Tree> = strs.iter().map(|s| {
            let mut c_iter = s[1..s.len()-1].chars();  // strip outer []
            let root_id = id_generator.next().unwrap();
            let mut tree: Tree = Tree::new(root_id);
            let mut root = Node {
                id: Some(root_id),
                parent: None,
                value: None,
                children: Vec::new(),
            };
            root.add_children_from_chars(&mut c_iter, &mut tree, &mut id_generator);
            tree.insert(root_id, root);
            tree.s = tree.to_string();
            tree
        }).collect();

        let sum = numbers
            .into_iter()
            .reduce(|accum, item| add(accum, item, &mut id_generator))
            .unwrap();
        sum.to_string()
    }

    #[test]
    fn temp2() {
        // assert_eq!(reduce_helper("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]".to_string());
        // assert_eq!(reduce_helper("[7,[6,[5,[4,[3,2]]]]]"), "[7,[6,[5,[7,0]]]]".to_string());
        // assert_eq!(reduce_helper("[[6,[5,[4,[3,2]]]],1]"), "[[6,[5,[7,0]]],3]".to_string());
        // assert_eq!(reduce_helper("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_string());
        // assert_eq!(reduce_helper("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_string());
        // assert_eq!(sum_helper(vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]"]), "[[[[1,1],[2,2]],[3,3]],[4,4]]".to_string());
        // assert_eq!(sum_helper(vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"]), "[[[[3,0],[5,3]],[4,4]],[5,5]]".to_string());
        // assert_eq!(sum_helper(vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"]), "[[[[5,0],[7,4]],[5,5]],[6,6]]".to_string());
        assert_eq!(sum_helper(vec!["[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"]), "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]".to_string());
        // assert_eq!(sum_helper(vec!["[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]", "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]", "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]", "[7,[5,[[3,8],[1,4]]]]", "[[2,[2,2]],[8,[8,1]]]", "[2,9]", "[1,[[[9,3],9],[[9,0],[0,7]]]]", "[[[5,[7,4]],7],1]", "[[[[4,2],2],6],[8,7]]"]), "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string());
        // assert_eq!(sum_helper(vec![]), "".to_string());
        // assert_eq!(sum_helper(vec![]), "".to_string());
        // assert_eq!(sum_helper(vec![]), "".to_string());
        // // assert_eq!(reduce_helper(""), "".to_string());
    }

    #[test]
    fn part2_matches_sample() {
        // assert_eq!(part2(DATA), );
    }
}
