use std::collections::{HashMap, VecDeque};

use tucant::models::ModuleMenuPathPart;

pub fn calculate_paths(path_to_root: Vec<ModuleMenuPathPart>) -> Vec<VecDeque<ModuleMenuPathPart>> {
    let leaves = path_to_root.iter().take_while(|v| v.leaf);

    let nonleaves = path_to_root
        .iter()
        .rev()
        .take_while(|v| !v.leaf)
        .map(|v| (&v.tucan_id, v))
        .collect::<HashMap<_, _>>();

    leaves
        .map(|l| {
            let mut current = Some(&l);
            let mut path = VecDeque::new();
            while let Some(curr) = current {
                path.push_front(curr.to_owned().clone());
                if let Some(parent) = &curr.parent {
                    current = nonleaves.get(&parent);
                } else {
                    break;
                }
            }
            path
        })
        .collect::<Vec<_>>()
}
