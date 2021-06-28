// source: https://blog.rust-lang.org/2015/04/17/Enums-match-mutation-and-moves.html
fn main() {
    println!("Hello, world!");
}

enum BinaryTree {
    Leaf(i32),
    Node(Box<BinaryTree>, i32, Box<BinaryTree>),
}

fn tree_weight_v1(t: &BinaryTree) -> i32 {
    match *t {
        BinaryTree::Leaf(payload) => payload,
        BinaryTree::Node(ref left, payload, ref right) => {
            tree_weight_v1(left) + payload + tree_weight_v1(right)
        }
    }
}

fn sample_tree() -> BinaryTree {
    let l1 = Box::new(BinaryTree::Leaf(1));
    let l3 = Box::new(BinaryTree::Leaf(3));
    let n2 = Box::new(BinaryTree::Node(l1, 2, l3));
    let l5 = Box::new(BinaryTree::Leaf(5));

    BinaryTree::Node(n2, 4, l5)
}

#[test]
fn tree_demo_1() {
    let tree = sample_tree();
    assert_eq!(tree_weight_v1(&tree), (1 + 2 + 3) + 4 + 5);
}

// Consider writing a function which maps a non-negative integer to a string
// rendering it as an ordinal ("1st", "2nd", "3rd", ...).
fn num_to_ordinal(x: u32) -> String {
    let suffix;
    match (x % 10, x % 100) {
        (1, 1) | (1, 21..=91) => {
            suffix = "st";
        }
        (2, 2) | (2, 22..=92) => {
            suffix = "nd";
        }
        (3, 3) | (3, 23..=93) => {
            suffix = "rd";
        }
        _ => {
            suffix = "th";
        }
    }
    return format!("{}{}", x, suffix);
}

fn num_to_ordinal_expr(x: u32) -> String {
    format!(
        "{}{}",
        x,
        match (x % 10, x % 100) {
            (1, 1) | (1, 21..=91) => "st",
            (2, 2) | (2, 22..=92) => "nd",
            (3, 3) | (3, 23..=93) => "rd",
            _ => "th",
        }
    )
}

#[test]
fn test_num_to_ordinal() {
    assert_eq!(num_to_ordinal(0), "0th");
    assert_eq!(num_to_ordinal(1), "1st");
    assert_eq!(num_to_ordinal(12), "12th");
    assert_eq!(num_to_ordinal(22), "22nd");
    assert_eq!(num_to_ordinal(43), "43rd");
    assert_eq!(num_to_ordinal(67), "67th");
    assert_eq!(num_to_ordinal(1901), "1901st");
}

#[test]
fn test_num_to_ordinal_expr() {
    assert_eq!(num_to_ordinal_expr(0), "0th");
    assert_eq!(num_to_ordinal_expr(1), "1st");
    assert_eq!(num_to_ordinal_expr(12), "12th");
    assert_eq!(num_to_ordinal_expr(22), "22nd");
    assert_eq!(num_to_ordinal_expr(43), "43rd");
    assert_eq!(num_to_ordinal_expr(67), "67th");
    assert_eq!(num_to_ordinal_expr(1901), "1901st");
}
