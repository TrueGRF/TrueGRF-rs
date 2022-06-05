use std::collections::HashMap;
use petgraph::graph::Graph;
use petgraph::visit::Dfs;
use petgraph::algo::is_cyclic_directed;

use super::actions::{
    Action8,
    Action14,
    ActionF,
    ActionFPart,
    ActionFName,
    ActionFId,
    ActionTrait,
};

use super::config::{
    NewGRFTownnamePart,
    NewGRFConfigTownname,
};

use super::Output;

fn create_part<'a>(part: &'a NewGRFTownnamePart, firstbit: &mut u8, subparts_ids: &HashMap<String, u8>) -> Result<ActionFPart<'a>, String> {
    let mut total_probability = 0;
    for name in &part.names {
        total_probability += name.probability;
    }

    /* Find out how many bits we need to fit this list. */
    let bitcount = (total_probability as f32 + 1.).log2().ceil() as u8;
    if *firstbit + bitcount >= 32 {
        return Err("Parts consume more bits than available; reduce complexity of your set".to_string());
    }

    let mut names = Vec::new();

    for name in &part.names {
        let probability = name.probability;

        if name.name.starts_with("@") {
            if !subparts_ids.contains_key(&name.name[1..]) {
                return Err(format!("Subpart {} not defined", name.name));
            }
            names.push(
                ActionFId { probability, id: subparts_ids[&name.name[1..]] }.into()
            );
        } else {
            names.push(
                ActionFName { probability, name: &name.name }.into()
            );
        }
    }

    let subpart = ActionFPart {
        firstbit: *firstbit,
        bitcount: bitcount,
        names: names,
    };

    *firstbit += bitcount;

    Ok(subpart)
}

pub fn write_townname_segments(output: &mut Output, options: NewGRFConfigTownname) -> Result<(), String> {
    Action14::Url { url: &options.general.url.to_string() }.write(output);
    Action14::Palette { palette: 'D' }.write(output);
    Action8::General { grfid: &hex::decode(options.general.grfid).unwrap(), name: &options.general.name, description: &options.general.description }.write(output);

    let mut id = 0;

    for townname in options.townnames {
        if !townname.available {
            continue;
        }

        let mut graph = Graph::new();

        /* Create the nodes in the graph. */
        let main_node = graph.add_node(None);
        let mut sub_nodes = HashMap::new();
        for set in &townname.subsets {
            sub_nodes.insert(&set.name, graph.add_node(Some(set)));
        }

        /* Find references, create the edges, starting the the mainset. */
        for part in &townname.mainset {
            for name in &part.names {
                if name.name.starts_with("@") {
                    graph.add_edge(main_node, sub_nodes[&name.name[1..].to_string()], 1);
                }
            }
        }
        /* Find references, create the edges, but now for all subsets. */
        for set in &townname.subsets {
            for part in &set.parts {
                for name in &part.names {
                    if name.name.starts_with("@") {
                        graph.add_edge(sub_nodes[&set.name], sub_nodes[&name.name[1..].to_string()], 1);
                    }
                }
            }
        }

        if is_cyclic_directed(&graph) {
            return Err(format!("[{}] Subsets create a cycle", townname.name))
        }

        /* Find the order the subsets needs to be generated in. */
        let mut subsets_order = Vec::new();
        let mut dfs = Dfs::new(&graph, main_node);
        while let Some(visited) = dfs.next(&graph) {
            subsets_order.push(graph.node_weight(visited).unwrap());
        }

        /* Generate the subsets in reversed order. */
        subsets_order.reverse();

        let mut subparts_ids = HashMap::new();
        let mut firstbit = 0;

        /* Generate all the subsets. */
        for set in subsets_order {
            let mut parts: Vec<ActionFPart> = Vec::new();

            let name = match set {
                Some(set) => {
                    for part in &set.parts {
                        parts.push(match create_part(&part, &mut firstbit, &subparts_ids) {
                            Err(err) => return Err(format!("[{}:{}] {}", townname.name, set.name, err)),
                            Ok(part) => part,
                        });
                    }

                    subparts_ids.insert(set.name.clone(), id);

                    None
                }
                /* None means it is the mainset. */
                None => {
                    for part in &townname.mainset {
                        parts.push(match create_part(&part, &mut firstbit, &subparts_ids) {
                            Err(err) => return Err(format!("[{}] {}", townname.name, err)),
                            Ok(part) => part,
                        });
                    }

                    Some(&townname.name)
                }
            };

            ActionF::Style { id: id, name: name, parts: &parts }.write(output);
            id += 1;
        }
    }

    Ok(())
}
