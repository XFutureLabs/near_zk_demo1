pragma circom 2.0.0;

include "../node_modules/circomlib/circuits/poseidon.circom";
include "./tree.circom";

template Main(nLevels) {
    signal input pathIndices[nLevels];
    signal input siblings[nLevels];

    signal input root;
    signal input question;
    signal input answer;

    signal input new_owner;

    signal output nullifier;

    component leaf = Poseidon(2);
    leaf.inputs[0] <== question;
    leaf.inputs[1] <== answer;

    component verifyRoot = SecretProtectionTree(nLevels);
    verifyRoot.leaf <== leaf.out;

    for (var i = 0; i < nLevels; i++) {
        verifyRoot.siblings[i] <== siblings[i];
        verifyRoot.pathIndices[i] <== pathIndices[i];
    }
    root === verifyRoot.root;

    component nullifierHash = Poseidon(1);
    nullifierHash.inputs[0] <== answer;

    nullifier <== nullifierHash.out;
}

component main {public [root, new_owner]} = Main(2);