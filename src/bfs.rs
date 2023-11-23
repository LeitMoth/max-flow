use std::collections::{HashMap, HashSet, VecDeque};

fn bfs(
    g: &Vec<HashSet<usize>>,
    s: usize,
    t: usize,
) -> Option<(HashMap<usize, usize>, HashMap<usize, usize>)>
{
    let mut q = VecDeque::new();
    let mut parent = HashMap::new();
    let mut level = HashMap::new();
    let mut visited = HashSet::new();

    q.push_back(s);
    visited.insert(s);
    level.insert(s, 0);
    parent.insert(s, s);

    while let Some(u) = q.pop_front() {
        for &v in &g[u] {
            if !visited.contains(&v) {
                visited.insert(v);
                level.insert(v, level[&u] + 1);
                parent.insert(v, u);
                q.push_back(v);

                if v == t {
                    return Some((parent, level));
                }
            }
        }
    }

    return None;
}

pub fn find_path(g: &Vec<HashSet<usize>>, s: usize, t: usize) -> Option<Vec<usize>>
{
    if let Some((parent, _)) = bfs(g, s, t) {
        let mut next = t;
        let mut res = Vec::new();
        loop {
            res.push(next);
            if next == parent[&next] {
                break;
            }
            next = parent[&next];
        }
        res.reverse();
        Some(res)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bfs_works() {
        let g = vec![vec![1, 2], vec![4, 0], vec![3], vec![], vec![5], vec![0]];

        // I started out with a vec of vecs, so I wrote this quick converter

        let g = g.iter().map(|i| {
            let mut h = HashSet::new();
            for &x in i {
                h.insert(x);
            }
            h
        }).collect();

        let res = find_path(&g, 1, 3);
        assert_eq!(res, Some(vec![1, 0, 2, 3]));
    }

}
