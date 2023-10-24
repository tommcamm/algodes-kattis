use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Node {
    id: i32,
    pair: (i32, i32),
    result: i32,
    is_pair: bool,
    matched: bool,
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
    deleted: HashSet<i32>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            arcs: Vec::new(),
            deleted: HashSet::new(),
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
        println!("=== [ Pair nodes ] ===\nID   PAIR");
        for node in &self.nodes {
            if node.is_pair {
                println!("{} : {:?}", node.id, node.pair);
            }
        }

        println!("=== [ Result nodes ] ===\nID  RESULT");
        for node in &self.nodes {
            if !node.is_pair {
                println!("{} : {:?}", node.id, node.result);
            }
        }
    }

    fn print_arcs(&self) {
        println!("=== [ Arcs ] ===");
        for arc in &self.arcs {
            println!("({}) -- {} --  ({})", arc.nodes.0, arc.operation, arc.nodes.1);
        }
    }

    fn get_node_by_id(&self, id: i32) -> Option<Node> {
        for node in &self.nodes {
            if node.id == id {
                return Some(node.clone());
            }
        }
        None
    }

    fn set_node_matched(&mut self, id: i32) {
        for node in &mut self.nodes {
            if node.id == id {
                node.matched = true;
            }
        }
    }

    // Used HashSet for bfs to make it more efficient
    fn bfs(&self, node: Node) -> HashSet<i32> {
        let mut visited_nodes = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(node.id);
        visited_nodes.insert(node.id);

        while let Some(current_node_id) = queue.pop_front() {
            for arc in &self.arcs {
                let next_node_id = if arc.nodes.0 == current_node_id { 
                    arc.nodes.1 
                } else if arc.nodes.1 == current_node_id { 
                    arc.nodes.0 
                } else {
                    continue;
                };

                if visited_nodes.insert(next_node_id) {
                    queue.push_back(next_node_id);
                }
            }
        }

        visited_nodes
    }

    // Depth-first search
    fn dfs(&self, node: Node) -> Option<Vec<Arc>> {
        let mut visited_arcs :Vec<Arc> = Vec::new();
        let mut visited_nodes :Vec<Node> = Vec::new();
        let mut stack :Vec<Node> = Vec::new();

        stack.push(node);

        while !stack.is_empty() {
            let current_node = stack.pop().unwrap(); // we pop the last element in the stack
            visited_nodes.push(current_node);
            
            for arc in &self.arcs {
                if arc.nodes.0 == current_node.id && !self.deleted.contains(&arc.nodes.1)  {
                    let next_node = self.get_node_by_id(arc.nodes.1).expect("missing node");
                    if !visited_nodes.contains(&next_node) && !stack.contains(&next_node) {
                        visited_arcs.push(arc.clone());
                        if next_node.is_pair && !next_node.matched {
                            return Some(visited_arcs);
                        }
                        stack.push(next_node);
                    }
                } else if arc.nodes.1 == current_node.id && !self.deleted.contains(&arc.nodes.0) {
                    let next_node = self.get_node_by_id(arc.nodes.0).expect("missing node");
                    if !visited_nodes.contains(&next_node) && !stack.contains(&next_node) {
                        visited_arcs.push(arc.clone());
                        if next_node.is_pair && !next_node.matched {
                            return Some(visited_arcs);
                        }
                        stack.push(next_node);
                    }
                }
            }
        }

        None
    } 

    fn hopcroft_karp(&mut self) -> Vec<Arc> {
        let mut matching :Vec<Arc> = Vec::new();
        let mut unmatched_nodes :HashSet<Node> = HashSet::new();

        // Let's get all the unmatched nodes
        self.nodes.iter()
            .filter(|p| p.is_pair && !p.matched)
            .flat_map(|node| self.bfs(node.clone()))
            .for_each(|node| {unmatched_nodes.insert(self.get_node_by_id(node).expect("can't find node"));});

        unmatched_nodes.iter().filter(|p| !p.is_pair && !p.matched).for_each(|node| {
            let path = match self.dfs(node.clone()) {
                Some(p) => p,
                None => Vec::new(),
            };

            // We delete the nodes we matched (if any) and set it matched
            path.into_iter().for_each(|arc| {
                self.deleted.insert(arc.nodes.0);
                self.set_node_matched(arc.nodes.0);
                self.deleted.insert(arc.nodes.1);
                self.set_node_matched(arc.nodes.1);
                matching.push(arc);
            });
            
        });
        self.deleted.clear();


         matching
    }

    fn print_output(&self, matching: Vec<Arc>, pairs: Vec<(i32, i32)>) {
        for pair in pairs {
            for arc in matching.clone() {
                let pair_node = self.get_node_by_id(arc.nodes.0).expect("Missing node");
                let result_node = self.get_node_by_id(arc.nodes.1).expect("Missing node");

                if pair_node.pair == pair {
                    println!("{} {} {} = {}", pair_node.pair.0, arc.operation, pair_node.pair.1, result_node.result);
                    break;
                }
            }
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
            matched: false,
        }
    }

    fn new_result(id: i32, result: i32) -> Self {
        Node {
            id: id,
            pair: (0, 0),
            result: result,
            is_pair: false,
            matched: false,
        }
    }
}


fn main(){
    let mut buf = String::new();
    let stdin = std::io::stdin();

    stdin.read_line(&mut buf).expect("Failed to read line");

    let n = buf.trim().parse::<i32>().expect("Invalid number of pairs");

    let mut graph = Graph::new();
    let mut pairs :Vec<(i32, i32)> = Vec::new();

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).expect("Failed to read line");
        let pair :Vec<i32> = buf.split_whitespace().map(|x| x.parse::<i32>().expect("Invalid number")).collect();
        pairs.push((pair[0], pair[1]));
    }

    graph.initialize(pairs.clone());
    
    let matching = graph.hopcroft_karp();

    if matching.len() < pairs.len() {
        println!("impossible");
    } else {
        graph.print_output(matching, pairs);
    }

    
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

    #[test]
    fn test_graph_bfs() {
        let mut graph = Graph::new();
        graph.initialize(vec![(1, 2), (1, 2), (2, 3)]);

        let found_nodes = graph.bfs(graph.nodes[0]);
        assert_eq!(found_nodes.len(), 8);   
        for node in found_nodes {
            println!("{:?}", node);
        }
    }

    #[test]
    fn test_graph_hopcroft_karp() {
        let mut graph = Graph::new();
        graph.initialize(vec![(1, 2), (1, 2), (2, 3)]);

        let matching = graph.hopcroft_karp();

        graph.print_nodes();
        for arc in matching {
            println!("{:?}", arc);
        }
    }

    #[test]
    fn test_graph_hopcroft_karp_fail() {
        let mut graph = Graph::new();
        graph.initialize(vec![(1, 2), (1, 2), (1, 2), (1, 2), (3, 1)]);

        let matching = graph.hopcroft_karp();

        graph.print_nodes();
        for arc in matching {
            println!("{:?}", arc);
        }
    }

    // test for execution time of hopcroft_karp, 10k pairs
    #[test]
    fn test_graph_hopcroft_karp_time() {
        let mut graph = Graph::new();
        let mut pairs :Vec<(i32, i32)> = Vec::new();
        for i in 0..500 {
            pairs.push((i, i));
        }
        
        // Measure time from here
        let start = std::time::Instant::now();
        
        graph.initialize(pairs);
        graph.hopcroft_karp();
        let end = std::time::Instant::now();

        println!("Execution time: {:?}", end.duration_since(start));
        //assert!(now.duration_since(now).as_secs() < 1);


    }
}
