import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import {
    ASSOCIATED_TOKEN_PROGRAM_ID,
    createAssociatedTokenAccountInstruction,
    createInitializeMintInstruction,
    createMintToInstruction,
    getAssociatedTokenAddress, getAssociatedTokenAddressSync,
    TOKEN_PROGRAM_ID
} from "@solana/spl-token";
import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import { assert } from "chai";
import * as fs from 'fs';
import * as path from 'path';
import { Udie } from "../target/types/udie";

const admin = Keypair.fromSecretKey(
    Buffer.from(JSON.parse(fs.readFileSync(path.join(__dirname, 'wallets', 'admin-keypair.json'), 'utf-8')))
);
// owner: 8WDTeBsfimB7UpfrqjRJbFtGkZsdyF3aQXSWPsW6oLTo
const owner = Keypair.fromSecretKey(
    Buffer.from(JSON.parse(fs.readFileSync(path.join(__dirname, 'wallets', 'owner-keypair.json'), 'utf-8')))
);
// beneficiary: 3NL8ad8Ab7pDmJkXFYwzcZ3mmwLDGNogcgaL17btXBfo
const beneficiary = Keypair.fromSecretKey(
    Buffer.from(JSON.parse(fs.readFileSync(path.join(__dirname, 'wallets', 'beneficiary-keypair.json'), 'utf-8')))
);

// Set up localnet connection with new port
const connection = new Connection("http://localhost:8899");  // Change port if needed

// Create a custom provider for admin operations
const provider1 = new anchor.AnchorProvider(
    connection,
    new anchor.Wallet(admin),  // Use admin's keypair
    { commitment: "confirmed" }
);

// Create a custom provider for owner operations
const provider2 = new anchor.AnchorProvider(
    connection,
    new anchor.Wallet(owner),  // Use owner's keypair
    { commitment: "confirmed" }
);

// Add this helper
const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

// Add this helper function at the top level
async function confirmTransaction(
    connection: Connection,
    signature: string,
    commitment: Commitment = 'confirmed'
): Promise<void> {
    const latestBlockhash = await connection.getLatestBlockhash();
    
    return new Promise((resolve, reject) => {
        let done = false;
        let retries = 5; // Add retry count
        
        const checkSignature = async () => {
            if (done) return;
            
            try {
                const status = await connection.getSignatureStatus(signature);
                const blockHeight = await connection.getBlockHeight();
                
                if (status?.value?.confirmationStatus === commitment) {
                    done = true;
                    resolve();
                    return;
                }

                if (blockHeight > latestBlockhash.lastValidBlockHeight) {
                    done = true;
                    reject(new Error(`Block height exceeded for ${signature}`));
                    return;
                }

                retries--;
                if (retries <= 0) {
                    done = true;
                    reject(new Error(`Retry limit reached for ${signature}`));
                    return;
                }

                // Wait and retry
                setTimeout(checkSignature, 1000);
            } catch (err) {
                if (!done) {
                    done = true;
                    reject(err);
                }
            }
        };

        // Start checking
        checkSignature();

        // Set a timeout as a fallback
        setTimeout(() => {
            if (!done) {
                done = true;
                console.log('Timed out for', signature);
                reject(new Error(`Global timeout for ${signature}`));
            }
        }, 60000); // Increased to 60 seconds
    });
}

describe("udie", () => {
    // Set the custom provider
    anchor.setProvider(provider1);

    const program = anchor.workspace.Udie as Program<Udie>;

    let adminPda: PublicKey;
    let inheritance_plan: PublicKey;
    let inheritance_plan_bump: number;
    let asset: PublicKey;
    let asset_bump: number;
    let vault: PublicKey;
    let beneficiary_account: PublicKey;
    let beneficiary_ata: PublicKey;

    // mint can still be generated if needed
    const mint = Keypair.generate();

        
    // Store accounts that we'll reuse across tests
    let mintKeypair: Keypair;

    let beneficiary: Keypair;  // Store beneficiary keypair
    beneficiary = Keypair.generate();

    before(async () => {
        // Find PDAs
        [adminPda] = PublicKey.findProgramAddressSync(
            [anchor.utils.bytes.utf8.encode("admin_config"), admin.publicKey.toBuffer()],
            program.programId
        );

        [inheritance_plan, inheritance_plan_bump] = PublicKey.findProgramAddressSync(
            [
                anchor.utils.bytes.utf8.encode("inheritance_plan"),
                owner.publicKey.toBuffer()
            ],
            program.programId
        );

        [asset, asset_bump] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("asset"),
                inheritance_plan.toBuffer(),
                mint.publicKey.toBuffer()
            ],
            program.programId
        );

        [beneficiary_account] = PublicKey.findProgramAddressSync(
            [
                Buffer.from("beneficiary"),
                inheritance_plan.toBuffer(),
                beneficiary.publicKey.toBuffer()
            ],
            program.programId
        );

        // Get ATA for beneficiary
        beneficiary_ata = getAssociatedTokenAddressSync(
            mint.publicKey,
            beneficiary.publicKey
        );

        try {
            // Use admin provider
            const tx = await program.methods
                .initialize(100)
                .accounts([{
                    admin: admin.publicKey,
                    adminConfig: adminPda,
                    systemProgram: SystemProgram.programId,
                }])
                .signers([admin])
                .rpc();

            await program.provider.connection.confirmTransaction({
                signature: tx,
                blockhash: (await program.provider.connection.getLatestBlockhash()).blockhash,
                lastValidBlockHeight: (await program.provider.connection.getLatestBlockhash()).lastValidBlockHeight
            });

            // Wait a bit before fetching transaction details
            await sleep(15000);

        } catch (error) {
            console.error("Error:", error);
            if (error.logs) {
                console.error("Program logs:", error.logs);
            }
            throw error;
        }

        // Initialize beneficiary here so it's available for all tests
        beneficiary = Keypair.generate();
    });

    before(async () => {
        // Airdrop to admin and owner
        await connection.requestAirdrop(admin.publicKey, 5 * LAMPORTS_PER_SOL);
        await sleep(500);
        
        await connection.requestAirdrop(owner.publicKey, 5 * LAMPORTS_PER_SOL);
        await sleep(500);

        await connection.requestAirdrop(beneficiary.publicKey, 5 * LAMPORTS_PER_SOL);
        await sleep(500);

    });

    describe("Success Cases", () => {
        it("Config Initialized", async () => {
            // Use admin provider
            anchor.setProvider(provider1);
            const program = anchor.workspace.Udie as Program<Udie>;

            // Verify config state
            const accountInfo = await connection.getAccountInfo(adminPda);
            const configAccount = await program.account.adminConfig.fetch(adminPda);
            assert.notEqual(accountInfo, null);
            assert.equal(configAccount.admin.toBase58(), admin.publicKey.toBase58());
            assert.equal(configAccount.fee, 100);
        });

        it("Create Inheritance Plan", async () => {
            anchor.setProvider(provider2);
            // Create new program instance with owner provider
            const ownerProgram = new anchor.Program(
                anchor.workspace.Udie.idl,
                provider2,
                anchor.workspace.Udie.coder,
            ) as Program<Udie>;

            const tx = await ownerProgram.methods
                .createInheritancePlan()
                .accounts([{
                    owner: owner.publicKey,
                    inheritancePlan: inheritance_plan,
                    systemProgram: SystemProgram.programId,
                }])
                .signers([owner])
                .rpc();
            await connection.confirmTransaction({
                signature: tx,
                blockhash: (await connection.getLatestBlockhash()).blockhash,
                lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight
            });

            // Wait for confirmation
            await sleep(15000);

            // Verify inheritance plan state
            const planAccount = await ownerProgram.account.inheritancePlan.fetch(inheritance_plan);
            assert.equal(planAccount.owner.toBase58(), owner.publicKey.toBase58());
            assert.equal(planAccount.isActive, true);
            assert.equal(planAccount.deathVerified, false);
            assert.equal(planAccount.totalBeneficiaries, 0);
            assert.equal(planAccount.totalAssets, 0);
        });

        it("Add Beneficiary", async () => {
            anchor.setProvider(provider2);
            // Create new program instance with owner provider
            const ownerProgram = new anchor.Program(
                anchor.workspace.Udie.idl,
                provider2,
                anchor.workspace.Udie.coder
            ) as Program<Udie>;

            // Get inheritance plan PDA with correct seeds
            [inheritance_plan] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("inheritance_plan"),
                    owner.publicKey.toBuffer()
                ],
                ownerProgram.programId
            );

            // Get beneficiary PDA with correct seeds
            [beneficiary_account] = PublicKey.findProgramAddressSync(
                [
                    Buffer.from("beneficiary"),
                    inheritance_plan.toBuffer(),
                    beneficiary.publicKey.toBuffer()
                ],
                ownerProgram.programId
            );

            const tx = await ownerProgram.methods
                .addBeneficiary("son", 50)
                .accounts({
                    owner: owner.publicKey,
                    inheritancePlan: inheritance_plan,
                    beneficiary: beneficiary_account,
                    beneficiaryWallet: beneficiary.publicKey,
                    systemProgram: SystemProgram.programId,
                })
                .signers([owner])
                .rpc();

            await connection.confirmTransaction({
                signature: tx,
                blockhash: (await connection.getLatestBlockhash()).blockhash,
                lastValidBlockHeight: (await connection.getLatestBlockhash()).lastValidBlockHeight
            });

            // Wait for confirmation
            await sleep(15000);

            // Verify beneficiary was added
            const beneficiaryInfo = await ownerProgram.account.beneficiary.fetch(beneficiary_account);
            assert.equal(beneficiaryInfo.wallet.toBase58(), beneficiary.publicKey.toBase58());
            assert.equal(beneficiaryInfo.relationship, "son");
            assert.equal(beneficiaryInfo.sharePercentage, 50);
        });

        it("Add Asset", async () => {
            try {
                // Create mint
                mintKeypair = Keypair.generate();

                const ownerProgram = new anchor.Program(
                    anchor.workspace.Udie.idl,
                    provider2,
                    anchor.workspace.Udie.coder
                ) as Program<Udie>;

                const createMintAccountIx = SystemProgram.createAccount({
                    fromPubkey: owner.publicKey,
                    newAccountPubkey: mintKeypair.publicKey,
                    space: 82,
                    lamports: await connection.getMinimumBalanceForRentExemption(82),
                    programId: TOKEN_PROGRAM_ID
                });

                // 2. Initialize the mint
                const initializeMintIx = createInitializeMintInstruction(
                    mintKeypair.publicKey,
                    9,
                    owner.publicKey,
                    null
                );

                // Send both instructions in one transaction
                const mintTx = new Transaction().add(createMintAccountIx, initializeMintIx);
                const latestBlockhash = await connection.getLatestBlockhash();
                const mintSig = await sendAndConfirmTransaction(
                    connection, 
                    mintTx, 
                    [owner, mintKeypair]
                );
                await connection.confirmTransaction({
                    signature: mintSig,
                    blockhash: latestBlockhash.blockhash,
                    lastValidBlockHeight: latestBlockhash.lastValidBlockHeight
                });
                await sleep(13000);

                // 3. Create Associated Token Account (ATA)
                const ataAddress = await getAssociatedTokenAddress(
                    mintKeypair.publicKey,
                    owner.publicKey
                );

                const createAtaIx = createAssociatedTokenAccountInstruction(
                    owner.publicKey,
                    ataAddress,
                    owner.publicKey,
                    mintKeypair.publicKey
                );

                const latestBlockhash2 = await connection.getLatestBlockhash();
                const ataSig = await sendAndConfirmTransaction(
                    connection, 
                    new Transaction().add(createAtaIx), 
                    [owner]
                );
                await connection.confirmTransaction({
                    signature: ataSig,
                    blockhash: latestBlockhash2.blockhash,
                    lastValidBlockHeight: latestBlockhash2.lastValidBlockHeight
                });
                await sleep(13000);

                // 4. Mint tokens to the ATA
                const mintToIx = createMintToInstruction(
                    mintKeypair.publicKey,
                    ataAddress,
                    owner.publicKey,
                    5000000000
                );

                const latestBlockhash3 = await connection.getLatestBlockhash();
                const mintToSig = await sendAndConfirmTransaction(
                    connection, 
                    new Transaction().add(mintToIx), 
                    [owner]
                );
                await connection.confirmTransaction({
                    signature: mintToSig,
                    blockhash: latestBlockhash3.blockhash,
                    lastValidBlockHeight: latestBlockhash3.lastValidBlockHeight
                });
                await sleep(13000);

                // 5. Get PDAs and add asset to inheritance plan
                [inheritance_plan] = PublicKey.findProgramAddressSync(
                    [Buffer.from("inheritance_plan"), owner.publicKey.toBuffer()],
                    ownerProgram.programId
                );

                [asset] = PublicKey.findProgramAddressSync(
                    [Buffer.from("asset"), inheritance_plan.toBuffer(), mintKeypair.publicKey.toBuffer()],
                    ownerProgram.programId
                );

                // Get vault (which is an ATA)
                const vault = await getAssociatedTokenAddress(
                    mintKeypair.publicKey,
                    asset,
                    true // allowOwnerOffCurve
                );

                // Add asset to inheritance plan
                await ownerProgram.methods
                    .addAsset(new anchor.BN(1000000000))
                    .accounts({
                        owner: owner.publicKey,
                        inheritancePlan: inheritance_plan,
                        asset: asset,
                        mint: mintKeypair.publicKey,
                        ownerAta: ataAddress,
                        vault: vault,
                        systemProgram: SystemProgram.programId,
                        tokenProgram: TOKEN_PROGRAM_ID,
                        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    })
                    .signers([owner])
                    .rpc();

                await sleep(15000);

                // Verify asset was created correctly
                const assetAccount = await program.account.asset.fetch(asset);

                // Verify vault balance
                const vaultBalance = await connection.getTokenAccountBalance(vault);

                // Assertions
                assert.equal(assetAccount.mint.toBase58(), mintKeypair.publicKey.toBase58());
                assert.equal(assetAccount.amount.toString(), "1000000000");
                assert.equal(assetAccount.inheritancePlan.toBase58(), inheritance_plan.toBase58());
                assert.equal(assetAccount.vault.toBase58(), vault.toBase58());
                assert.equal(vaultBalance.value.amount, "1000000000");

                // // Store all accounts for next tests
                // console.log("Storing accounts for next tests:");
                // console.log("- Mint:", mintKeypair.publicKey.toBase58());
                // console.log("- Inheritance Plan:", inheritance_plan.toBase58());
                // console.log("- Asset:", asset.toBase58());
                // console.log("- Vault:", vault.toBase58());
                // console.log("- Beneficiary:", beneficiary.publicKey.toBase58());
                // console.log("- Beneficiary Account:", beneficiary_account.toBase58());

            } catch (error) {
                console.error("Full error:", error);
                if (error.logs) {
                    console.log("Program logs:", error.logs);
                }
                throw error;
            }
        });

        it("Verify Death", async () => {
            try {
                // Get the death verification PDA
                const [verificationPda] = PublicKey.findProgramAddressSync(
                    [Buffer.from("death_verification"), inheritance_plan.toBuffer()],
                    program.programId
                );

                const tx = await program.methods
                    .verifyDeath("QmHash123")
                    .accounts({
                        authority: admin.publicKey,
                        inheritancePlan: inheritance_plan,
                        verification: verificationPda,
                        systemProgram: SystemProgram.programId,
                    })
                    .signers([admin])
                    .rpc();

                await connection.confirmTransaction({
                    signature: tx,
                    ...(await connection.getLatestBlockhash()),
                });

            } catch (error) {
                console.error("Error:", error);
                throw error;
            }
        });

        it("Withdraw Asset", async () => {
            const sleep = async (ms: number) => new Promise(r => setTimeout(r, ms));

            async function sendAndMonitorTransaction(
                transaction: Transaction,
                signer: Keypair,
                label: string = "Transaction"
            ) {
                // Get fresh blockhash and lastValidBlockHeight
                const blockhashResponse = await connection.getLatestBlockhash('confirmed');
                // console.log(`${label} - Initial blockhash:`, blockhashResponse.blockhash);
                // console.log(`${label} - Initial lastValidBlockHeight:`, blockhashResponse.lastValidBlockHeight);
                
                let currentBlockHeight = await connection.getBlockHeight();
                // console.log(`${label} - Current blockHeight:`, currentBlockHeight);

                // Set a more conservative lastValidBlockHeight
                const lastValidBlockHeight = blockhashResponse.lastValidBlockHeight - 150;
                // console.log(`${label} - Adjusted lastValidBlockHeight:`, lastValidBlockHeight);

                // Update transaction with new blockhash
                transaction.recentBlockhash = blockhashResponse.blockhash;
                transaction.feePayer = signer.publicKey;
                transaction.lastValidBlockHeight = lastValidBlockHeight;
                transaction.sign(signer);
                const rawTransaction = transaction.serialize();

                let signature: string | null = null;
                let retryCount = 0;
                const maxRetries = 10;

                while (currentBlockHeight < lastValidBlockHeight && retryCount < maxRetries) {
                    try {
                        if (!signature) {
                            // Only send the transaction if we don't have a signature yet
                            signature = await connection.sendRawTransaction(rawTransaction, {
                                skipPreflight: false,
                                preflightCommitment: 'confirmed'
                            });
                            // console.log(`${label} - Transaction sent with signature:`, signature);
                        }

                        // Check transaction status
                        const status = await connection.getSignatureStatus(signature);
                        // console.log(`${label} - Transaction status:`, status.value?.confirmationStatus);

                        if (status.value?.confirmationStatus === 'confirmed' || 
                            status.value?.confirmationStatus === 'finalized') {
                            // console.log(`${label} - Transaction confirmed!`);
                            return signature;
                        }

                    } catch (e) {
                        console.log(`${label} - Retry ${retryCount + 1}/${maxRetries} failed:`, e.message);
                    }

                    retryCount++;
                    await sleep(1000);
                    currentBlockHeight = await connection.getBlockHeight();
                    // console.log(`${label} - Current blockHeight: ${currentBlockHeight}, Target: ${lastValidBlockHeight}`);
                }

                throw new Error(`${label} - Failed after ${retryCount} retries`);
            }

            const vault = await getAssociatedTokenAddress(
                mintKeypair.publicKey,
                asset,
                true
            );
            
            try {
                // Create owner program instance first
                const ownerProgram = new anchor.Program(
                    anchor.workspace.Udie.idl,
                    provider2,
                    anchor.workspace.Udie.coder
                ) as Program<Udie>;

                // Derive beneficiary account PDA with ownerProgram.programId
                [beneficiary_account] = PublicKey.findProgramAddressSync(
                    [
                        Buffer.from("beneficiary"),
                        inheritance_plan.toBuffer(),
                        beneficiary.publicKey.toBuffer()
                    ],
                    ownerProgram.programId  // Use ownerProgram.programId instead of program.programId
                );

                // Get vault (which is an ATA)
                const vault = await getAssociatedTokenAddress(
                    mintKeypair.publicKey,
                    asset,
                    true // allowOwnerOffCurve
                );
                
                // console.log("Using stored accounts:");
                // console.log("- Mint:", mintKeypair.publicKey.toBase58());
                // console.log("- Inheritance Plan:", inheritance_plan.toBase58());
                // console.log("- Asset:", asset.toBase58());
                // console.log("- Vault:", vault.toBase58());
                // console.log("- Beneficiary:", beneficiary.publicKey.toBase58());
                // console.log("- Derived Beneficiary Account:", beneficiary_account.toBase58());

                
                // Create beneficiary's ATA if it doesn't exist
                const beneficiaryAta = await getAssociatedTokenAddress(
                    mintKeypair.publicKey,
                    beneficiary.publicKey
                );

                // Create ATA if it doesn't exist
                if (!(await connection.getAccountInfo(beneficiaryAta))) {
                    const createAtaTx = new Transaction().add(
                        createAssociatedTokenAccountInstruction(
                            beneficiary.publicKey,
                            beneficiaryAta,
                            beneficiary.publicKey,
                            mintKeypair.publicKey
                        )
                    );
                    
                    await sendAndMonitorTransaction(createAtaTx, beneficiary, "Create ATA");
                    await sleep(5000);
                }

                const withdrawIx = await program.methods
                    .withdrawAsset()
                    .accounts({
                        beneficiary: beneficiary.publicKey,
                        inheritancePlan: inheritance_plan,
                        beneficiaryAccount: beneficiary_account,
                        mint: mintKeypair.publicKey,
                        asset: asset,
                        vault: vault,
                        beneficiaryAta: beneficiaryAta,
                        tokenProgram: TOKEN_PROGRAM_ID,
                        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                        systemProgram: SystemProgram.programId,
                    })
                    .instruction();

                const withdrawTx = new Transaction().add(withdrawIx);
                
                const txid = await sendAndMonitorTransaction(withdrawTx, beneficiary, "Withdraw");

                // Add extra verification step
                const confirmedTx = await connection.getTransaction(txid, {
                    commitment: 'confirmed',
                    maxSupportedTransactionVersion: 0
                });

                if (!confirmedTx) {
                    throw new Error("Transaction not found after confirmation");
                }

                await sleep(15000);

                // Verify withdrawal
                const beneficiaryBalance = await connection.getTokenAccountBalance(beneficiaryAta);
                assert.equal(beneficiaryBalance.value.amount, "500000000");

            } catch (error) {
                console.error("Final error:", error);
                if (error.logs) {
                    console.log("Program logs:", error.logs);
                }
                throw error;
            }
        });
    });
});

async function airdrop(connection, address: PublicKey, amount: number) {
    const signature = await connection.requestAirdrop(
        address,
        amount * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(signature);
    console.log(`Airdropped ${amount} SOL to ${address.toBase58()}`);
}
