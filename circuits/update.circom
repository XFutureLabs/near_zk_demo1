pragma circom 2.0.0;

include "../node_modules/circomlib/circuits/poseidon.circom";
include "./tree.circom";

template Main(nLevels) {
    signal input pathIndices[nLevels];
    signal input siblings[nLevels];

    signal input root;
    signal input oldQuestion;
    signal input oldAnswer;

    signal input newQuestion;
    signal input newAnswer;
    signal output leaf;


    component oldLeaf = Poseidon(2);
    oldLeaf.inputs[0] <== oldQuestion;
    oldLeaf.inputs[1] <== oldAnswer;

    component verifyOldRoot = SecretProtectionTree(nLevels);
    verifyOldRoot.leaf <== oldLeaf.out;
    for (var i = 0; i < nLevels; i++) {
        verifyOldRoot.siblings[i] <== siblings[i];
        verifyOldRoot.pathIndices[i] <== pathIndices[i];
    }
    root === verifyOldRoot.root;

    component newLeaf = Poseidon(2);
    newLeaf.inputs[0] <== newQuestion;
    newLeaf.inputs[1] <== newAnswer;

    leaf <== newLeaf.out;
}

component main {public [root, oldQuestion, newQuestion]} = Main(2);