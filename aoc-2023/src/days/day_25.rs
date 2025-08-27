//!day_25.rs

use anyhow::anyhow;
use anyhow::Result as AnyResult;
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use rustworkx_core::Result;

struct WiringDiagramm<'a> {
    graph: UnGraph<&'a str, u32>,
}

impl<'a> From<&'a str> for WiringDiagramm<'a> {
    fn from(value: &'a str) -> Self {
        let mut wd = Self {
            graph: UnGraph::new_undirected(),
        };
        for line in value.lines() {
            let (current_node, nodes_to_link) = line
                .split_once(':')
                .map(|(c, n)| (c.trim(), n.split_whitespace()))
                .unwrap();
            let current_node_index = wd.check_add_node(current_node);
            for link_node in nodes_to_link {
                let link_node_index = wd.check_add_node(link_node);
                wd.graph.add_edge(current_node_index, link_node_index, 1);
            }
        }
        wd
    }
}

impl<'a> WiringDiagramm<'a> {
    fn check_add_node(&mut self, node_label: &'a str) -> NodeIndex<u32> {
        let node_index = self
            .graph
            .node_indices()
            .find(|i| self.graph[*i] == node_label);
        match node_index {
            Some(index) => index,
            None => self.graph.add_node(node_label),
        }
    }
    fn calc_min_set(&self) -> AnyResult<usize> {
        let min_cut_res: Result<Option<(usize, Vec<_>)>> =
            stoer_wagner_min_cut(&self.graph, |_| Ok(1));
        let (min_cut, partition) = match min_cut_res {
            Ok(min_cut_res) => min_cut_res.ok_or(anyhow!("min cut did not yield a result"))?,
            Err(_) => return Err(anyhow!("min cut result in error")),
        };

        assert_eq!(min_cut, 3);
        let num_half1 = partition.len();
        let num_half2 = self.graph.node_count() - num_half1;

        Ok(num_half1 * num_half2)
    }
}

pub fn day_25() -> AnyResult<()> {
    let input = include_str!("../../../../aoc_input/aoc-2023/day_25.txt");
    let wd = WiringDiagramm::from(input);
    let result_part1 = wd.calc_min_set()?;
    eprintln!("result day 24 part 1: {}", result_part1);
    assert_eq!(result_part1, 562_978);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example_part_1() -> Result<()> {
        let input = "jqt: rhn xhk nvd\n\
                           rsh: frs pzl lsr\n\
                           xhk: hfx\n\
                           cmg: qnr nvd lhk bvb\n\
                           rhn: xhk bvb hfx\n\
                           bvb: xhk hfx\n\
                           pzl: lsr hfx nvd\n\
                           qnr: nvd\n\
                           ntq: jqt hfx bvb xhk\n\
                           nvd: lhk\n\
                           lsr: lhk\n\
                           rzs: qnr cmg lsr rsh\n\
                           frs: qnr lhk lsr";
        let wd = WiringDiagramm::from(input);
        let result_part1 = wd.calc_min_set().unwrap();
        assert_eq!(result_part1, 54);
        Ok(())
    }
}
