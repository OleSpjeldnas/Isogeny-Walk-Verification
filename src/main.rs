
use math::{fields::f128ext::BaseElement, FieldElement, log2, StarkField };
use winterfell::{StarkProof, Prover, ProofOptions, HashFunction, FieldExtension, VerifierError,
Trace};
use prover::IsogenyProver;
use examples::{Example};
use std::time::Instant;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};
use log::debug;
pub mod air;
use air::IsogenyAir;
pub mod prover;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<(u128,u128)>> {
    BufReader::new(File::open(filename)?).lines()
    .map(|line| {
        let line = line?;
        //println!("{}",line);
        let mut parts = line.trim().split(",");
        //println!("{}", parts.next().unwrap().parse::<u128>().unwrap());
        //println!("{}", parts.next().unwrap());
        let a = parts.next().unwrap().parse::<u128>().unwrap();
        let b = parts.next().unwrap().parse::<u128>().unwrap();
        //println!("yes");
        Ok((a,b))
    })
    .collect()
}
fn main() {
    // First get the points on the recorded isogeny walk, and define the 
    // polynomial psi
    let lines = lines_from_file("small_roots.txt").expect("Could not load lines");
    let roots: Vec<BaseElement> = lines.iter().map(|(a,b)| BaseElement::new(*a, *b)).collect();
    let _psi = create_psi(&roots);
    let phii = create_phii(&roots);
    //println!("phi: {}, phii: {}, psi: {}", roots[0],phii[0],_psi[0]);

    //let trace = IsogenyProver::build_trace(roots, _psi); 
    //let t = trace.main_segment();
    //println!("Phi: {}, Phii: {},", mod_poly(t.get(0, 0),t.get(1, 0)),t.get(2, 0)*(t.get(0,0)-t.get(1,1)));
    //let secret_inputs = (roots, _psi);
    env_logger::Builder::new()
    .format(|buf, record| writeln!(buf, "{}", record.args()))
    .filter_level(log::LevelFilter::Debug)
    .init();
    let options = ProofOptions::new(
        28,
        8,
        16,
        HashFunction::Blake3_256,
        FieldExtension::None,
        8,
        256
    );
    let now = Instant::now();
    let gen_isogeny_walk_proof = get_example(options,roots);
    let isogeny_walk_proof = gen_isogeny_walk_proof.prove();
    println!("Here now");
    //let proof_gen_time = now.elapsed().as_millis();
    let proof_bytes = isogeny_walk_proof.to_bytes();
    //let proof_size = proof_bytes.len();
    let parsed_proof = StarkProof::from_bytes(&proof_bytes).unwrap();
    assert_eq!(isogeny_walk_proof, parsed_proof);
    //println!("now here");
    let now = Instant::now();
    match gen_isogeny_walk_proof.verify(isogeny_walk_proof) {
        Ok(_) => debug!(
            "Proof verified in {:.1} ms",
            now.elapsed().as_micros() as f64 / 1000f64
        ),
        Err(msg) => debug!("Failed to verify proof: {}", msg),
    }
}

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
fn create_phii(root_arr: &Vec<BaseElement>) -> Vec<BaseElement> {
    let mut phii: Vec<BaseElement> = root_arr[1..].to_vec();
    let l = root_arr.len();
    phii.push(root_arr[l-2]);
    phii
}
fn create_phiii(root_arr: &Vec<BaseElement>) -> Vec<BaseElement> {
    let mut phiii: Vec<BaseElement> = root_arr[2..].to_vec();
    let l = root_arr.len();
    phiii.push(BaseElement::ZERO);
    phiii.push(BaseElement::ZERO);
    phiii
}

pub fn get_example(options: ProofOptions, result: Vec<BaseElement>) -> Box<dyn Example> {
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
        //println!("Verifying proof...");


        IsogenyWalkProof{
            options,
            result: (phi, psi),
        }
    }
}
impl Example for IsogenyWalkProof {
    fn prove(&self) -> StarkProof {
        //let result = &self.result;
       
        // create a prover
        let prover = IsogenyProver::new(self.options.clone());

        // generate execution trace
        let now = Instant::now();
        let (phi, psi) = &self.result;
        //let trace = IsogenyProver::build_trace(phi.to_vec(), psi.to_vec()); 
        //let t = trace.main_segment();
        
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
fn mod_poly<E: FieldElement + From<BaseElement>>(x: E, y: E) -> E {
    x*x*x+y*y*y-x*x*y*y+E::from(1488u128)
    *(x*x*y+y*y*x)-E::from(162000u128)*
    (x*x+y*y)+E::from(40773375u128)*x*y
    +E::from(8748000000u128)*(x+y)-
    E::from(157464000000000u128)
}