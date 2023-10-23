#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    id: i32,
    pair: (i32, i32),
    result: i32,
    is_pair: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Arc {
    nodes: (i32, i32),
    operation: char,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Graph {
    nodes: Vec<Node>,
    arcs: Vec<Arc>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            arcs: Vec::new(),
        }
    }

    fn initialize(&mut self, pairs: Vec<(i32, i32)>) {
        let mut nodes :Vec<Node> = Vec::new();
        let mut arcs :Vec<Arc> = Vec::new();

        for pair in pairs {
            // Let's create the pair node first
            let node = self.add_pair(pair);

            // Let's create the result node (if it does not exists) and add the arc to it
            self.add_result(pair, '+');
            self.add_arc(node, '+');

            self.add_result(pair, '-');
            self.add_arc(node, '-');

            self.add_result(pair, '*');
            self.add_arc(node, '*');
        } 
    }

    fn add_pair(&mut self, pair: (i32, i32)) -> Node {
        // We can have multiple nodes with the same pair
        let pair_node = Node::new_pair(self.nodes.len() as i32, pair); // id always the last index
        self.nodes.push(pair_node);
        pair_node
    }

    fn add_result(&mut self, pair: (i32, i32), operation: char) {
        let result = match operation {
            '+' => pair.0 + pair.1,
            '-' => pair.0 - pair.1,
            '*' => pair.0 * pair.1,
            _ => panic!("Invalid operation"),
        };

        for current_result in &self.nodes {
            if !current_result.is_pair && current_result.result == result {
                return;
            }
        }

        let result_node = Node::new_result(self.nodes.len() as i32, result); // id always the last index
        self.nodes.push(result_node);
    }

    fn get_result_node(&mut self, result: i32, operation: char) -> Option<Node> {
        for node in &self.nodes {
            if !node.is_pair && node.result == result {
                return Some(node.clone());
            }
        }
        None
    }

    fn add_arc(&mut self, pair_node: Node, operation: char) {
        let result_node = self.get_result_node(match operation {
            '+' => pair_node.pair.0 + pair_node.pair.1,
            '-' => pair_node.pair.0 - pair_node.pair.1,
            '*' => pair_node.pair.0 * pair_node.pair.1,
            _ => panic!("Invalid operation"),
        }, operation).expect("Missing result node");

        let arc = Arc {
            nodes: (pair_node.id, result_node.id),
            operation: operation,
        };

        self.arcs.push(arc);
    }

    fn print_nodes(&self) {
        println!("Pair nodes:");
        for node in &self.nodes {
            if node.is_pair {
                println!("{}: {:?}", node.id, node.pair);
            }
        }

        println!("Result nodes:");
        for node in &self.nodes {
            if !node.is_pair {
                println!("{}: {:?}", node.id, node.result);
            }
        }
    }

    fn print_arcs(&self) {
        println!("Arcs:");
        for arc in &self.arcs {
            println!("({}) [{}] ({})", arc.nodes.0, arc.operation, arc.nodes.1);
        }
    }

}

impl Node {
    fn new_pair(id: i32, pair: (i32, i32)) -> Self {
        Node {
            id: id,
            pair: pair,
            result: 0,
            is_pair: true,
        }
    }

    fn new_result(id: i32, result: i32) -> Self {
        Node {
            id: id,
            pair: (0, 0),
            result: result,
            is_pair: false,
        }
    }
}


fn main(){
    panic!("Still need to implement main...");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_graph() {
        let mut graph = Graph::new();
        graph.initialize(vec![(1, 2), (1, 2), (2, 3)]);

        graph.print_nodes();
        graph.print_arcs();
    }
}
