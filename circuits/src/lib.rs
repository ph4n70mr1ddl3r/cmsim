use ark_bn254::Fr;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_crypto_primitives::sponge::poseidon::PoseidonConfig;
use ark_crypto_primitives::sponge::poseidon::constraints::PoseidonSpongeVar;
use ark_crypto_primitives::sponge::constraints::CryptographicSpongeVar;
use zksim_core::merkle_tree::MerkleTree;

#[derive(Clone)]
pub struct MembershipCircuit {
    // Public inputs
    pub root: Fr,
    pub nullifier_hash: Fr,
    pub external_nullifier: Fr,

    // Private inputs
    pub secret: Fr,
    pub path_elements: Vec<Fr>,
    pub path_indices: Vec<bool>,
    
    // Config
    pub poseidon_config: PoseidonConfig<Fr>,
}

impl ConstraintSynthesizer<Fr> for MembershipCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // 1. Allocate Public Inputs
        let root_var = FpVar::new_input(cs.clone(), || Ok(self.root))?;
        let nullifier_hash_var = FpVar::new_input(cs.clone(), || Ok(self.nullifier_hash))?;
        let external_nullifier_var = FpVar::new_input(cs.clone(), || Ok(self.external_nullifier))?;

        // 2. Allocate Private Inputs
        let secret_var = FpVar::new_witness(cs.clone(), || Ok(self.secret))?;
        
        let mut path_elements_vars = Vec::new();
        for elem in self.path_elements {
            path_elements_vars.push(FpVar::new_witness(cs.clone(), || Ok(elem))?);
        }

        let mut path_indices_vars = Vec::new();
        for idx in self.path_indices {
            path_indices_vars.push(Boolean::new_witness(cs.clone(), || Ok(idx))?);
        }

        // 3. Derive Address = Poseidon(Secret)
        let mut sponge = PoseidonSpongeVar::new(cs.clone(), &self.poseidon_config);
        sponge.absorb(&vec![secret_var.clone()])?;
        let address_var = sponge.squeeze_field_elements(1)?[0].clone();

        // 4. Verify Merkle Path
        // Current hash starts at leaf (address)
        let mut current_hash = address_var;

        for (sibling, is_right) in path_elements_vars.into_iter().zip(path_indices_vars.into_iter()) {
            let mut sponge = PoseidonSpongeVar::new(cs.clone(), &self.poseidon_config);
            
            // If is_right is true, then current is left, sibling is right?
            // No, path_indices usually: 0 = left child, 1 = right child.
            // If I am left child (index 0), then sibling is right. Hash(me, sibling).
            // If I am right child (index 1), then sibling is left. Hash(sibling, me).
            
            // Conditionally select left and right
            // left = is_right ? sibling : current
            // right = is_right ? current : sibling
            
            let left = FpVar::conditionally_select(&is_right, &sibling, &current_hash)?;
            let right = FpVar::conditionally_select(&is_right, &current_hash, &sibling)?;

            sponge.absorb(&vec![left, right])?;
            current_hash = sponge.squeeze_field_elements(1)?[0].clone();
        }

        // Enforce Root Equality
        current_hash.enforce_equal(&root_var)?;

        // 5. Calculate Nullifier = Poseidon(Secret, ExternalNullifier)
        let mut sponge = PoseidonSpongeVar::new(cs.clone(), &self.poseidon_config);
        sponge.absorb(&vec![secret_var, external_nullifier_var])?;
        let calculated_nullifier = sponge.squeeze_field_elements(1)?[0].clone();

        // Enforce Nullifier Equality
        calculated_nullifier.enforce_equal(&nullifier_hash_var)?;

        Ok(())
    }
}
