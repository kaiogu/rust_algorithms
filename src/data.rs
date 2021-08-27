pub enum BinaryTree<T> {
    Node { val: T, left: Box<BinaryTree<T>>, right: Box<BinaryTree<T>> },
    Cotoco,
}

#[cfg(test)]
mod tests {
    use super::BinaryTree::*;

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
}
