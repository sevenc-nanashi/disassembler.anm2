pub struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let parent = (0..size).collect();
        let rank = vec![0; size];
        UnionFind { parent, rank }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // Union by rank
            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }

    pub fn into_components_for_nodes(mut self, nodes: &[usize]) -> Vec<Vec<usize>> {
        let mut groups: std::collections::HashMap<usize, Vec<usize>> =
            std::collections::HashMap::new();
        for &node in nodes {
            let root = self.find(node);
            groups.entry(root).or_default().push(node);
        }
        groups.into_values().collect()
    }
}
