import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { HorseRacing } from '../target/types/horse_racing';

describe('horse-racing', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.HorseRacing as Program<HorseRacing>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
