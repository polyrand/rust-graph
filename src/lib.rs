pub mod graph;

pub use crate::graph::Edge;
pub use crate::graph::Graph;
pub use crate::graph::Node;

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

#[cfg(test)]
mod tests {

    use super::*;

    fn generate_base_graph() -> Graph {
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

        g
    }

    #[test]
    fn single_node_is_boundary() {
        let mut g2 = Graph::new();

        let idx = g2.add_node(Node::from(1));
        assert_eq!(vec![idx], g2.boundary().unwrap());
    }

    #[test]
    fn test_boundary() {
        // println!("{:#?}", g);
        let g = generate_base_graph();
        assert_eq!(vec![1, 2, 5], g.boundary().unwrap());
    }

    #[test]
    fn test_bfs_distance() {
        let g = generate_base_graph();
        assert_eq!(0, g.bfs_distance(0, 0));
        assert_eq!(0, g.bfs_distance(2, 2));
        assert_eq!(4, g.bfs_distance(0, 5));
    }

    #[test]
    fn shortest_path() {
        let g = generate_base_graph();
        let path = g.shortest_path(0, 5).unwrap();

        assert!(path == vec![0, 3, 5] || path == vec![0, 4, 5]);
        assert_eq!(vec![3, 5], g.shortest_path(3, 5).unwrap());

        assert_eq!(None, g.shortest_path(2, 5));
        assert_eq!(None, g.shortest_path(1, 5));
    }

    // println!("==============");
    // println!("Removing");

    // let rm = g.remove_node(idx3).unwrap();

    // println!("Removed node: {:#?}", rm);

    // println!("{:?}", g);

    // println!("{:?}", g.reachable_nodes_from(0));
    // println!("{:?}", g.nodes_that_can_reach(1));
}
