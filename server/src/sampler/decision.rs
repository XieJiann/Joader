use super::sampler_node::NodeRef;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

#[derive(Clone)]
pub struct Decision {
    node: NodeRef,
    job_ids: HashSet<u64>,
    compensation: HashSet<u64>,
    item: u32,
}

impl Hash for Decision {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        self.node.get_job_id().hasher();
    }
}

impl PartialEq for Decision {
    fn eq(&self, other: &Self) -> bool {
        self.node.get_job_id() == other.node.get_job_id()
    }
}

impl Eq for Decision {}

impl Decision {
    pub fn new(node: NodeRef, job_ids: HashSet<u64>) -> Self {
        Self {
            node,
            job_ids,
            compensation: HashSet::new(),
            item: 0,
        }
    }

    pub fn execute(&mut self, mask: &HashSet<u64>) -> u32 {
        let mut_ref = self.node.get_mut_unchecked();
        self.job_ids = HashSet::from_iter(self.job_ids.difference(mask).cloned());
        let (ret, comp) = mut_ref.random_choose(&self.job_ids);
        self.compensation = comp;
        self.item = ret;
        ret
    }

    pub fn complent(&mut self, mut root: NodeRef) -> bool {
        if self.compensation.is_empty() {
            return false;
        }
        root
            .get_mut_unchecked()
            .complent(&mut self.compensation, self.item)
    }

    pub fn get_jobs(&self) -> HashSet<u64> {
        self.job_ids.clone()
    }
}
