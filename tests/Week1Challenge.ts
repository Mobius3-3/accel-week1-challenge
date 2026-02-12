import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import {
  Keypair,
  PublicKey,
  SendTransactionError,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  createInitializeTransferHookInstruction,
  createInitializeMintInstruction,
  createInitializeAccountInstruction,
  createEnableRequiredMemoTransfersInstruction,
  createSetAuthorityInstruction,
  AuthorityType,
  getAccountLen,
  getMintLen,
  ExtensionType,
  TOKEN_2022_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  createTransferCheckedWithTransferHookInstruction,
  createApproveInstruction,
  getAccount,
} from "@solana/spl-token";
import { assert } from "chai";
import { WhitelistVault } from "../target/types/whitelist_vault";
import { MEMO_PROGRAM_ID, createMemoInstruction } from "@solana/spl-memo";

describe("whitelist_vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const wallet = provider.wallet as anchor.Wallet;

  const withTxLogs = async (label: string, run: () => Promise<string>) => {
    try {
      const sig = await run();
      const tx = await provider.connection.getTransaction(sig, {
        commitment: "confirmed",
        maxSupportedTransactionVersion: 0,
      });
      console.log(`\n[${label}] tx:`, sig);
      console.log(`[${label}] logs:`, tx?.meta?.logMessages ?? []);
      return sig;
    } catch (e: any) {
      console.log(`\n[${label}] error:`, e);
      console.log(`[${label}] logs:`, (await e.getLogs?.(provider.connection)) ?? []);
      throw e;
    }
  };

  const program = anchor.workspace.whitelistVault as Program<WhitelistVault>;

  const mint2022 = anchor.web3.Keypair.generate();

  // Sender token account address
  const sourceTokenAccount = getAssociatedTokenAddressSync(
    mint2022.publicKey,
    wallet.publicKey,
    false,
    TOKEN_2022_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
  );

  // ExtraAccountMetaList address
  // Store extra accounts required by the custom transfer hook instruction
  const [extraAccountMetaListPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from('extra-account-metas'), mint2022.publicKey.toBuffer()],
    program.programId,
  );

  const source_user_pda = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("user"),
      wallet.publicKey.toBuffer(),
    ],
    program.programId
  )[0];

  const vaultConfigPda = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault-config")],
    program.programId,
  )[0];

  let vaultTokenAccount: Keypair;
  let vaultTokenSetupSig: string;
  let initVaultSig: string;

  before(async () => {
    const mintExtensions = [ExtensionType.TransferHook];
    const mintLen = getMintLen(mintExtensions);
    const mintLamports = await provider.connection.getMinimumBalanceForRentExemption(mintLen);

    const createMintTx = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: mint2022.publicKey,
        space: mintLen,
        lamports: mintLamports,
        programId: TOKEN_2022_PROGRAM_ID,
      }),
      createInitializeTransferHookInstruction(
        mint2022.publicKey,
        wallet.publicKey,
        program.programId,
        TOKEN_2022_PROGRAM_ID,
      ),
      createInitializeMintInstruction(
        mint2022.publicKey,
        9,
        wallet.publicKey,
        null,
        TOKEN_2022_PROGRAM_ID,
      ),
    );

    await sendAndConfirmTransaction(provider.connection, createMintTx, [wallet.payer, mint2022], {
      skipPreflight: true,
      commitment: "finalized",
    });

    vaultTokenAccount = Keypair.generate();
    const vaultTokenExtensions = [
      ExtensionType.TransferHookAccount,
      ExtensionType.MemoTransfer,
    ];
    const vaultTokenLen = getAccountLen(vaultTokenExtensions);
    const vaultTokenLamports = await provider.connection.getMinimumBalanceForRentExemption(vaultTokenLen);

    const createVaultTokenAccountTx = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: wallet.publicKey,
        newAccountPubkey: vaultTokenAccount.publicKey,
        space: vaultTokenLen,
        lamports: vaultTokenLamports,
        programId: TOKEN_2022_PROGRAM_ID,
      }),
      createInitializeAccountInstruction(
        vaultTokenAccount.publicKey,
        mint2022.publicKey,
        wallet.publicKey,
        TOKEN_2022_PROGRAM_ID,
      ),
      createEnableRequiredMemoTransfersInstruction(
        vaultTokenAccount.publicKey,
        wallet.publicKey,
        [],
        TOKEN_2022_PROGRAM_ID,
      ),
      createSetAuthorityInstruction(
        vaultTokenAccount.publicKey,
        wallet.publicKey,
        AuthorityType.AccountOwner,
        vaultConfigPda,
        [],
        TOKEN_2022_PROGRAM_ID,
      ),
    );

    try {
      vaultTokenSetupSig = await sendAndConfirmTransaction(
        provider.connection,
        createVaultTokenAccountTx,
        [wallet.payer, vaultTokenAccount],
        {
          skipPreflight: true,
          commitment: "finalized",
        }
      );
    } catch (error: any) {
      console.log("[vault_token_setup] error:", error);
      const failedSig = error?.signature as string | undefined;
      if (failedSig) {
        const failedTx = await provider.connection.getTransaction(failedSig, {
          commitment: "confirmed",
          maxSupportedTransactionVersion: 0,
        });
        console.log("[vault_token_setup] tx:", failedSig);
        console.log("[vault_token_setup] logs:", failedTx?.meta?.logMessages ?? []);
      }
      throw error;
    }



    console.log("Vault token account setup signature:", vaultTokenSetupSig);
    console.log("Derived vaultTaPda:", vaultTokenAccount.publicKey.toBase58());
    console.log("Program-owned vault token account:", vaultTokenAccount.publicKey.toBase58());
  });


  it("Add source user to whitelist", async () => {
    const tx = await withTxLogs("add_to_whitelist", async () =>
      program.methods.addToWhitelist(wallet.publicKey)
        .accountsPartial({
          admin: wallet.publicKey,
          user: source_user_pda,
          systemProgram: SystemProgram.programId,
        })
        .rpc()
    );

    console.log("\nUser added to whitelist:", wallet.publicKey.toBase58());
    console.log("Transaction signature:", tx);
  });

  xit("Remove source user to whitelist", async () => {
    const tx = await withTxLogs("remove_from_whitelist", async () =>
      program.methods.removeFromWhitelist(wallet.publicKey)
        .accountsPartial({
          admin: wallet.publicKey,
          user: source_user_pda,
          systemProgram: SystemProgram.programId,
        })
        .rpc()
    );

    console.log("\nUser removed from whitelist:", wallet.publicKey.toBase58());
    console.log("Transaction signature:", tx);
  });

  it("Initializes the vault config", async () => {
      initVaultSig = await withTxLogs("init_vault_config", async () =>
      program.methods
        .initVaultConfig()
        .accountsPartial({
          admin: wallet.publicKey,
          vaultConfig: vaultConfigPda,
          mint: mint2022.publicKey,
          vaultTa: vaultTokenAccount.publicKey,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc()
    );
    console.log("Init vault config transaction signature:", initVaultSig);

    // Verify vault config was created
    const vaultConfigAccount = await program.account.vaultConfig.fetch(
      vaultConfigPda
    );
    assert.equal(
      vaultConfigAccount.admin.toString(),
      wallet.publicKey.toString()
    );
    assert.equal(
      vaultConfigAccount.mint.toString(),
      mint2022.publicKey.toString()
    );
    assert.equal(vaultConfigAccount.vaultTa.toString(), vaultTokenAccount.publicKey.toString());
  });

  it("Initializes extra account meta list for transfer hook", async () => {
     const initializeExtraAccountMetaListInstruction = await program.methods
      .initTransferHook()
      .accountsPartial({
        payer: wallet.publicKey,
        mint: mint2022.publicKey,
        extraAccountMetaList: extraAccountMetaListPDA,
        systemProgram: SystemProgram.programId,
      })
      //.instruction();
      .rpc();

    //const transaction = new Transaction().add(initializeExtraAccountMetaListInstruction);

    //const txSig = await sendAndConfirmTransaction(provider.connection, transaction, [wallet.payer], { skipPreflight: true, commitment: 'confirmed' });
    console.log(
      "\nExtraAccountMetaList Account created:",
      extraAccountMetaListPDA.toBase58()
    );
    console.log(
      "Transaction Signature:",
      initializeExtraAccountMetaListInstruction
    );
  });

  it("Mints tokens to source", async () => {
    const amount = new BN(1000 * 10 ** 9); // 1000 tokens with 9 decimals

    const tx = await program.methods
      .mintToken(amount)
      .accounts({
        admin: wallet.publicKey,
        user: wallet.publicKey,
        mint: mint2022.publicKey,
        userAta: sourceTokenAccount,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Mint token transaction signature:", tx);

    const latestBlockhash = await provider.connection.getLatestBlockhash("confirmed");
    await provider.connection.confirmTransaction(
      {
        signature: tx,
        blockhash: latestBlockhash.blockhash,
        lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
      },
      "finalized"
    );

    // Verify tokens were minted
    const user1TokenAccount = await getAccount(
      provider.connection,
      sourceTokenAccount,
      "finalized",
      TOKEN_2022_PROGRAM_ID
    );
    assert.equal(user1TokenAccount.amount.toString(), amount.toString());
  });

  it("Deposits tokens from source to vault with valid memo", async () => {
    const amount = new BN(500 * 10 ** 9); // 500 tokens
    const nonce = new BN(Date.now());
    const expectedMemo = `deposit:${wallet.publicKey.toString()}:${amount.toString()}:${nonce.toString()}`;

    // Create memo instruction
    const memoInstruction = createMemoInstruction(expectedMemo, [wallet.publicKey]);

    // Create transfer instruction with transfer hook
    const transferInstruction = await createTransferCheckedWithTransferHookInstruction(
      provider.connection,
      sourceTokenAccount,
      mint2022.publicKey,
      vaultTokenAccount.publicKey,
      wallet.publicKey,
      BigInt(amount.toString()),
      9,
      [],
      "confirmed",
      TOKEN_2022_PROGRAM_ID
    );

    const tx = await withTxLogs("deposit", async () =>
      program.methods
        .deposit(amount, nonce)
        .accounts({
          signer: wallet.publicKey,
          user: source_user_pda,
          vaultConfig: vaultConfigPda,
          mint: mint2022.publicKey,
          sourceAta: sourceTokenAccount,
          vaultTa: vaultTokenAccount.publicKey,
          instructions: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .preInstructions([memoInstruction, transferInstruction])
        .signers([wallet.payer])
        .rpc()
    );

    console.log("Deposit transaction signature:", tx);

    // Verify balance was updated
    const userAccount = await program.account.user.fetch(source_user_pda);
    assert.equal(userAccount.balance.toString(), amount.toString());
  });


  it("Withdraws tokens from vault to source", async () => {
    const withdrawAmount = new BN(200 * 10 ** 9); // 200 tokens

    const userAccountBefore = await program.account.user.fetch(source_user_pda);
    const balanceBefore = userAccountBefore.balance;

    const tx = await withTxLogs("withdraw", async () =>
      program.methods
        .withdraw(withdrawAmount)
        .accounts({
          signer: wallet.publicKey,
          user: source_user_pda,
          vaultConfig: vaultConfigPda,
          mint: mint2022.publicKey,
          userAta: sourceTokenAccount,
          vaultTa: vaultTokenAccount.publicKey,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .signers([wallet.payer])
        .rpc()
    );

    console.log("Withdraw transaction signature:", tx);

    // Verify balance was updated
    const userAccountAfter = await program.account.user.fetch(source_user_pda);
    assert.equal(
      userAccountAfter.balance.toString(),
      balanceBefore.sub(withdrawAmount).toString()
    );
  });
});
