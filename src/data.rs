pub enum BinaryTree<T> {
    Node { val: T, left: Box<BinaryTree<T>>, right: Box<BinaryTree<T>> },
    Cotoco,
}

pub struct Tree<T> {
    pub val: T,
    pub branches: Vec<Tree<T>>
}

#[cfg(test)]
mod tests {
    use super::BinaryTree::{Node, Cotoco};
    use super::Tree;

    #[test]
    fn make_binary_tree() {
        let _bt = Node {
            val: true,
            left: Box::new(Node{
                val: false,
                left: Box::new(Cotoco),
                right: Box::new(Cotoco),
            }),
            right: Box::new(Cotoco),
        };
    }

    #[test]
    fn make_tree() {
        let _tree = Tree {
            val: true,
            branches: Vec::from([
                Tree { val: true, branches: Vec::from(
                    [Tree {val: false, branches: Vec::new()}]
                ) },
                Tree { val: false, branches: Vec::new() },
            ])
        };
    }
}
