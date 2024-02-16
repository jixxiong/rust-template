pub mod sssp {
    use std::collections::BinaryHeap;

    pub fn dijkstra_sssp(graph: Vec<Vec<(usize, i64)>>, source: usize) -> Vec<i64> {
        let n = graph.len() - 1;
        let mut dis = vec![i64::MAX; n + 1];
        let mut que = BinaryHeap::new();
        dis[source] = 0;
        que.push((0, source));
        while !que.is_empty() {
            let (f, u) = que.pop().unwrap();
            if -f >= dis[source] {
                continue;
            }
            for &(v, w) in graph[u].iter() {
                if dis[u] + w < dis[v] {
                    dis[v] = dis[u] + w;
                    que.push((-dis[v], v));
                }
            }
        }
        dis
    }
}
