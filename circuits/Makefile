build: update recover

update:
	mkdir -p out
	circom update.circom --r1cs --wasm --sym -o out
	snarkjs groth16 setup out/update.r1cs pot14_final.ptau out/update_0000.zkey
	snarkjs zkey export verificationkey out/update_0000.zkey out/update_verification_key.json

	node out/update_js/generate_witness.js out/update_js/update.wasm update_input.json out/update_witness.wtns
	snarkjs groth16 prove out/update_0000.zkey out/update_witness.wtns out/update_proof.json out/update_public.json

recover:
	mkdir -p out
	circom recover.circom --r1cs --wasm --sym -o out
	snarkjs groth16 setup out/recover.r1cs pot14_final.ptau out/recover_0000.zkey
	snarkjs zkey export verificationkey out/recover_0000.zkey out/recover_verification_key.json

	node out/recover_js/generate_witness.js out/recover_js/recover.wasm recover_input.json out/recover_witness.wtns
	snarkjs groth16 prove out/recover_0000.zkey out/recover_witness.wtns out/recover_proof.json out/recover_public.json
