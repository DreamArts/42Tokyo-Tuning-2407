use sqlx::FromRow;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(FromRow, Clone, Debug)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(FromRow, Clone, Debug)]
pub struct Edge {
    pub node_a_id: i32,
    pub node_b_id: i32,
    pub weight: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<i32, Node>,
    pub edges: HashMap<i32, Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges
            .entry(edge.node_a_id)
            .or_default()
            .push(edge.clone());

        let reverse_edge = Edge {
            node_a_id: edge.node_b_id,
            node_b_id: edge.node_a_id,
            weight: edge.weight,
        };
        self.edges
            .entry(reverse_edge.node_a_id)
            .or_default()
            .push(reverse_edge);
    }

    pub fn dijkstra(&self, start: i32) -> HashMap<i32, i32> {
        let mut distances: HashMap<i32, i32> =
            self.nodes.keys().map(|&node| (node, i32::MAX)).collect();
        let mut heap = BinaryHeap::new();

        distances.insert(start, 0);
        heap.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if cost > *distances.get(&position).unwrap() {
                continue;
            }

            if let Some(neighbors) = self.edges.get(&position) {
                for edge in neighbors {
                    let next = State {
                        cost: cost + edge.weight,
                        position: edge.node_b_id,
                    };

                    if next.cost < *distances.get(&edge.node_b_id).unwrap() {
                        heap.push(next);
                        distances.insert(edge.node_b_id, next.cost);
                    }
                }
            }
        }

        distances
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: i32,
}

// The priority queue depends on `Ord`.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost) // Notice that the we flip the ordering on costs.
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
