use winterfell::ProofOptions;
//use winterfell::examples::;\
use winterfell::{
    Air, AirContext, Assertion, EvaluationFrame,TraceInfo,
    TransitionConstraintDegree
};
use super::{BaseElement, FieldElement, StarkField};
// ISOGENY WALK AIR
// The second modular polynomial Phi_2
fn mod_poly<E: FieldElement + From<BaseElement>>(x: E, y: E) -> E {
    x*x*x+y*y*y-x*x*y*y+E::from(1488u128)
    *(x*x*y+y*y*x)-E::from(162000u128)*
    (x*x+y*y)+E::from(40773375u128)*x*y
    +E::from(8748000000u128)*(x+y)-
    E::from(157464000000000u128)
}

pub struct IsogenyAir {
    context: AirContext<BaseElement>,
    seed: BaseElement
}

impl Air for IsogenyAir {
    type BaseField = BaseElement;
    type PublicInputs = BaseElement;

    // CONSTRUCTOR
    // --------------------------------------------------------------------------------------------
    fn new(trace_info: TraceInfo, pub_inputs: Self::PublicInputs, options: ProofOptions) -> Self {
        let degrees = vec![
            TransitionConstraintDegree::new(4),
            TransitionConstraintDegree::new(2)
        ];
        assert_eq!(3, trace_info.width());
        IsogenyAir {
            context: AirContext::new(trace_info, degrees, 1, options).set_num_transition_exemptions(1),
            seed: pub_inputs
        }
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }
// Enforces transition constraints defined by the polynomial Phi_2 as well as Psi
    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let current = frame.current();
        let next = frame.next();
        
        // expected state width is 3 field elements: [phi(i), phi(i+1), psi(i)]
        debug_assert_eq!(3, current.len());
        //println!("Wrong");
        //debug_assert_eq!(current, next);
        //println!("Wrong");
        debug_assert_eq!(3, next.len());
        //result[0] = mod_poly(current[0], current[1]);
        //println!("phi: {:?}, phii: {:?}, psi: {:?}", current[0], next[1], current[2]);
        //println!("mod_constraint: {:?}", mod_poly(current[0],current[1]));
        result[1] = current[2]*(current[0]-next[1])-E::from(1u128);
        //println!("{}",result[1]);
    }
// Boundary constraint: register 0 should have j_0 as its initial value
    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        vec![
            Assertion::single(0, 0, self.seed),
        ]
    }

}