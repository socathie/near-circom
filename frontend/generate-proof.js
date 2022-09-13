/* global BigInt */

import { generateWitness } from './circuit_js/generate_witness';
import { groth16 } from 'snarkjs';

import { F1Field, Scalar} from  "ffjavascript"; 
const Fr = new F1Field(Scalar.fromString("21888242871839275222246405745257275088548364400416034343698204186575808495617"));

export async function generateCalldata(input) {

    let generateWitnessSuccess = true;

    let formattedInput = {};
    
    for (var key in input) {
        formattedInput[key] = Fr.e(input[key]);
    }

    let witness = await generateWitness(formattedInput).then()
        .catch((error) => {
            console.error(error);
            generateWitnessSuccess = false;
        });
    
    //console.log(witness);

    if (!generateWitnessSuccess) { return; }

    const { proof, publicSignals } = await groth16.prove('/circuit_final.zkey', witness);

    return {
        proof: JSON.stringify(proof),
        inputs: JSON.stringify(publicSignals)
    }
}