import * as anchor from '@project-serum/anchor';
import * as assert from "assert";
import { Program } from '@project-serum/anchor';
import { SolanaAds } from '../target/types/solana_ads';
import { PublicKey } from '@solana/web3.js';
import BN from 'BN.js';

export const kolyanPublicKey = new PublicKey('hobB4CWoWbxatFFXfCJTKEeoWXbFfTDbrQmRdGr7aUz');
export const viktrchPublicKey = new PublicKey('B27hkJAvhk8bb2x7ytyNoXSRy9CvEQ5sWyTqGGMZghEF');


describe('solana_ads', () => {
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaAds as Program<SolanaAds>;

  it('can create ad!', async () => {
    
    const ad = anchor.web3.Keypair.generate();

    const title = 'test';
    const content = 'test';

    const derivedAddress = await anchor.web3.PublicKey.createWithSeed(
      program.provider.wallet.publicKey,
      'seed',
      program.programId,
    )

    console.log(derivedAddress.toBase58());

    const tx = await program.rpc.createAd(
      title,
      content,
      title.length + content.length,
      new BN(0),
      {
        accounts: {
          ad: ad.publicKey,
          kolyanAccount: kolyanPublicKey,
          viktrchAccount: viktrchPublicKey,
          authority: program.provider.wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
          derivedAddress: derivedAddress,
        },
        signers: [ad],
      }
    );

    console.log("Your transaction signature", tx);

    let adAccount = await program.account.ad.fetch(ad.publicKey);
    console.log(adAccount);
    assert.equal(adAccount.title, title);
    assert.equal(adAccount.content, content);
  });

  // it('can append ad content', async () => {
    
  //   const ad = anchor.web3.Keypair.generate();

  //   const title = 'test';
  //   const content = 'test';
  //   const appendedContent = 'appended content'

  //   const tx = await program.rpc.createAd(
  //     title,
  //     content,
  //     title.length + content.length + appendedContent.length,
  //     {
  //       accounts: {
  //         authority: program.provider.wallet.publicKey,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         ad: ad.publicKey,
  //       },
  //       signers: [ad],
  //     }
  //   );

  //   console.log("Your transaction signature", tx);

  //   let adAccount = await program.account.ad.fetch(ad.publicKey);
  //   console.log(adAccount);
  //   assert.equal(adAccount.title, title);
  //   assert.equal(adAccount.content, content);

  //   await program.rpc.appendAdContent(
  //     appendedContent,
  //     {
  //       accounts: {
  //         authority: program.provider.wallet.publicKey,
  //         ad: ad.publicKey,
  //       },
  //     }
  //   );

  //   adAccount = await program.account.ad.fetch(ad.publicKey);
  //   console.log(adAccount);
  //   assert.equal(adAccount.title, title);
  //   assert.equal(adAccount.content, content + appendedContent);
    
  // });

  // it('can not exceed text limit', async () => {
    
  //   const ad = anchor.web3.Keypair.generate();

  //   const title = 'test';
  //   const content = 'test';
  //   const appendedContent = 'appended content'

  //   const tx = await program.rpc.createAd(
  //     title,
  //     content,
  //     title.length + content.length,
  //     {
  //       accounts: {
  //         authority: program.provider.wallet.publicKey,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         ad: ad.publicKey,
  //       },
  //       signers: [ad],
  //     }
  //   );

  //   console.log("Your transaction signature", tx);

  //   try {
  //     await program.rpc.appendAdContent(
  //       appendedContent,
  //       {
  //         accounts: {
  //           authority: program.provider.wallet.publicKey,
  //           ad: ad.publicKey,
  //         },
  //       }
  //     );
  //   } catch (error) {
  //     assert.equal(error.msg, 'Can not update Ad. Text limit will be exceeded.');
  //     return;
  //   } 
  // });

  // it('can append a lot of content', async () => {
    
  //   const ad = anchor.web3.Keypair.generate();

  //   const title = "a".repeat(280);
  //   const content = '';

  //   let tx = await program.rpc.createAd(
  //     title,
  //     content,
  //     2500,
  //     {
  //       accounts: {
  //         authority: program.provider.wallet.publicKey,
  //         systemProgram: anchor.web3.SystemProgram.programId,
  //         ad: ad.publicKey,
  //       },
  //       signers: [ad],
  //     }
  //   );

  //   console.log("Your transaction signature", tx);

  //   let adAccount = await program.account.ad.fetch(ad.publicKey);
  //   console.log(adAccount);
  //   assert.equal(adAccount.title, title);
  //   assert.equal(adAccount.content, content);

  //   tx = await program.rpc.appendAdContent(
  //     "♡".repeat(300),
  //     {
  //       accounts: {
  //         authority: program.provider.wallet.publicKey,
  //         ad: ad.publicKey,
  //       },
  //     }
  //   );

  //   console.log("Your transaction signature", tx);

  //   adAccount = await program.account.ad.fetch(ad.publicKey);
  //   console.log(adAccount);
  // });
});
