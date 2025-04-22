import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { InfiniteAdventure } from '../../target/types/infinite_adventure';
import { Keypair, PublicKey } from '@solana/web3.js';
import { getAssociatedTokenAddressSync } from '@solana/spl-token';

// Configure the client to use the devnet cluster.
const cluster = 'devnet';
const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl(cluster), 'confirmed');

// Load the program ID from your Anchor.toml or hardcode it.
const programId = new PublicKey('YOUR_PROGRAM_ID_HERE'); // Replace with your program ID

// Generate a random keypair for the payer.
const payer = Keypair.generate();

async function main() {
  console.log('Running client...');

  // Create an AnchorProvider.
  const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(payer), {
    preflightCommitment: 'confirmed',
  });
  anchor.setProvider(provider);

  // Get the program.
  const program = anchor.workspace.InfiniteAdventure as Program<InfiniteAdventure>; // If using Anchor.toml

  // Alternatively, if not using Anchor.toml workspace:
  // const program = new Program<InfiniteAdventure>(idl as any, programId, provider);

  // Derive the PDAs for the game accounts.
  const gameLevelSeed = Buffer.from('level1');
  const gameMapAccountSeed = Buffer.from('game_map');

  const [gameDataAccount] = PublicKey.findProgramAddressSync(
    [gameLevelSeed],
    programId
  );

  const [gameMapAccount] = PublicKey.findProgramAddressSync(
    [gameMapAccountSeed],
    programId
  );

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
  } catch (error) {
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
    console.log('Game Map Locations:', gameMap.locations);
  } catch (error) {
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
    console.log('Updated Game Map Locations:', newGameMap.locations);
  } catch (error) {
    console.error(`Error moving ${moveDirection}:`, error);
  }
}

main().catch((err) => {
  console.error('Error:', err);
});