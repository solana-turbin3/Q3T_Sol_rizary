import { Keypair } from "@solana/web3.js";
import bs58 from 'bs58';

function base58ToWallet(base58String: string): string {
    try {
        const secretKey = bs58.decode(base58String);
        const keypair = Keypair.fromSecretKey(secretKey);
        return `[${Array.from(keypair.secretKey)}]`;
    } catch (error: unknown) {
        if (error instanceof Error) {
            throw new Error(`Failed to decode Base58 string: ${error.message}`);
        }
        throw new Error('Failed to decode Base58 string: Unknown error');
    }
}

function walletToBase58(): string {
    try {
        const secretKeyUint8 = Uint8Array.from([244,3,253,145,140,197,245,126,171,159,133,226,202,159,203,87,218,182,164,29,238,174,167,42,29,136,237,181,147,206,159,89,18,222,58,221,184,87,251,209,144,150,189,76,205,114,182,56,76,90,3,210,172,107,144,129,240,45,10,161,5,121,24,87]);
        const keypair = Keypair.fromSecretKey(secretKeyUint8);
        return bs58.encode(keypair.secretKey);
    } catch (error: unknown) {
        if (error instanceof Error) {
            throw new Error(`Failed to create keypair: ${error.message}`);
        }
        throw new Error('Failed to create keypair: Unknown error');
    }
}

if (require.main === module) {
    const prompt = require('prompt-sync')();
    console.log('Choose operation:');
    console.log('1. Base58 to Wallet');
    console.log('2. Wallet to Base58');
    const choice = prompt('');
    
    try {
        if (choice === '1') {
            console.log('Enter Base58 string:');
            const base58String = prompt('');
            const wallet = base58ToWallet(base58String);
            console.log(wallet);
        } else if (choice === '2') {
            console.log('Enter wallet array (comma-separated numbers):');
            const walletInput = prompt('');
            const wallet = walletInput.split(',').map((num: string) => parseInt(num.trim()));
            const base58String = walletToBase58();
            console.log(base58String);
        }
    } catch (error: unknown) {
        console.error('Error:', error instanceof Error ? error.message : 'Unknown error');
    }
}