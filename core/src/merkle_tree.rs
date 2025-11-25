use ark_bn254::Fr;
use ark_crypto_primitives::sponge::poseidon::{PoseidonConfig, PoseidonSponge};
use ark_crypto_primitives::sponge::{CryptographicSponge, FieldBasedCryptographicSponge};

use std::collections::HashMap;

pub struct MerkleTree {
    pub depth: u32,
    pub root: Fr,
    pub leaves: HashMap<u64, Fr>, // index -> value
    pub nodes: HashMap<(u32, u64), Fr>, // (level, index) -> value
    pub default_nodes: Vec<Fr>, // default value for each level
    pub poseidon_config: PoseidonConfig<Fr>,
}

impl MerkleTree {
    pub fn new(depth: u32, poseidon_config: PoseidonConfig<Fr>) -> Self {
        let mut default_nodes = vec![Fr::from(0u64); (depth + 1) as usize];
        
        // Compute default nodes up the tree
        // Level 0 = leaves (default 0)
        // Level i = Hash(Level i-1, Level i-1)
        for i in 1..=depth {
            let child = default_nodes[(i - 1) as usize];
            default_nodes[i as usize] = Self::hash(&poseidon_config, child, child);
        }

        Self {
            depth,
            root: default_nodes[depth as usize],
            leaves: HashMap::new(),
            nodes: HashMap::new(),
            default_nodes,
            poseidon_config,
        }
    }

    pub fn insert(&mut self, index: u64, value: Fr) {
        self.leaves.insert(index, value);
        self.update_path(index, value);
    }

    fn update_path(&mut self, index: u64, value: Fr) {
        let mut current_idx = index;
        let mut current_val = value;

        // Level 0
        self.nodes.insert((0, current_idx), current_val);

        for level in 1..=self.depth {
            let sibling_idx = if current_idx % 2 == 0 { current_idx + 1 } else { current_idx - 1 };
            let sibling_val = self.get_node(level - 1, sibling_idx);

            let (left, right) = if current_idx % 2 == 0 {
                (current_val, sibling_val)
            } else {
                (sibling_val, current_val)
            };

            current_val = Self::hash(&self.poseidon_config, left, right);
            current_idx /= 2;
            self.nodes.insert((level, current_idx), current_val);
        }
        self.root = current_val;
    }

    pub fn get_node(&self, level: u32, index: u64) -> Fr {
        *self.nodes.get(&(level, index)).unwrap_or(&self.default_nodes[level as usize])
    }

    pub fn get_proof(&self, index: u64) -> (Vec<Fr>, Vec<bool>) {
        let mut path = Vec::new();
        let mut path_indices = Vec::new(); // 0 for left, 1 for right (is_right_child)
        
        let mut current_idx = index;
        for level in 0..self.depth {
            let sibling_idx = if current_idx % 2 == 0 { current_idx + 1 } else { current_idx - 1 };
            path.push(self.get_node(level, sibling_idx));
            path_indices.push(current_idx % 2 != 0);
            current_idx /= 2;
        }
        (path, path_indices)
    }

    fn hash(config: &PoseidonConfig<Fr>, left: Fr, right: Fr) -> Fr {
        let mut sponge = PoseidonSponge::new(config);
        sponge.absorb(&vec![left, right]);
        sponge.squeeze_field_elements(1)[0]
    }
}

// Helper to generate a dummy config for demo purposes
pub fn test_poseidon_config() -> PoseidonConfig<Fr> {
    // NOTE: This is NOT secure. Real params should be generated properly.
    // Using minimal params for speed and demo.
    use ark_std::UniformRand;
    
    let full_rounds = 8;
    let partial_rounds = 31;
    let alpha = 5;
    let rate = 2;
    let capacity = 1; 
    let width = rate + capacity;

    let mut rng = ark_std::test_rng();

    let ark = (0..full_rounds + partial_rounds)
        .map(|_| (0..width).map(|_| Fr::rand(&mut rng)).collect())
        .collect();
    let mds = (0..width)
        .map(|_| (0..width).map(|_| Fr::rand(&mut rng)).collect())
        .collect();

    PoseidonConfig::new(
        full_rounds,
        partial_rounds,
        alpha,
        mds,
        ark,
        rate,
        capacity,
    )
}
