use crate::ast::ast::Node;

pub trait Visitor {
    fn visit(&self, node: &Node) -> Option<&dyn Visitor>;
}

pub fn walk(v: &dyn Visitor, node: &Node) {
    if let Some(v) = v.visit(node) {
        match node {
            Node::Loop(n) => walk(v, &n.body),
            Node::Program(n) => walk(v, &n.body),
            Node::Body(n) => walk_for_list(v, &n.list),
            _ => {}
        }
    }
}

fn walk_for_list(v: &dyn Visitor, list: &Vec<Node>) {
    for child in list {
        walk(v, child);
    }
}
