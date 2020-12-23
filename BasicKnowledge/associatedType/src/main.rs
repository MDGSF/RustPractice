// // use generic parameters
// trait Graph1<N, E> {
//   fn has_edge(&self, &N, &N) -> bool;
//   fn edges(&self, &N) -> Vec<E>;
// }
//
// fn distance<N, E, G: Graph1<N, E>>(graph: &G, start: &N, end: &N) -> u32 {}

// use associated type
trait Graph {
  type N;
  type E;
  fn has_edge(&self, &Self::N, &Self::N) -> bool;
  fn edges(&self, &Self::N) -> Vec<Self::E>;
}

fn distance<G: Graph>(graph: &G, start: &G::N, end: &G::N) -> u32 {}

struct Node;

struct Edge;

struct SimpleGraph;

impl Graph for SimpleGraph {
  type N = Node;
  type E = Edge;

  fn has_edge(&self, n1: &Node, n2: &Node) -> bool {}

  fn edges(&self, n: &Node) -> Vec<Edge> {}
}

fn main() {
  println!("Hello, world!");
}
