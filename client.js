const anchor = require('@project-serum/anchor');

// Configure the local cluster.
anchor.setProvider(anchor.Provider.local());

async function main() {
    // #region main
    // Read the generated IDL.
    const idl = JSON.parse(require('fs').readFileSync('./target/idl/anchor_multisig.json', 'utf8'));

    // Address of the deployed program.
    // Program ID is given at deploy time
    const programId = new anchor.web3.PublicKey('<YOUR-PROGRAM-ID>');

    // Generate the program client from IDL.
    const program = new anchor.Program(idl, programId);

    // Execute the RPC.
    await program.rpc.initialize();
    // #endregion main
}

console.log('Running client.');
main().then(() => console.log('Success'));