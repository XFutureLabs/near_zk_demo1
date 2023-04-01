build: update recover

update:
	circom update.circom --r1cs --wasm --sym 
	snarkjs groth16 setup update.r1cs pot14_final.ptau update_0000.zkey
	snarkjs zkey export verificationkey update_0000.zkey update_verification_key.json

	node update_js/generate_witness.js update_js/update.wasm update_input.json update_witness.wtns
	snarkjs groth16 prove update_0000.zkey update_witness.wtns update_proof.json update_public.json

recover:
	circom recover.circom --r1cs --wasm --sym 
	snarkjs groth16 setup recover.r1cs pot14_final.ptau recover_0000.zkey
	snarkjs zkey export verificationkey recover_0000.zkey recover_verification_key.json

	node recover_js/generate_witness.js recover_js/recover.wasm recover_input.json recover_witness.wtns
	snarkjs groth16 prove recover_0000.zkey recover_witness.wtns recover_proof.json recover_public.json
