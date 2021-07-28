# Rust入门实战与进阶

## 代码模板

### dfs 递归

```rust
let mut visited = Vec::new();

fn dfs(node, visited) {
  // 终止条件：已经访问过当前节点
  if visited.contains(node) {
    return;
  }

  // 将当前节点加入visited
  visited.push(node);

  // 处理当前节点
  process(node);

  // 获得当前节点的子节点并递归执行
  let child_nodes = generate_child_nodes(node);
  for child_node in child_nodes {
    if !visited.contains(child_node) {
      dfs(child_node, visited);
    }
  }
}
```

### dfs 非递归

```rust
fn dfs(root) {
  if root.is_none() {
    return;
  }

  let mut visited = Vec::new();
  let mut stack = Vec::new();
  stack.push(root);

  while !stack.is_empty() {
    let node = stack.pop();

    visited.push(node);

    process(node);

    let releated_nodes = generate_related_nodes(node);
    for releated_node in releated_nodes {
      stack.push(releated_node);
    }
  }
}
```

### bfs

```rust
fn bfs(root) {
  if root.is_none() {
    return;
  }

  let mut visited = Vec::new();
  let mut deque = VecDeque::new();
  deque.push_back(root);

  while !stack.is_empty() {
    let node = stack.pop_front();

    visited.push(node);

    process(node);

    let releated_nodes = generate_related_nodes(node);
    for releated_node in releated_nodes {
      stack.push_back(releated_node);
    }
  }
}
```

