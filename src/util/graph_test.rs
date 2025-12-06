#[cfg(test)]
use crate::util::graph;

#[cfg(test)]
mod tests {
    use super::*;

    fn get_base_graph() -> graph::Graph<i32> {
        let mut g = graph::Graph::new();
        g.add_node(1);
        g.add_node(2);
        g.add_node(3);

        g.add_edge(1, 2);
        g.add_edge(2, 3);

        g
    }

    #[test]
    fn test_dfs_with_condition() {
        let g = get_base_graph();

        let mut visited = std::collections::HashMap::new();

        fn condition(s: &i32, e: &i32) -> bool {
            e - s == 1
        }

        let res = g.dfs_with_condition(&1, &3, condition, &mut visited);

        assert_eq!(res, true);
    }

    #[test]
    fn test_dfs_with_condition_false() {
        let g = get_base_graph();

        let mut visited = std::collections::HashMap::new();

        fn condition(s: &i32, e: &i32) -> bool {
            e - s == 2
        }

        let res = g.dfs_with_condition(&1, &3, condition, &mut visited);

        assert_eq!(res, false);
    }

    #[test]
    fn test_dfs_with_condition_complex() {
        let mut g = get_base_graph();
        g.add_node(4);
        g.add_node(5);
        g.add_node(6);
        g.add_node(7);
        g.add_node(8);
        g.add_node(9);

        g.add_bidirectional_edge(4, 1);
        g.add_bidirectional_edge(4, 5);
        g.add_bidirectional_edge(4, 7);

        g.add_bidirectional_edge(5, 2);
        g.add_bidirectional_edge(5, 6);
        g.add_bidirectional_edge(5, 8);

        g.add_bidirectional_edge(6, 3);
        g.add_bidirectional_edge(6, 9);

        g.add_bidirectional_edge(7, 8);

        g.add_bidirectional_edge(8, 9);

        let mut visited = std::collections::HashMap::new();

        fn condition(s: &i32, e: &i32) -> bool {
            (e - s).abs() == 1 || (e - s).abs() == 3
        }

        let res = g.dfs_with_condition(&1, &9, condition, &mut visited);

        assert_eq!(res, true);
    }
}
