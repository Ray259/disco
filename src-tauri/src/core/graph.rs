use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::domain::values::entity_ref::{EntityRef, EntityType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityNode {
    pub id: Uuid,
    pub entity_type: EntityType,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityEdge {
    pub relation_type: String, // e.g., "influenced_by", "authored", "located_in"
}

pub struct EncyclopediaGraph {
    graph: DiGraph<EntityNode, EntityEdge>,
    index_map: HashMap<Uuid, NodeIndex>,
}

impl EncyclopediaGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            index_map: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, id: Uuid, entity_type: EntityType, name: String) -> NodeIndex {
        if let Some(&idx) = self.index_map.get(&id) {
            return idx;
        }
        let node = EntityNode { id, entity_type, name };
        let idx = self.graph.add_node(node);
        self.index_map.insert(id, idx);
        idx
    }

    pub fn add_relation(&mut self, from_id: Uuid, to_id: Uuid, relation_type: &str) -> bool {
        let from_idx = match self.index_map.get(&from_id) {
            Some(&idx) => idx,
            None => return false,
        };
        let to_idx = match self.index_map.get(&to_id) {
            Some(&idx) => idx,
            None => return false,
        };
        self.graph.add_edge(from_idx, to_idx, EntityEdge {
            relation_type: relation_type.to_string(),
        });
        true
    }

    pub fn get_entity(&self, id: Uuid) -> Option<&EntityNode> {
        self.index_map.get(&id).map(|&idx| &self.graph[idx])
    }

    pub fn get_relations(&self, id: Uuid, direction: Direction) -> Vec<(&EntityNode, &EntityEdge)> {
        let idx = match self.index_map.get(&id) {
            Some(&idx) => idx,
            None => return Vec::new(),
        };

        self.graph.edges_directed(idx, direction)
            .filter_map(|edge| {
                let other_idx = match direction {
                    Direction::Outgoing => edge.target(),
                    Direction::Incoming => edge.source(),
                };
                Some((&self.graph[other_idx], edge.weight()))
            })
            .collect()
    }

    // Extract all entity refs from the graph (for serialization)
    pub fn all_refs(&self) -> Vec<EntityRef> {
        self.graph.node_indices()
            .map(|idx| {
                let node = &self.graph[idx];
                EntityRef::new(node.entity_type, node.id, node.name.clone())
            })
            .collect()
    }

    pub fn entity_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn relation_count(&self) -> usize {
        self.graph.edge_count()
    }
}

impl Default for EncyclopediaGraph {
    fn default() -> Self {
        Self::new()
    }
}