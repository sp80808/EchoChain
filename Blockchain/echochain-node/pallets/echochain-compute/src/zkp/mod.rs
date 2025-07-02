//! ZKP verification module

use ark_groth16::{Groth16, Proof, VerifyingKey, PreparedVerifyingKey};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_ff::{PrimeField, Field, BigInteger, Fq};
use log;

/// Represents a Zero-Knowledge Proof.
pub struct ZKP {
    /// The proof data.
    proof: Vec<u8>,
    /// Public inputs to the ZKP.
    public_inputs: Vec<u8>,
    /// Additional data.
    extra_data: Vec<u8>,
}

impl ZKP {
    /// Creates a new ZKP instance.
    pub fn new(proof: Vec<u8>, public_inputs: Vec<u8>, extra_data: Vec<u8>) -> Self {
        ZKP {
            proof,
            public_inputs,
            extra_data,
        }
    }
}

/// Verifies a ZKP.
pub fn verify_zkp(zkp: &ZKP, vk_bytes: &[u8]) -> bool {
    // Deserialize the verifying key
    let vk = match VerifyingKey::<ark_bls12_381::Bls12_381>::deserialize_compressed(&vk_bytes[..]) {
        Ok(vk) => vk,
        Err(e) => {
            log::error!("Failed to deserialize verifying key: {:?}", e);
            return false;
        }
    };

    // Deserialize the proof
    let proof = match Proof::<ark_bls12_381::Bls12_381>::deserialize_compressed(&zkp.proof[..]) {
        Ok(proof) => proof,
        Err(e) => {
            log::error!("Failed to deserialize proof: {:?}", e);
            return false;
        }
    };

    // Convert public inputs to field elements
    let public_inputs: Vec<Fq> = zkp.public_inputs
        .chunks(32) // Assuming 32 bytes per field element
        .map(|chunk| {
            let mut bytes = [0u8; 32];
            bytes.copy_from_slice(chunk);
            Fq::from_le_bytes_mod_order(&bytes)
        })
        .collect();

    // Verify the proof
    let pvk = PreparedVerifyingKey::<ark_bls12_381::Bls12_381>::from(vk);
    match Groth16::<ark_bls12_381::Bls12_381>::verify_proof(&pvk, &proof, &public_inputs) {
        Ok(verified) => verified,
        Err(e) => {
            log::error!("Failed to verify proof: {:?}", e);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
    use ark_ff::{Field, PrimeField};
    use ark_groth16::ProvingKey;
    use ark_serialize::{CanonicalSerialize};
    use ark_std::rand::thread_rng;
    use ark_bls12_381::Bls12_381;

    // Define a simple circuit for testing
    #[derive(Clone)]
    struct SimpleCircuit<F: PrimeField> {
        pub a: Option<F>,
        pub b: Option<F>,
        pub c: Option<F>,
    }

    impl<F: PrimeField> ConstraintSynthesizer<F> for SimpleCircuit<F> {
        fn generate_constraints(
            self,
            cs: ConstraintSystemRef<F>,
        ) -> Result<(), SynthesisError> {
            let a = cs.new_witness_variable(|| self.a.ok_or(SynthesisError::AssignmentMissing))?;
            let b = cs.new_witness_variable(|| self.b.ok_or(SynthesisError::AssignmentMissing))?;
            let c = cs.new_input_variable(|| self.c.ok_or(SynthesisError::AssignmentMissing))?;

            // Enforce a * b = c
            cs.enforce(
                |_| a,
                |_| b,
                |_| c,
            );

            Ok(())
        }
    }

    #[test]
    fn test_zkp_verification() {
        // Generate the parameters for the circuit
        let rng = &mut thread_rng();
        let circuit = SimpleCircuit::<ark_ff::Fq> {
            a: Some(ark_ff::Fq::from(5)),
            b: Some(ark_ff::Fq::from(10)),
            c: Some(ark_ff::Fq::from(50)),
        };
        let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), rng).unwrap();

        // Generate the proof
        let proof = Groth16::<Bls12_381>::prove(&pk, circuit, rng).unwrap();

        // Prepare the public inputs
        let c = ark_ff::Fq::from(50);
        let mut public_inputs_bytes = Vec::new();
        public_inputs_bytes.extend_from_slice(&c.into_bigint().to_bytes_le());

        // Serialize the proof and verifying key
        let mut proof_bytes = Vec::new();
        proof.serialize_compressed(&mut proof_bytes).unwrap();
        let mut vk_bytes = Vec::new();
        vk.serialize_compressed(&mut vk_bytes).unwrap();

        // Create a ZKP instance
        let zkp = ZKP::new(proof_bytes, public_inputs_bytes, Vec::new());

        // Verify the proof
        let verified = verify_zkp(&zkp, &vk_bytes);
        assert!(verified, "ZKP verification failed");
    }
}