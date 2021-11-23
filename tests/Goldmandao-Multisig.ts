import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { GoldmandaoMultisig } from '../target/types/goldmandao_multisig';
import assert from "assert";

const { SystemProgram } = anchor.web3;

describe('GoldmanDAO-Multisig', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);


  it('Is initialized!', async () => {
    const program = anchor.workspace.GoldmandaoMultisig as Program<GoldmandaoMultisig>;
    const myAccount = anchor.web3.Keypair.generate();
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
    const account = await program.account.myAccount.fetch(myAccount.publicKey);
    assert.ok(account.data.eq(new anchor.BN(1234)));
  });
});
