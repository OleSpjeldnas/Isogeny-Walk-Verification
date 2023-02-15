
use math::{fields::f128ext::BaseElement, FieldElement, log2, StarkField };
use winterfell::{StarkProof, Prover, ProofOptions, HashFunction, FieldExtension, VerifierError,
Trace};
use prover::IsogenyProver;
use examples::{Example};
use std::time::Instant;

use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};
use log::debug;
pub mod air;
use air::IsogenyAir;
pub mod prover;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<(u128,u128)>> {
    BufReader::new(fs::File::open(filename)?).lines()
    .map(|line| {
        let line = line?;
        let mut parts = line.trim().split(",");
        let a = parts.next().unwrap().parse::<u128>().unwrap();
        let b = parts.next().unwrap().parse::<u128>().unwrap();
        Ok((a,b))
    })
    .collect()
}

fn main() -> std::io::Result<()> {
    env_logger::Builder::new()
    .format(|buf, record| writeln!(buf, "{}", record.args()))
    .filter_level(log::LevelFilter::Debug)
    .init();
    let lines = lines_from_file("j_invariants.txt").expect("Could not load lines");
    let roots: Vec<BaseElement> = lines.iter().map(|(a,b)| BaseElement::new(*a, *b)).collect();
    let path = "benchmarks.txt"; 
        let mut benchmark_str = "".to_owned();
    for i in 4..20 {
        println!("{}", 2usize.pow(i));

        let mut prover_i: Vec<usize> = Vec::new();
        let mut verifier_i: Vec<usize> = Vec::new();
        let mut size_i: Vec<usize> = Vec::new();
        for _ in 0..2 {
        let benchmark_i = prove_and_verify(&roots[..2usize.pow(i)].try_into().unwrap());
        prover_i.push(benchmark_i[0]);
        verifier_i.push(benchmark_i[2]);
        size_i.push(benchmark_i[1]);
        }
        benchmark_str.push_str("i: ");
        benchmark_str.push_str(&i.to_string());
        benchmark_str.push_str(" Prover_Time: ");
        benchmark_str.push_str(&(prover_i.iter().sum::<usize>()/prover_i.len()).to_string());
        benchmark_str.push_str(" Verifier_Time: ");
        benchmark_str.push_str(&(verifier_i.iter().sum::<usize>()/verifier_i.len()).to_string());
        benchmark_str.push_str(" Proof_Size: ");
        benchmark_str.push_str(&(size_i.iter().sum::<usize>()/size_i.len()).to_string());
        benchmark_str.push_str("\n");
    }
    fs::write(path, benchmark_str)?;
Ok(())
}

fn prove_and_verify(roots: &Vec<BaseElement>) -> [usize; 3]{
    let phi = roots.to_vec();
    let options = ProofOptions::new(
        32,
        16,
        0,
        HashFunction::Blake3_256,
        FieldExtension::None,
        8,
        256
    );
    let now = Instant::now();
    let gen_isogeny_walk_proof = build_proof(options,phi);
    let isogeny_walk_proof = gen_isogeny_walk_proof.prove();
    let proof_gen_time = now.elapsed().as_millis().try_into().unwrap();
    let proof_bytes = isogeny_walk_proof.to_bytes();
    let proof_length = proof_bytes.len();
    let parsed_proof = StarkProof::from_bytes(&proof_bytes).unwrap();
    assert_eq!(isogeny_walk_proof, parsed_proof);
    let mut verification_time: usize = 0;
    let now = Instant::now();
    match gen_isogeny_walk_proof.verify(isogeny_walk_proof) {
        Ok(_) => {
            verification_time = now.elapsed().as_millis().try_into().unwrap();
            debug!(
            "Proof verified in {:.1} ms",
            verification_time
        )},
        Err(msg) => debug!("Failed to verify proof: {}", msg),
    }
    [proof_gen_time, proof_length, verification_time]
}
pub fn build_proof(options: ProofOptions, result: Vec<BaseElement>) -> Box<dyn Example> {
    Box::new(IsogenyWalkProof::new(
        options,
        result
    ))
}

pub struct IsogenyWalkProof {
    options: ProofOptions,
    result: (Vec<BaseElement>, Vec<BaseElement>)
}
impl IsogenyWalkProof {
    pub fn new(options: ProofOptions, root_arr: Vec<BaseElement>) -> IsogenyWalkProof {
       
        let phi = root_arr.clone();
        let psi = create_psi(&root_arr);
        IsogenyWalkProof{
            options,
            result: (phi, psi),
        }
    }
}
impl Example for IsogenyWalkProof {
    fn prove(&self) -> StarkProof {
        // create a prover
        let prover = IsogenyProver::new(self.options.clone());

        // generate execution trace
        let now = Instant::now();
        let (phi, psi) = &self.result;
        let trace = IsogenyProver::build_trace(phi.to_vec(), psi.to_vec()); 
    
        let trace_width = trace.width();
        let trace_length = trace.length();
        debug!(
            "Generated execution trace of {} registers and 2^{} steps in {} ms",
            trace_width,
            log2(trace_length),
            now.elapsed().as_millis()
        );

        // generate the proof
        prover.prove(trace).unwrap()
    }

    fn verify(&self, proof: StarkProof) -> Result<(), VerifierError> {
        winterfell::verify::<IsogenyAir>(proof, self.result.0[0])
    }

    fn verify_with_wrong_inputs(&self, proof: StarkProof) -> Result<(), VerifierError> {
        winterfell::verify::<IsogenyAir>(proof, self.result.0[0]+BaseElement::ONE)
    }
}

// Functions to generate Psi
fn create_psi(root_arr: &Vec<BaseElement>) -> Vec<BaseElement> {
    let mut psi: Vec<BaseElement> = Vec::new();
    let phii: Vec<BaseElement> = create_phiii(root_arr);
    let l = root_arr.len();
    for i in 0..l-2 {
        let psi_el: BaseElement = (root_arr[i]-phii[i]).inv();
        psi.push(psi_el);
        //println!("{}: {}",i, psi_el);}
    }
        psi.push((root_arr[l-2]-root_arr[l-3]).inv());
        psi.push((root_arr[l-2]-root_arr[l-3]).inv());
        //println!("Root: {}", psi[l-1]);
    psi
}
fn create_phiii(root_arr: &Vec<BaseElement>) -> Vec<BaseElement> {
    let mut phiii: Vec<BaseElement> = root_arr[2..].to_vec();
    let l = root_arr.len();
    phiii.push(BaseElement::ZERO);
    phiii.push(BaseElement::ZERO);
    phiii
}
