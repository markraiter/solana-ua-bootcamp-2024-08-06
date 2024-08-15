import "dotenv/config";
import {
  Connection,
  LAMPORTS_PER_SOL,
  PublicKey,
  clusterApiUrl,
} from "@solana/web3.js";

const connection = new Connection(clusterApiUrl("devnet"));
const public_key = "5BgTrJEQw1XWSJ1DiX1hT78xiHC1RpNtcV8rwrbmGfwU";
console.log(`⚡️ Connected to devnet`);

const publicKey = new PublicKey(public_key);
const balanceInLamports = await connection.getBalance(publicKey);
const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;

console.log(
  `💰 The balance for the wallet at address ${publicKey} is: ${balanceInSOL}`
);
