use std::collections::VecDeque;
use std::io::{self, BufRead};

#[derive(Clone)]
struct Edge {
    t: usize,
    rev: usize,
    cap: i32,
    f: i32,
}

fn add_edge(graph: &mut Vec<Vec<Edge>>, s: usize, t: usize, cap: i32) {
    let rev_s = graph[t].len();
    let rev_t = graph[s].len();
    graph[s].push(Edge { t, rev: rev_s, cap, f: 0 });
    graph[t].push(Edge { t: s, rev: rev_t, cap: 0, f: 0 });
}

fn dinic_bfs(graph: &Vec<Vec<Edge>>, src: usize, dest: usize, dist: &mut Vec<i32>) -> bool {
    dist.iter_mut().for_each(|d| *d = -1);
    dist[src] = 0;
    let mut queue = VecDeque::new();
    queue.push_back(src);
    while let Some(u) = queue.pop_front() {
        for edge in &graph[u] {
            if dist[edge.t] < 0 && edge.f < edge.cap {
                dist[edge.t] = dist[u] + 1;
                queue.push_back(edge.t);
            }
        }
    }
    dist[dest] >= 0
}

fn dinic_dfs(
    graph: &mut Vec<Vec<Edge>>,
    ptr: &mut Vec<usize>,
    dist: &Vec<i32>,
    dest: usize,
    u: usize,
    f: i32,
) -> i32 {
    if u == dest {
        return f;
    }
    while ptr[u] < graph[u].clone().len() {
        let edge_idx = ptr[u];
        let edge = &mut graph.clone()[u][edge_idx];
        if dist[edge.t] == dist[u] + 1 && edge.f < edge.cap {
            let df = dinic_dfs(graph, ptr, dist, dest, edge.t, f.min(edge.cap - edge.f));
            if df > 0 {
                edge.f += df;
                graph[edge.t][edge.rev].f -= df;
                return df;
            }
        }
        ptr[u] += 1;
    }
    0
}

fn max_flow(graph: &mut Vec<Vec<Edge>>, src: usize, dest: usize) -> i32 {
    let mut flow = 0;
    let mut dist = vec![-1; graph.len()];
    while dinic_bfs(graph, src, dest, &mut dist) {
        let mut ptr = vec![0; graph.len()];
        while let Some(df) = (0..).find_map(|_| {
            let flow = dinic_dfs(graph, &mut ptr, &dist, dest, src, i32::MAX);
            if flow > 0 { Some(flow) } else { None }
        }) {
            flow += df;
        }
    }
    flow
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let line = lines.next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let p: usize = iter.next().unwrap().parse().unwrap();

    let num_nodes = 2 + n + m + p;
    let source = 0;
    let dest = num_nodes - 1;

    let mut graph = vec![Vec::new(); num_nodes];

    // source to kid and kid to toy
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        let toys: Vec<usize> = line.split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();
        add_edge(&mut graph, source, i, 1);
        for t in toys {
            let ti = n + t;
            add_edge(&mut graph, i, ti, 1);
        }
    }

    let mut used = std::collections::HashSet::new();
    // toy to toy group to dest
    for i in 1..=p {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let num_toys = values[0];
        let most = values[values.len() - 1];
        let group_id = i + n + m;
        for j in 1..=num_toys {
            let toy_id = values[j] + n;
            used.insert(values[j]);
            add_edge(&mut graph, toy_id, group_id, 1);
        }
        add_edge(&mut graph, group_id, dest, most as i32);
    }

    for i in 1..=m {
        if !used.contains(&(i as usize)) {
            add_edge(&mut graph, i + n, dest, 1);
        }
    }

    println!("{}", max_flow(&mut graph, source, dest));
}
