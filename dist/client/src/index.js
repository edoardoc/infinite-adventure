"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
const anchor = __importStar(require("@coral-xyz/anchor"));
const web3_js_1 = require("@solana/web3.js");
// Configure the client to use the devnet cluster.
const cluster = 'devnet';
const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl(cluster), 'confirmed');
// Load the program ID from your Anchor.toml or hardcode it.
const programId = new web3_js_1.PublicKey('YOUR_PROGRAM_ID_HERE'); // Replace with your program ID
// Generate a random keypair for the payer.
const payer = web3_js_1.Keypair.generate();
async function main() {
    console.log('Running client...');
    // Create an AnchorProvider.
    const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(payer), {
        preflightCommitment: 'confirmed',
    });
    anchor.setProvider(provider);
    // Get the program.
    const program = anchor.workspace.InfiniteAdventure; // If using Anchor.toml
    // Alternatively, if not using Anchor.toml workspace:
    // const program = new Program<InfiniteAdventure>(idl as any, programId, provider);
    // Derive the PDAs for the game accounts.
    const gameLevelSeed = Buffer.from('level1');
    const gameMapAccountSeed = Buffer.from('game_map');
    const [gameDataAccount] = web3_js_1.PublicKey.findProgramAddressSync([gameLevelSeed], programId);
    const [gameMapAccount] = web3_js_1.PublicKey.findProgramAddressSync([gameMapAccountSeed], programId);
    // --- Initialize the game ---
    console.log('Initializing game...');
    try {
        const tx = await program.methods
            .initialize()
            .accounts({
            newGameDataAccount: gameDataAccount,
            newGameMapAccount: gameMapAccount,
            signer: payer.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
            .signers([payer])
            .rpc();
        console.log('Initialized game. Transaction:', tx);
    }
    catch (error) {
        console.error('Error initializing game:', error);
        return;
    }
    // --- View the initial location ---
    console.log('\nViewing initial location...');
    try {
        const tx = await program.methods
            .viewLocation()
            .accounts({
            gameDataAccount: gameDataAccount,
            gameMapAccount: gameMapAccount,
            authority: payer.publicKey,
        })
            .signers([payer])
            .rpc();
        console.log('Viewed location. Transaction:', tx);
        const gameData = await program.account.gameDataAccount.fetch(gameDataAccount);
        const gameMap = await program.account.gameMapAccount.fetch(gameMapAccount);
        console.log('Current Player Location Index:', gameData.playerLocation);
        console.log('Game Map:', JSON.stringify(gameMap.locations, null, 2));
    }
    catch (error) {
        console.error('Error viewing location:', error);
    }
    // --- Example: Move North ---
    const moveDirection = 'north';
    console.log(`\nAttempting to move ${moveDirection}...`);
    try {
        const tx = await program.methods
            .movePlayer(moveDirection)
            .accounts({
            gameDataAccount: gameDataAccount,
            gameMapAccount: gameMapAccount,
            authority: payer.publicKey,
        })
            .signers([payer])
            .rpc();
        console.log(`Moved ${moveDirection}. Transaction:`, tx);
        const gameData = await program.account.gameDataAccount.fetch(gameDataAccount);
        console.log('New Player Location Index:', gameData.playerLocation);
        // View the new location
        const txView = await program.methods
            .viewLocation()
            .accounts({
            gameDataAccount: gameDataAccount,
            gameMapAccount: gameMapAccount,
            authority: payer.publicKey,
        })
            .signers([payer])
            .rpc();
        console.log('Viewed new location. Transaction:', txView);
        const newGameMap = await program.account.gameMapAccount.fetch(gameMapAccount);
        console.log('Game Map:', JSON.stringify(newGameMap.locations, null, 2));
    }
    catch (error) {
        console.error(`Error moving ${moveDirection}:`, error);
    }
}
main().catch((err) => {
    console.error('Error:', err);
});
