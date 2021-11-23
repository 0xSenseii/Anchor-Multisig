const anchor = require('@project-serum/anchor');

// Configure the local cluster.
anchor.setProvider(anchor.Provider.local());

async function main() {
    // #region main
    // Read the generated IDL.
    const idl = JSON.parse(require('fs').readFileSync('./target/idl/goldmandao_multisig.json', 'utf8'));

    // Address of the deployed program.
    // Program ID is given at deploy time
    const programId = new anchor.web3.PublicKey('D3ogHpj4ZXexUjWCo2Y1PKZsKVHPY7x4whCCCyq5kHsx');

    // Generate the program client from IDL.
    const program = new anchor.Program(idl, programId);

    // Execute the RPC.
    await program.rpc.initialize();
    // #endregion main
}

console.log('Running client.');
main().then(() => console.log('Success'));
