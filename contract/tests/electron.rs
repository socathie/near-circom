use std::fs::read_to_string;
use color_eyre::Result;
use electron_rs::verifier::near::{verify_proof, parse_verification_key, get_prepared_verifying_key};

#[test]
fn proof_verification_electron() -> Result<()> {
    let vkey_str = read_to_string("circuits/build/verification_key.json").unwrap();

    let proof_str = read_to_string("circuits/proof.json").unwrap();

    let pub_input_str = read_to_string("circuits/public.json").unwrap();

    println!("{}", pub_input_str);

    let vkey = parse_verification_key(vkey_str).unwrap();
    let prepared_vkey = get_prepared_verifying_key(vkey);

    let res = verify_proof(prepared_vkey, proof_str, pub_input_str);

    assert!(res.unwrap());

    Ok(())
}


#[test]
fn invalid_verification_electron() -> Result<()> {
    let vkey_str = read_to_string("circuits/build/verification_key.json").unwrap();

    let proof_str = read_to_string("circuits/proof.json").unwrap();

    let pub_input_str = r#"
    [
        "0"
    ]
    "#;

    let vkey = parse_verification_key(vkey_str).unwrap();
    let prepared_vkey = get_prepared_verifying_key(vkey);

    let res = verify_proof(prepared_vkey, proof_str, pub_input_str.to_string());

    assert!(!res.unwrap());

    Ok(())
}