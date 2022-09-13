/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use electron_rs::verifier::near::{
    get_prepared_verifying_key, parse_verification_key, verify_proof,
};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

// Define the default message
const DEFAULT_MESSAGE: &str = "Hello";

const VKEY_STR: &str = r#"
{
 "protocol": "groth16",
 "curve": "bn128",
 "nPublic": 1,
 "vk_alpha_1": [
  "20491192805390485299153009773594534940189261866228447918068658471970481763042",
  "9383485363053290200918347156157836566562967994039712273449902621266178545958",
  "1"
 ],
 "vk_beta_2": [
  [
   "6375614351688725206403948262868962793625744043794305715222011528459656738731",
   "4252822878758300859123897981450591353533073413197771768651442665752259397132"
  ],
  [
   "10505242626370262277552901082094356697409835680220590971873171140371331206856",
   "21847035105528745403288232691147584728191162732299865338377159692350059136679"
  ],
  [
   "1",
   "0"
  ]
 ],
 "vk_gamma_2": [
  [
   "10857046999023057135944570762232829481370756359578518086990519993285655852781",
   "11559732032986387107991004021392285783925812861821192530917403151452391805634"
  ],
  [
   "8495653923123431417604973247489272438418190587263600148770280649306958101930",
   "4082367875863433681332203403145435568316851327593401208105741076214120093531"
  ],
  [
   "1",
   "0"
  ]
 ],
 "vk_delta_2": [
  [
   "14432025526384517745806574175009480084863780194247978511417681951320978999649",
   "10255229232152850527995321518900764479916054723165184651676449117406735188740"
  ],
  [
   "15059404967031136599919939480483317322531172278416941298465462639772497119214",
   "8954040702113710687334940106457500786842134867881833059219848195594959173390"
  ],
  [
   "1",
   "0"
  ]
 ],
 "vk_alphabeta_12": [
  [
   [
    "2029413683389138792403550203267699914886160938906632433982220835551125967885",
    "21072700047562757817161031222997517981543347628379360635925549008442030252106"
   ],
   [
    "5940354580057074848093997050200682056184807770593307860589430076672439820312",
    "12156638873931618554171829126792193045421052652279363021382169897324752428276"
   ],
   [
    "7898200236362823042373859371574133993780991612861777490112507062703164551277",
    "7074218545237549455313236346927434013100842096812539264420499035217050630853"
   ]
  ],
  [
   [
    "7077479683546002997211712695946002074877511277312570035766170199895071832130",
    "10093483419865920389913245021038182291233451549023025229112148274109565435465"
   ],
   [
    "4595479056700221319381530156280926371456704509942304414423590385166031118820",
    "19831328484489333784475432780421641293929726139240675179672856274388269393268"
   ],
   [
    "11934129596455521040620786944827826205713621633706285934057045369193958244500",
    "8037395052364110730298837004334506829870972346962140206007064471173334027475"
   ]
  ]
 ],
 "IC": [
  [
   "6819801395408938350212900248749732364821477541620635511814266536599629892365",
   "9092252330033992554755034971584864587974280972948086568597554018278609861372",
   "1"
  ],
  [
   "17882351432929302592725330552407222299541667716607588771282887857165175611387",
   "18907419617206324833977586007131055763810739835484972981819026406579664278293",
   "1"
  ]
 ]
}
"#;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    message: String,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            message: DEFAULT_MESSAGE.to_string(),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Public method - returns the greeting saved, defaulting to DEFAULT_MESSAGE
    pub fn get_greeting(&self) -> String {
        return self.message.clone();
    }

    // Public method - accepts a greeting, such as "howdy", and records it
    pub fn set_greeting(&mut self, message: String) {
        // Use env::log to record logs permanently to the blockchain!
        log!("Saving greeting {}", message);
        self.message = message;
    }

    pub fn verify_proof_on_chain(&self, proof: String, inputs: String) -> bool {
        let vkey = parse_verification_key(VKEY_STR.to_string()).unwrap();
        let prepared_vkey = get_prepared_verifying_key(vkey);

        verify_proof(prepared_vkey, proof, inputs).unwrap()
    }

    pub fn set_verified_greeting(
        &mut self,
        message: String,
        proof: String,
        inputs: String,
    ) {
        assert!(self.verify_proof_on_chain(proof, inputs), "invalid proof");
        log!("Verified and saving greeting {}", message);
        self.message = message;
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn get_default_greeting() {
        let contract = Contract::default();
        // this test did not call set_greeting so should return the default "Hello" greeting
        assert_eq!(contract.get_greeting(), "Hello".to_string());
    }

    #[test]
    fn set_then_get_greeting() {
        let mut contract = Contract::default();
        contract.set_greeting("howdy".to_string());

        assert_eq!(contract.get_greeting(), "howdy".to_string());
    }

    #[test]

    fn proof_verification() {
        let proof_str = read_to_string("circuits/proof.json").unwrap();

        let pub_input_str = read_to_string("circuits/public.json").unwrap();

        let contract = Contract::default();
        let res = contract.verify_proof_on_chain(proof_str, pub_input_str);

        assert!(res);
    }

    #[test]
    fn invalid_verification() {
        let proof_str = read_to_string("circuits/proof.json").unwrap();

        let pub_input_str = r#"
        [
            "0"
        ]
        "#;

        let contract = Contract::default();
        let res = contract.verify_proof_on_chain(proof_str, pub_input_str.to_string());

        assert!(!res);
    }

    #[test]
    fn verified_set_then_get_greeting() {
        let proof_str = read_to_string("circuits/proof.json").unwrap();

        let pub_input_str = read_to_string("circuits/public.json").unwrap();

        let mut contract = Contract::default();
        contract.set_verified_greeting("howdy".to_string(), proof_str, pub_input_str);

        assert_eq!(contract.get_greeting(), "howdy".to_string());
    }

    #[test]
    #[should_panic(expected = "invalid proof")]
    fn invalid_set_then_get_greeting() {
        let proof_str = read_to_string("circuits/proof.json").unwrap();
        
        let pub_input_str = r#"
        [
            "0"
        ]
        "#;

        let mut contract = Contract::default();
        contract.set_verified_greeting("howdy".to_string(), proof_str, pub_input_str.to_string());

        assert_eq!(contract.get_greeting(), "hello".to_string());
    }
}
