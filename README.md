# rust-graph

Simple directed graph data structure

> Why?

To learn some `rust`. I read about people saying that implementing tree-like
structures in Rust was harder than in other languages, so I decided to try one
to learn some Rust.

## Quickstart

[WIP]

```rust
// TODO
```

## To-Do's

- [ ] Make `Node`'s generic
- [ ] Implement depth-first search
- [ ] Document shortest_path implementation more
- [ ] Add edge operations (modifying/removing/...)

## Implementation details

The graph is implemented as 2 arrays. One for the nodes and one for the edges.

Node removal is performed as:

1. Remove node from node array. The empty space in the array is filled with the last
   node in the node array, this making the operation `O(1)`.
2. Remove all the edges point to/from the removed array.
3. Modify the edges that were previously pointing to the last array and make them
   point to the new location.
