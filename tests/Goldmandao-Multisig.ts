import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";

import { GoldmandaoMultisig } from '../target/types/goldmandao_multisig';
import assert from "assert";

// const { SystemProgram } = anchor.web3;

describe('GoldmanDAO-Multisig', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const signer = anchor.web3.Keypair.generate()
  const program = anchor.workspace.GoldmandaoMultisig as Program<GoldmandaoMultisig>;

  // All mints default to 6 decimal places.
  const daoTokenAmount = new anchor.BN(5000000)
  const daoName = 'test_dao'

  it('Is initialized!', async () => {
    // We give money to the "signer"
    await requestAirdrop({provider, pubKey: signer.publicKey});
    let bumps = new PoolBumps();

    // We need the program as authority so it can tranfer funds
    const [daoAccount, daoAccountBump] = await anchor.web3.PublicKey.findProgramAddress(
        [Buffer.from(daoName)],
        program.programId
    )
    bumps.daoAccount = daoAccountBump

    const [redeemableMint, redeemableMintBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(daoName), Buffer.from("redeemable_mint")],
      program.programId
    );
    bumps.redeemableMint = redeemableMintBump;

    const tx = await program.rpc.initializeDao(
      daoName,
      bumps,
      daoTokenAmount,
      {
        accounts: {
          daoAuthority: provider.wallet.publicKey,
          daoAccount,
          redeemableMint,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        },
      }
    )
    console.log("Transaction is: ", tx)
  });
  function PoolBumps() {
    this.daoAccount;
    this.redeemableMint;
  };
});


const createToken = async ({ provider, wallet, mintAuthority }) =>
  Token.createMint(
    provider.connection,
    wallet,
    mintAuthority,
    null,
    8,
    TOKEN_PROGRAM_ID
  )


const requestAirdrop = async ({ provider, pubKey }) =>
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(pubKey, 10000000000),
      "confirmed"
    );


