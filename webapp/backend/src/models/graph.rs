use sqlx::FromRow;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

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

    pub fn shortest_path(&self, from_node_id: i32, to_node_id: i32) -> i32 {
        #[derive(Copy, Clone, Eq, PartialEq)]
        struct State {
            cost: i32,
            node: i32,
        }

        // The priority queue depends on `Ord`. Explicitly implement the trait so the queue becomes a min-heap
        // instead of a max-heap.
        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                // Notice that the we flip the ordering on costs.
                // In case of a tie we compare the nodes.
                other.cost.cmp(&self.cost).then_with(|| self.node.cmp(&other.node))
            }
        }

        // `PartialOrd` needs to be implemented as well.
        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut dist: HashMap<i32, i32> = HashMap::new();
        let mut heap = BinaryHeap::new();

        // Initialize distances
        for &node_id in self.nodes.keys() {
            dist.insert(node_id, i32::MAX);
        }
        dist.insert(from_node_id, 0);
        heap.push(State { cost: 0, node: from_node_id });

        while let Some(State { cost, node }) = heap.pop() {
            // Early exit if we reach the target node
            if node == to_node_id {
                return cost;
            }

            // Skip if the current cost is greater than the recorded shortest distance
            if cost > *dist.get(&node).unwrap() {
                continue;
            }

            // Process each edge connected to the current node
            if let Some(edges) = self.edges.get(&node) {
                for edge in edges {
                    let next = edge.node_b_id;
                    let next_cost = cost + edge.weight;

                    // Only consider this new path if it's better
                    if next_cost < *dist.get(&next).unwrap() {
                        heap.push(State { cost: next_cost, node: next });
                        dist.insert(next, next_cost);
                    }
                }
            }
        }

        // If we reach here, there is no path
        -1
    }
}
