use plonky2x::backend::circuit::Circuit;
use plonky2x::backend::function::VerifiableFunction;
use std::env;

use plonky2x::frontend::vars::U32Variable;
use plonky2x::prelude::{CircuitBuilder, PlonkParameters};

pub struct U32AddFunction {}

impl Circuit for U32AddFunction {
    fn define<L: PlonkParameters<D>, const D: usize>(builder: &mut CircuitBuilder<L, D>) {
        let a = builder.evm_read::<U32Variable>();
        let b = builder.evm_read::<U32Variable>();
        let c = builder.add(a, b);

        builder.evm_write(c);
    }
}

fn main() {
    VerifiableFunction::<U32AddFunction>::entrypoint();
}

#[cfg(test)]
mod tests {
    use plonky2x::prelude::DefaultParameters;

    use super::*;

    type L = DefaultParameters;
    const D: usize = 2;

    #[test]
    fn test_circuit() {
        let mut builder = CircuitBuilder::<L, D>::new();
        U32AddFunction::define(&mut builder);
        let circuit = builder.build();
        let mut input = circuit.input();
        input.evm_write::<U32Variable>(0x12345678);
        input.evm_write::<U32Variable>(0x01234567);
        let (proof, mut output) = circuit.prove(&input);
        circuit.verify(&proof, &input, &output);
        let sum = output.evm_read::<U32Variable>();
        assert_eq!(sum, 0x12345678 + 0x01234567);
    }
}
