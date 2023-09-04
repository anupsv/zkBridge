use clap::Parser;
#[allow(unused_imports)]
use halo2_base::{
    QuantumCell::{Constant, Existing, Witness},
};
use halo2_base::{gates::GateChip, utils::ScalarField, AssignedValue, Context};
use halo2_scaffold::scaffold::{cmd::Cli, run};
use poseidon::PoseidonChip;
use ethers_core::utils::keccak256;
use serde::{Deserialize, Serialize};

const T: usize = 2;
const RATE: usize = 1;
const R_F: usize = 8;
const R_P: usize = 57;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInput {
    pub x: String, // field element, but easier to deserialize as a string
    pub keccak_hash: Vec<u8>, // field element, but easier to deserialize as a string
    pub poseidon_hash: String, // field element, but easier to deserialize as a string
}

// this algorithm takes a public input x, computes x^2 + 72, and outputs the result as public output
fn some_algorithm_in_zk<F: ScalarField>(
    ctx: &mut Context<F>,
    inp: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>,
) {
    // `Context` can roughly be thought of as a single-threaded execution trace of a program we want to ZK prove. We do some post-processing on `Context` to optimally divide the execution trace into multiple columns in a PLONKish arithmetization
    // More advanced usage with multi-threaded witness generation is possible, but we do not explain it here

    // first we load a private input `x` (let's not worry about public inputs for now)
    let hash_input = ctx.load_witness(F::from_str_vartime(&inp.x).unwrap());
    let poseidon_hash_input = ctx.load_witness(F::from_str_vartime(&inp.poseidon_hash).unwrap());
    let keccak_loaded = ctx.assign_witnesses(inp.keccak_hash.iter().map(|each| F::from(*each as u64)));
    make_public.extend([poseidon_hash_input]);

    // create a Gate chip that contains methods for basic arithmetic operations
    let gate = GateChip::<F>::default();
    let mut poseidon = PoseidonChip::<F, T, RATE>::new(ctx, R_F, R_P).unwrap();
    poseidon.update(&[hash_input]);
    let hash = poseidon.squeeze(ctx, &gate).unwrap();
    make_public.push(hash);
    assert_eq!(hash.value(), poseidon_hash_input.value());
    // println!("x: {:?}, poseidon(x): {:?}", poseidon_hash_input.value(), hash.value());

    let out_expected = keccak256(inp.x.as_bytes());

    for (b1, b2) in keccak_loaded.into_iter().zip(out_expected) {
        assert_eq!(b1.value().get_lower_32(), b2 as u32);
        // print!("{:02x}", b2);
    }
}

fn main() {
    env_logger::init();

    let args = Cli::parse();

    // run different zk commands based on the command line arguments
    run(some_algorithm_in_zk, args);
}
