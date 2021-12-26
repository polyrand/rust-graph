#![allow(dead_code, unused_variables, unused_labels)]

use core::panic;
use std::collections::{HashSet, VecDeque};
use std::convert::From;
// use std::sync::RwLock;

type NodeIndex = usize;
type EdgeIndex = usize;

#[derive(PartialEq, Eq, Hash, Debug)]
struct PathStep(NodeIndex, EdgeIndex);
type Path = Vec<PathStep>;
// type NodeData = String;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum DataTypes {
    Text(String),
    Blob(Vec<u8>),
    Integer(isize),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Node {
    data: DataTypes,
}

impl From<&str> for Node {
    fn from(v: &str) -> Node {
        Node {
            data: DataTypes::Text(v.to_owned()),
        }
    }
}

impl From<String> for Node {
    fn from(v: String) -> Node {
        Node {
            data: DataTypes::Text(v),
        }
    }
}

impl From<isize> for Node {
    fn from(v: isize) -> Node {
        Node {
            data: DataTypes::Integer(v),
        }
    }
}

impl From<Vec<u8>> for Node {
    fn from(v: Vec<u8>) -> Node {
        Node {
            data: DataTypes::Blob(v),
        }
    }
}

impl Node {
    fn extract_int(&self) -> isize {
        match self.data {
            DataTypes::Integer(x) => x,
            _ => panic!("Method only available for int data"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
struct Edge {
    from: usize,
    to: usize,
}

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, new_node: Node) -> NodeIndex {
        for (idx, node) in self.nodes.iter().enumerate() {
            if node == &new_node {
                return idx;
            }
        }

        self.nodes.push(new_node);
        self.nodes.len() - 1
    }

    fn add_edge(&mut self, new_edge: Edge) -> EdgeIndex {
        for (idx, edge) in self.edges.iter().enumerate() {
            if edge == &new_edge {
                return idx;
            }
        }

        self.edges.push(new_edge);
        self.edges.len() - 1
    }

    fn find_node_idx(&self, node: Node) -> Option<NodeIndex> {
        for (idx, current_node) in self.nodes.iter().enumerate() {
            if current_node == &node {
                return Some(idx);
            }
        }

        None
    }

    fn remove_node(&mut self, node_idx: NodeIndex) -> Option<Node> {
        match self.nodes.get(node_idx) {
            None => None,
            Some(_) => {
                // retrieve current last idx because we are doing a swap_remove
                // and we will need to update the edges to the last node too
                let last_node_idx = self.nodes.len();

                let removed_node = self.nodes.swap_remove(node_idx);

                // remove all edges pointing to the removed node
                self.edges
                    .retain(|x| x.from != node_idx && x.to != node_idx);

                // if we just removed the last node, we don't need to update
                // more edges, otherwise, all the edges that were pointing to/from
                // the last node, now need to point to the new position (the one we just freed)
                if node_idx != last_node_idx {
                    for edge in self.edges.iter_mut() {
                        if edge.from == last_node_idx {
                            edge.from = node_idx;
                        }

                        if edge.to == last_node_idx {
                            edge.to = node_idx;
                        }
                    }
                }

                Some(removed_node)
            }
        }
    }

    fn reachable_nodes_from(&self, node_idx: NodeIndex) -> Vec<NodeIndex> {
        self.edges
            .iter()
            .filter(|e| e.from == node_idx)
            .map(|e| e.to)
            .collect()
    }

    fn nodes_that_can_reach(&self, node_idx: NodeIndex) -> Vec<NodeIndex> {
        self.edges
            .iter()
            .filter(|e| e.to == node_idx)
            .map(|e| e.from)
            .collect()
    }

    fn bfs_distance(&self, start: NodeIndex, end: NodeIndex) -> usize {
        if start == end {
            return 0;
        }

        let mut queue: VecDeque<NodeIndex> = VecDeque::new();
        let mut visited: HashSet<NodeIndex> = HashSet::new();
        let mut distance = 0;

        queue.push_front(start);

        while !queue.is_empty() {
            let working_node = queue[0];

            for neighbour in self
                .reachable_nodes_from(working_node)
                .iter()
                .filter(|node| !visited.contains(node))
            {
                queue.push_back(*neighbour);

                if *neighbour == end {
                    return distance + 1;
                }
            }

            let finished = queue.pop_front().unwrap();
            visited.insert(finished);
            distance += 1
        }

        distance
    }

    fn boundary(&self) -> Option<Vec<NodeIndex>> {
        // find all nodes that do NOT have a "from" edge, that is:
        // other nodes may reach it but it doesn't reach any, thus making it
        // a "boundary" node.

        let froms: HashSet<usize> = self.edges.iter().map(|e| e.from).collect();

        let b: Vec<NodeIndex> = (0..self.nodes.len())
            .filter(|node_idx| !froms.contains(node_idx))
            .collect();

        if b.is_empty() {
            None
        } else {
            Some(b)
        }
    }

    // Alias function for tree structures
    fn leaves(&self) -> Option<Vec<NodeIndex>> {
        self.boundary()
    }

    /*

    Note: the original graph (&self) node indexes will be wrapped as new nodes
    in the path three.

    Strategy:
    1. Build a tree (path_tree), using the "start" as the root.
    2. Append leaves as we explore the graph
        Each leave is a node, the data contained in the node is the index of our
        original graph (&self). `path_tree` will contain its own node indexes

    3. If we find the destination node, we need to "backtrack" the tree we just
        built (`path_tree`) back to the root node to find the path we went through.
        Backtracking strategy:

        - The path starts with the node we just found (the destination)
        path: [just_found_idx]
        - We have found the destination while looking for neighbours, in terms of our
          `path_tree`, we are still in the previous node. We inspect the data
          of the node we are iterating over and append that to the path.
        path: [just_found_idx, current_exploration_node]
        - Go back in the tree (`path_tree`), building the path. Notice that we go back
          the path_tree by using its node/edge indexes, but at the same time we are
          appending to our `path` the indexes of the original graph, which are contained
          in the `path_tree` nodes.
        path: [just_found_idx, current_exploration_node, ...
    */
    fn shortest_path(&self, start: NodeIndex, end: NodeIndex) -> Option<Vec<NodeIndex>> {
        let mut visited_graph: HashSet<NodeIndex> = HashSet::new();
        visited_graph.insert(start);

        let mut visited_tree: HashSet<NodeIndex> = HashSet::new();

        // set the starting node as the root of our path tree
        let mut path_tree = Graph::new();
        let first_node = path_tree.add_node(Node::from(start as isize));
        visited_tree.insert(first_node);

        // loop as long as we have paths to explore
        // or we haven't found the destination
        loop {
            // working node is the index of the **path** tree,
            // NOT our original graph
            for path_tree_node_idx in path_tree.leaves().unwrap().iter() {
                let path_node = &path_tree.nodes[*path_tree_node_idx];
                let orig_graph_node_idx = path_node.extract_int();

                // now we find all the neighbour nodes in our graph

                'neighbours: for neighbour_idx in self
                    .reachable_nodes_from(orig_graph_node_idx.try_into().unwrap())
                    .iter()
                {
                    if visited_graph.contains(neighbour_idx) {
                        continue 'neighbours;
                    }

                    if *neighbour_idx == end {
                        /*
                        Found end of path!

                        Now we start backtracking from our current situation
                        back to the root of the path_tree. We will keep track of the
                        data contained in parent nodes as we backtrack. This will
                        become the path used to reach our objective node index.
                        */
                        let mut path = vec![*neighbour_idx];

                        let prev_node_data = path_tree.nodes[*path_tree_node_idx].extract_int();
                        path.push(prev_node_data as usize);

                        let mut current_path_tree_node_idx = *path_tree_node_idx;

                        // start backtracking
                        loop {
                            let parent = path_tree
                                .edges
                                .iter()
                                .find(|path_edge| path_edge.to == current_path_tree_node_idx);

                            match parent {
                                // We are still backtracking, add the data in the
                                // current node to the path, and keep moving
                                Some(edge) => {
                                    let path_tree_parent = &path_tree.nodes[edge.from];
                                    current_path_tree_node_idx = edge.from;
                                    let orig_graph_idx = path_tree_parent.extract_int();
                                    path.push(orig_graph_idx as usize);
                                }
                                // None = we reached the tree root, we can return the path
                                // we need to reverse the path because we were pushing items starting
                                // from the end of the path until we reach the root
                                None => return Some(path.into_iter().rev().collect()),
                            }
                        }
                    }

                    let idx = path_tree.add_node(Node::from(*neighbour_idx as isize));
                    path_tree.add_edge(Edge {
                        from: *path_tree_node_idx,
                        to: idx,
                    });
                    visited_graph.insert(*path_tree_node_idx);

                    // if we have visited all the node but didn't find the objective
                    if visited_graph.len() == self.nodes.len() {
                        return None;
                    }
                }

                visited_tree.insert(*path_tree_node_idx);

                visited_tree
                    .difference(&HashSet::from_iter(
                        path_tree.leaves().unwrap().iter().copied(),
                    ))
                    .next()?;
            }
        }
    }
}

/*
                            ┌──────┐
                            │      │
                    ┌─────► │  N2  │
                    │       │      │
                    │       └──────┘
                    │
                    │
                ┌───┴──┐
    ┌───────────┤      │
    │           │  N0  │           ┌──────┐
    ▼           │      ├─────────► │      │
 ┌──────┐       └────┬─┘           │  N3  │
 │      │            │             │      │
 │  N1  │            │             └───┬──┘
 │      │            │                 │
 └──────┘            │                 │
                     ▼                 │
                   ┌──────┐            ▼
                   │      │         ┌──────┐
                   │  N4  │         │      │
                   │      │ ───────►│  N5  │
                   └──────┘         │      │
                                    └──────┘
*/

fn main() {
    let mut g2 = Graph::new();

    g2.add_node(Node::from(1));
    println!("Single node b {:?}", g2.boundary());

    let mut g = Graph::new();

    let idx0 = g.add_node(Node::from("hello"));
    let idx1 = g.add_node(Node::from("world"));
    let idx2 = g.add_node(Node::from("foo"));
    let idx3 = g.add_node(Node::from("bar"));
    let idx4 = g.add_node(Node::from("baz"));
    let idx5 = g.add_node(Node::from("asd"));

    println!("Reachable {:?}", g.reachable_nodes_from(idx2));

    g.add_edge(Edge {
        from: idx0,
        to: idx1,
    });
    g.add_edge(Edge {
        from: idx0,
        to: idx2,
    });
    g.add_edge(Edge {
        from: idx0,
        to: idx3,
    });
    g.add_edge(Edge {
        from: idx0,
        to: idx4,
    });

    g.add_edge(Edge {
        from: idx3,
        to: idx5,
    });
    g.add_edge(Edge {
        from: idx4,
        to: idx5,
    });

    // println!("{:#?}", g);
    println!("Boundary: {:?}", g.boundary().unwrap());

    println!("Bfs {}", g.bfs_distance(0, 5));
    println!("Shortest path {:?}", g.shortest_path(0, 5));
    println!("Shortest path {:?}", g.shortest_path(2, 5));
    println!("Shortest path {:?}", g.shortest_path(3, 5));

    println!("==============");
    println!("Removing");

    let rm = g.remove_node(idx3).unwrap();

    println!("Removed node: {:#?}", rm);

    println!("{:?}", g);

    println!("{:?}", g.reachable_nodes_from(0));
    println!("{:?}", g.nodes_that_can_reach(1));
}
