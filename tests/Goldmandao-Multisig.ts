import * as anchor from '@project-serum/anchor';
import { IdlAccounts, Program } from '@project-serum/anchor';
const serumCmn = require("@project-serum/common");
import { TOKEN_PROGRAM_ID, Token } from "@solana/spl-token";

import { GoldmandaoMultisig } from '../target/types/goldmandao_multisig';
import assert from "assert";

describe('GoldmanDAO-Multisig', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.Provider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.GoldmandaoMultisig as Program<GoldmandaoMultisig>;

  // All mints default to 6 decimal places.
  const daoTokenAmount = new anchor.BN(5000000)
  const daoName = 'test_dao'

  var bumps, daoAccount, daoAccountBump, redeemableMint, redeemableMintBump

  beforeEach(async () => {
    // Bumps are used to create PDA's. Their type y just a number from 0..256
    bumps = {
      daoAccount: undefined,
      redeemableMint: undefined
    }

    // We get the account and the bump of the GoldmanDAO multisig program
    const [_daoAccount, _daoAccountBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(daoName)],
      program.programId
    )
    daoAccount = _daoAccount
    daoAccountBump = _daoAccountBump
    bumps.daoAccount = daoAccountBump

    // We get what is going to be the token of the dao.
    // It's not yet deployed, but its address and bump can be obtained nonetheless
    const [_redeemableMint, _redeemableMintBump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from(daoName), Buffer.from("redeemable_mint")],
      program.programId
    );
    redeemableMint = _redeemableMint
    redeemableMintBump = _redeemableMintBump
    bumps.redeemableMint = redeemableMintBump;
  })

  it('should initialize the dao', async () => {
    // This is the first tx
    // It creates the Accunts for:
    // - daoAccount: The program storage
    // - redeemableMint: The PDA Token
    const tx = await program.rpc.initializeDao(
      daoName,
      bumps, // They are stored so we don't need to calculate them again
      daoTokenAmount,
      {
        accounts: {
          daoAuthority: provider.wallet.publicKey, // The authority is set to our address
          daoAccount,
          redeemableMint,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID, // The Program Code (stores things in the address you send)
          rent: anchor.web3.SYSVAR_RENT_PUBKEY, // To get the current rent
        },
      }
    )
    // Having a hard time trying to test this
    // let resultAccount = await program.account.daoAccount.fetch(daoAccount.toString())
    // assert(resultAccount.numDaoTokens.eq(daoTokenAmount))
  });

  it('should mint tokens to the owner', async () => {
    const [userRedeemable] = await anchor.web3.PublicKey.findProgramAddress(
      [provider.wallet.publicKey.toBuffer(),
      Buffer.from(daoName),
      Buffer.from("user_redeemable")],
      program.programId
    );

    const mintTx = await program.rpc.mintTokens(
      {
        accounts: {
          userAuthority: provider.wallet.publicKey,
          userRedeemable,
          daoAccount,
          redeemableMint,
          tokenProgram: TOKEN_PROGRAM_ID
        },
        instructions: [
          program.instruction.initUserRedeemable({
            accounts: {
              userAuthority: provider.wallet.publicKey,
              userRedeemable,
              daoAccount,
              redeemableMint,
              systemProgram: anchor.web3.SystemProgram.programId,
              tokenProgram: TOKEN_PROGRAM_ID,
              rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            }
          })
        ]
      },
    );
    let userRedeemableAccount = await serumCmn.getTokenAccount(provider, userRedeemable);
    assert(userRedeemableAccount.amount.eq(new anchor.BN(100)))
  });
});


