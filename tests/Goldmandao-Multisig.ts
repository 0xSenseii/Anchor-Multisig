import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { GoldmandaoMultisig } from '../target/types/goldmandao_multisig';
const { SystemProgram } = anchor.web3;

describe('GoldmanDAO-Multisig', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.GoldmandaoMultisig as Program<GoldmandaoMultisig>;
  const myAccount = anchor.web3.Keypair.generate();
  const myAccount2 = anchor.web3.Keypair.generate();

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize(
        new anchor.BN(1234),
        {
            accounts: {
                myAccount: myAccount.publicKey,
                user: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId
            },
            signers: [myAccount],
        }
    );
    console.log("Your transaction signature", tx);

    const tx2 = await program.rpc.initialize(
        new anchor.BN(4321),
        {
            accounts: {
                myAccount: myAccount2.publicKey,
                user: provider.wallet.publicKey,
                systemProgram: SystemProgram.programId
            },
            signers: [myAccount2],
        }
    );
    console.log("Your transaction signature", tx2);
  });
});
