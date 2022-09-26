use super::{BaseElement, IsogenyAir, ProofOptions, StarkField};
use winterfell::{Prover, TraceTable};
// FIBONACCI PROVER
// ================================================================================================

pub struct IsogenyProver {
    options: ProofOptions,
}

impl IsogenyProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }

    /// Builds an execution trace for computing a Fibonacci sequence of the specified length such
    /// that each row advances the sequence by 8 terms.
    pub fn build_trace(
        phi: Vec<BaseElement>,
        psi: Vec<BaseElement>
    ) -> TraceTable<BaseElement> {
        let mut phii: Vec<BaseElement> = phi[1..].to_vec();
        phii.push(phi[phi.len()-3]);
        TraceTable::init(vec![phi, phii, psi])
    }
}

impl Prover for IsogenyProver {
    type BaseField = BaseElement;
    type Air = IsogenyAir;
    type Trace = TraceTable<BaseElement>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> BaseElement {
        trace.get(0, 0)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}