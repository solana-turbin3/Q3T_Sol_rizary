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
        const secretKeyUint8 = Uint8Array.from([248,48,145,199,98,182,82,38,185,197,41,236,229,245,17,159,30,120,163,147,27,138,105,62,128,93,178,131,201,192,124,212,205,53,167,166,73,43,93,39,8,19,148,34,212,239,47,206,242,67,48,119,89,138,56,36,103,167,203,156,138,88,20,231]);
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