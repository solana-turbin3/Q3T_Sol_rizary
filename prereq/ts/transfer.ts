import {
    Transaction, SystemProgram, Connection, Keypair,
    LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey
} from
    "@solana/web3.js"
import wallet from "./dev-wallet.json"



const from = Keypair.fromSecretKey(new Uint8Array(wallet));
const to = Keypair.fromSecretKey(new Uint8Array([
    248, 48, 145, 199, 98, 182, 82, 38, 185, 197, 41, 236, 229, 245, 17, 159,
    30, 120, 163, 147, 27, 138, 105, 62, 128, 93, 178, 131, 201, 192, 124, 212,
    205, 53, 167, 166, 73, 43, 93, 39, 8, 19, 148, 34, 212, 239, 47, 206,
    242, 67, 48, 119, 89, 138, 56, 36, 103, 167, 203, 156, 138, 88, 20, 231
]));
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
    try {
        // Get balance of dev wallet
        const balance = await connection.getBalance(from.publicKey)
        // Create a test transaction to calculate fees
        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to.publicKey,
                lamports: balance,
            })
        );
        transaction.recentBlockhash = (await

            connection.getLatestBlockhash('confirmed')).blockhash;

        transaction.feePayer = from.publicKey;
    // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
        const fee = (await

            connection.getFeeForMessage(transaction.compileMessage(),
                'confirmed')).value || 0;
        // Remove our transfer instruction to replace it
        transaction.instructions.pop();
        // Now add the instruction back with correct amount of lamports

        transaction.add(
            SystemProgram.transfer({
                fromPubkey: from.publicKey,
                toPubkey: to.publicKey,
                lamports: balance - fee,
            })
        );
        // Sign transaction, broadcast, and confirm
        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
        );
        console.log(`Success! Check out your TX here:
    https://explorer.solana.com/tx/${signature}?cluster=devnet`)
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();