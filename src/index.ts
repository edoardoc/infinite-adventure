import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { InfiniteAdventure } from '../target/types/infinite_adventure';
import { Keypair, PublicKey } from '@solana/web3.js';
import * as readline from 'readline';

// Configure the client to use the devnet cluster.
const cluster = 'devnet';
const connection = new anchor.web3.Connection(anchor.web3.clusterApiUrl(cluster), 'confirmed');

// Load the program ID from your Anchor.toml or hardcode it.
const programId = new PublicKey('7gMbksSMJbpH45gVH3QDxTUa2w4oSv6rfdjifPM8LUwX'); // Replace with your program ID

// grab the payer from a private key
const privateKeyArray = Uint8Array.from([14,45,131,187,62,86,48,120,156,89,228,203,188,194,249,194,251,67,75,46,181,63,161,140,136,174,194,49,85,226,174,237,147,94,185,61,134,12,250,237,71,68,63,17,50,11,71,227,247,202,16,133,41,136,37,140,61,18,121,172,123,83,62,127]);
const payer = Keypair.fromSecretKey(privateKeyArray);
console.log('Payer Public Key:', payer.publicKey.toBase58());

async function main() {
  console.log('Running client...');

  // Create an AnchorProvider.
  const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(payer), {
    preflightCommitment: 'confirmed',
  });
  anchor.setProvider(provider);

  // Get the program.
  const program = anchor.workspace.InfiniteAdventure as Program<InfiniteAdventure>; // Anchor.toml

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

  try {
    const gameData = await program.account.gameDataAccount.fetch(gameDataAccount);
    const gameMap = await program.account.gameMapAccount.fetch(gameMapAccount);
    console.log('Accounts already initialized.');
    console.log('Current Player Location Index:', gameData.playerLocationIndex);
    console.log('Game Map:', JSON.stringify(gameMap.locations, null, 2));
  } catch (error) {
    console.log('Initializing game...');
    const tx = await program.methods
      .initialize()
      .accounts({
        newGameDataAccount: gameDataAccount as PublicKey,
        newGameMapAccount: gameMapAccount as PublicKey,
        signer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([payer])
      .rpc();
    console.log('Initialized game. Transaction:', tx);
  }

  // --- View the initial location ---
  console.log('\nViewing initial location...');
  try {
    const tx = await program.methods
      .viewLocation()
      .accounts({
        gameDataAccount: gameDataAccount as PublicKey,
        gameMapAccount: gameMapAccount as PublicKey,
        authority: payer.publicKey,
      } as any)
      .signers([payer])
      .rpc();
    console.log('Viewed location. Transaction:', tx);

    const gameData = await program.account.gameDataAccount.fetch(gameDataAccount);
    const gameMap = await program.account.gameMapAccount.fetch(gameMapAccount);
    console.log('Current Player Location Index:', gameData.playerLocationIndex);
    console.log('Game Map:', JSON.stringify(gameMap.locations, null, 2));
  } catch (error) {
    console.error('Error viewing location:', error);
  }




  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  async function movePlayerLoop() {
    while (true) {
      const moveDirection = await new Promise<string>((resolve) => {
        rl.question('\nEnter direction to move (north, south, east, west) or "stop" to exit: ', resolve);
      });

      if (moveDirection.toLowerCase() === 'stop') {
        console.log('Exiting movement loop...');
        break;
      }

      console.log(`\nAttempting to move ${moveDirection}...`);
      try {
        const tx = await program.methods
          .movePlayer(moveDirection)
          .accounts({
            gameDataAccount: gameDataAccount as PublicKey,
            gameMapAccount: gameMapAccount as PublicKey,
            authority: payer.publicKey,
          } as any)
          .signers([payer])
          .rpc();
        console.log(`Moved ${moveDirection}. Transaction:`, tx);

        const gameData = await program.account.gameDataAccount.fetch(gameDataAccount);
        console.log('New Player Location Index:', gameData.playerLocationIndex);

        // View the new location
        const txView = await program.methods
          .viewLocation()
          .accounts({
            gameDataAccount: gameDataAccount as PublicKey,
            gameMapAccount: gameMapAccount as PublicKey,
            authority: payer.publicKey,
          } as any)
          .signers([payer])
          .rpc();
        console.log('Viewed new location. Transaction:', txView);
        const newGameMap = await program.account.gameMapAccount.fetch(gameMapAccount);
        console.log('Updated Game Map:', JSON.stringify(newGameMap.locations, null, 2));
      } catch (error) {
        console.error(`Error moving ${moveDirection}:`, error);
      }
    }

    rl.close();
  }

  await movePlayerLoop();

}

main().catch((err) => {
  console.error('Error:', err);
});