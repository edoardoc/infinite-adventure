use std::rc::Rc;

use anchor_client::{
    solana_sdk::{
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        pubkey::Pubkey,
        system_program,
    },
    Client, Cluster,
};

// Define or import the MyAccount struct
#[derive(Debug, anchor_lang::AccountDeserialize)]
pub struct MyAccount {
    pub field: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let payer = read_keypair_file("keypair.json")?;
    let client = Client::new(Cluster::Localnet, Rc::new(payer));

    // Create program
    let program = client.program(Pubkey::from_str_const("33BuEyGHLbL7up1w6NK8NHRTxYiQmcRANCKp5tfKuv1m"))?;

    // Send transaction
    let my_account_kp = Keypair::new();
    program
        .request()
        .accounts(infinite_adventure::accounts::Initialize {
            my_account: my_account_kp.pubkey(),
            payer: program.payer(),
            system_program: system_program::ID,
        })
        .args(infinite_adventure::instruction::Initialize { field: 42 })
        .signer(&my_account_kp)
        .send()?;

    // Fetch account
    let my_account: MyAccount = program.account(my_account_kp.pubkey())?;
    assert_eq!(my_account.field, 42);

    Ok(())
}