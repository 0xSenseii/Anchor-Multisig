import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { AnchorMultisig } from '../target/types/anchor_multisig';

describe('Anchor-Multisig', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.AnchorMultisig as Program<AnchorMultisig>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
