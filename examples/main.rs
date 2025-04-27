use std::path::PathBuf;

use camt_053_001_02::{Document, ISO20022Document};
use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Opts {
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    println!("{opts:?}");

    let data = std::fs::read_to_string(opts.path)?;

    let d = ISO20022Document::from_str(&data)?;
    let Document::BankToCustomerStatement(s) = d.document;

    println!("Creation date: {}", s.header.creation_date);
    println!("Entries:");
    for e in &s.statement.entries {
        println!("== {} ==", e.book_date.dt);
        println!(
            "A: {} {} {}",
            e.amount.currency, e.amount.amount, e.credit_or_debit
        );
        if let Some(related) = &e.details.transaction.related_parties {
            if let Some(creditor) = &related.creditor {
                println!(
                    "P: {} - {:?}",
                    creditor.name,
                    related.creditor_account.as_ref().unwrap().id.identification
                );
            }
            if let Some(debitor) = &related.debitor {
                println!(
                    "P: {} - {:?} ",
                    debitor.name,
                    related.debitor_account.as_ref().unwrap().id.identification
                );
            }
        }
        if let Some(info) = &e.details.transaction.remittance_info {
            println!("Info: {info}");
        }
        if let Some(additional) = &e.additional_info {
            println!("Additional: {additional}");
        }
    }

    Ok(())
}
