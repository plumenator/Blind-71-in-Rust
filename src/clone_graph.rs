// https://leetcode.com/problems/clone-graph

// 133. Clone Graph

// Given a reference of a node in a connected undirected graph.

// Return a deep copy (clone) of the graph.

// Each node in the graph contains a value (int) and a list (List[Node]) of its neighbors.

// class Node {
//     public int val;
//     public List<Node> neighbors;
// }

// Test case format:

// For simplicity, each node's value is the same as the node's index (1-indexed). For example, the first node with val == 1, the second node with val == 2, and so on. The graph is represented in the test case using an adjacency list.

// An adjacency list is a collection of unordered lists used to represent a finite graph. Each list describes the set of neighbors of a node in the graph.

// The given node will always be the first node with val = 1. You must return the copy of the given node as a reference to the cloned graph.

// Example 1:

// Input: adjList = [[2,4],[1,3],[2,4],[1,3]]
// Output: [[2,4],[1,3],[2,4],[1,3]]
// Explanation: There are 4 nodes in the graph.
// 1st node (val = 1)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
// 2nd node (val = 2)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
// 3rd node (val = 3)'s neighbors are 2nd node (val = 2) and 4th node (val = 4).
// 4th node (val = 4)'s neighbors are 1st node (val = 1) and 3rd node (val = 3).
// Example 2:

// Input: adjList = [[]]
// Output: [[]]
// Explanation: Note that the input contains one empty list. The graph consists of only one node with val = 1 and it does not have any neighbors.
// Example 3:

// Input: adjList = []
// Output: []
// Explanation: This an empty graph, it does not have any nodes.

// Constraints:

// The number of nodes in the graph is in the range [0, 100].
// 1 <= Node.val <= 100
// Node.val is unique for each node.
// There are no repeated edges and no self-loops in the graph.
// The Graph is connected and all nodes can be visited starting from the given node.

// C++ solution

// /*
// // Definition for a Node.
// class Node {
// public:
//     int val;
//     vector<Node*> neighbors;
//     Node() {
//         val = 0;
//         neighbors = vector<Node*>();
//     }
//     Node(int _val) {
//         val = _val;
//         neighbors = vector<Node*>();
//     }
//     Node(int _val, vector<Node*> _neighbors) {
//         val = _val;
//         neighbors = _neighbors;
//     }
// };
// */
// #include<unordered_map>
// #include<iostream>

// class Solution {
//     std::unordered_map<int, Node*> visited;

// public:
//     Node* cloneGraph(Node* node) {
//         static int count = 0;
//         if (!node) {
//             return nullptr;
//         }
//         if (auto visitedIt = visited.find(node->val); visitedIt != visited.end()) {
//             return visitedIt->second;
//         }
//         auto* copy = new Node(node->val);
//         visited.emplace(node->val, copy);
//         for (auto n: node->neighbors) {
//             if (auto clone = cloneGraph(n); clone) {
//                 copy->neighbors.push_back(clone);
//             }
//         }
//         return copy;
//     }
// };

use std::collections::HashMap;
use std::ptr;

pub struct Node {
    pub val: i32,
    pub neighbors: Vec<*const Node>,
}

pub struct Problem {
    visited: HashMap<i32, *mut Node>,
}

impl Problem {
    pub fn create_graph(&mut self, adj_list: Vec<Vec<i32>>) -> *mut Node {
        for (val, neighbors) in adj_list.into_iter().enumerate() {
            let val = (val + 1) as i32;
            let node = self.mk_node(val);
            for nval in neighbors {
                let neighbor = self.mk_node(nval);
                unsafe {
                    (*node).neighbors.push(neighbor);
                }
                self.visited.insert(nval, neighbor);
            }
        }
        if self.visited.is_empty() {
            ptr::null_mut()
        } else {
            self.visited[&1]
        }
    }

    fn mk_node(&self, val: i32) -> *mut Node {
        if let Some(n) = self.visited.get(&val) {
            *n
        } else {
            let n = Box::new(Node {
                val,
                neighbors: vec![],
            });
            Box::into_raw(n)
        }
    }

    pub fn equal(&self, other: *const Node) -> bool {
        if other.is_null() {
            return true;
        }
        unsafe {
            let val = (*other).val;
            if self.visited.contains_key(&val) {
                let neighbors = &(*other).neighbors;
                neighbors.iter().all(|n| self.equal(*n))
            } else {
                false
            }
        }
    }
}

pub struct Solution {
    visited: HashMap<i32, *mut Node>,
}

impl Solution {
    pub fn clone_graph(&mut self, node: *const Node) -> *mut Node {
        if node.is_null() {
            return ptr::null_mut();
        }
        unsafe {
            let val = (*node).val;
            if let Some(v) = self.visited.get(&val) {
                return *v;
            }
            let copy = Box::new(Node {
                val,
                neighbors: vec![],
            });
            let copy = Box::into_raw(copy);
            self.visited.insert(val, copy);
            let neighbors = &(*node).neighbors;
            let copy_neighbors = &mut (*copy).neighbors;
            for n in neighbors {
                let clone = self.clone_graph(*n);
                if !clone.is_null() {
                    copy_neighbors.push(clone);
                }
            }
        }
        ptr::null_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut problem = Problem {
            visited: Default::default(),
        };
        let node = problem.create_graph(vec![vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]]);
        let clone = Solution {
            visited: Default::default(),
        }
        .clone_graph(node);
        assert!(problem.equal(clone))
    }

    #[test]
    fn ex2() {
        let mut problem = Problem {
            visited: Default::default(),
        };
        let node = problem.create_graph(vec![vec![]]);
        let clone = Solution {
            visited: Default::default(),
        }
        .clone_graph(node);
        assert!(problem.equal(clone))
    }

    #[test]
    fn ex3() {
        let mut problem = Problem {
            visited: Default::default(),
        };
        let node = problem.create_graph(vec![]);
        let clone = Solution {
            visited: Default::default(),
        }
        .clone_graph(node);
        assert!(problem.equal(clone))
    }

    #[test]
    fn ex4() {
        let mut problem = Problem {
            visited: Default::default(),
        };
        let node = problem.create_graph(vec![
            vec![2, 3, 4],
            vec![1, 3, 4],
            vec![1, 2, 4],
            vec![1, 2, 3],
        ]);
        let clone = Solution {
            visited: Default::default(),
        }
        .clone_graph(node);
        assert!(problem.equal(clone))
    }

    #[test]
    fn ex5() {
        let mut problem = Problem {
            visited: Default::default(),
        };
        let node = problem.create_graph(vec![
            vec![2, 3, 4, 5, 6],
            vec![1, 3, 4, 5, 6],
            vec![1, 2, 4, 5, 6],
            vec![1, 2, 3, 5, 6],
            vec![1, 2, 3, 4, 6],
            vec![1, 2, 3, 4, 5],
        ]);
        let clone = Solution {
            visited: Default::default(),
        }
        .clone_graph(node);
        assert!(problem.equal(clone))
    }
}
