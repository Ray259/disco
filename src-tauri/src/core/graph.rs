use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::HashMap;

use crate::core::domain::values::entity_ref::{EntityRef, EntityType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EntityKey {
    pub entity_type: EntityType,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityNode {
    pub entity_type: EntityType,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityEdge {
    pub relation_type: String,
}

pub struct EncyclopediaGraph {
    graph: DiGraph<EntityNode, EntityEdge>,
    index_map: HashMap<EntityKey, NodeIndex>,
}

impl EncyclopediaGraph {
    pub fn new() -> Self {
        Self { graph: DiGraph::new(), index_map: HashMap::new() }
    }

    pub fn add_entity(&mut self, entity_type: EntityType, name: String) -> NodeIndex {
        let key = EntityKey { entity_type, name: name.clone() };
        if let Some(&idx) = self.index_map.get(&key) { return idx; }
        let node = EntityNode { entity_type, name };
        let idx = self.graph.add_node(node);
        self.index_map.insert(key, idx);
        idx
    }

    pub fn add_relation(&mut self, from: &EntityKey, to: &EntityKey, relation_type: &str) -> bool {
        let from_idx = match self.index_map.get(from) { Some(&idx) => idx, None => return false };
        let to_idx = match self.index_map.get(to) { Some(&idx) => idx, None => return false };
        self.graph.add_edge(from_idx, to_idx, EntityEdge { relation_type: relation_type.to_string() });
        true
    }

    pub fn all_refs(&self) -> Vec<EntityRef> {
        self.graph.node_indices()
            .map(|idx| {
                let node = &self.graph[idx];
                EntityRef::new(node.entity_type, node.name.clone())
            })
            .collect()
    }

    pub fn entity_count(&self) -> usize { self.graph.node_count() }
    pub fn relation_count(&self) -> usize { self.graph.edge_count() }
}

impl Default for EncyclopediaGraph {
    fn default() -> Self { Self::new() }
}