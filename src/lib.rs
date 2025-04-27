use std::fmt::Display;

use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename = "Document")]
pub struct ISO20022Document {
    #[serde(rename = "$value")]
    pub document: Document,
}

impl ISO20022Document {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(document: &str) -> Result<Self, quick_xml::DeError> {
        let mut de = quick_xml::de::Deserializer::from_str(document);
        ISO20022Document::deserialize(&mut de)
    }
}

#[derive(Deserialize, Debug)]
pub enum Document {
    #[serde(rename = "BkToCstmrStmt")]
    BankToCustomerStatement(BankToCustomerStatement),
}

#[derive(Deserialize, Debug)]
pub struct BankToCustomerStatement {
    #[serde(rename = "GrpHdr")]
    pub header: Header,
    // assuming just one statement
    #[serde(rename = "Stmt")]
    pub statement: Statement,
}

#[derive(Deserialize, Debug)]
pub struct Header {
    #[serde(rename = "CreDtTm")]
    pub creation_date: DateTime<FixedOffset>,
}

#[derive(Deserialize, Debug)]
pub struct Statement {
    #[serde(rename = "Ntry")]
    pub entries: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
pub struct Entry {
    #[serde(rename = "Amt")]
    pub amount: Amount,
    #[serde(rename = "CdtDbtInd")]
    pub credit_or_debit: String,
    #[serde(rename = "BookgDt")]
    pub book_date: Date,
    #[serde(rename = "ValDt")]
    pub value_date: Date,
    #[serde(rename = "NtryDtls")]
    pub details: EntryDetails,
    #[serde(rename = "AddtlNtryInf")]
    pub additional_info: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Amount {
    #[serde(rename = "@Ccy")]
    pub currency: String,
    #[serde(rename = "$value")]
    pub amount: f64,
}

#[derive(Deserialize, Debug)]
pub struct Date {
    #[serde(rename = "Dt")]
    pub dt: NaiveDate,
}

#[derive(Deserialize, Debug)]
pub struct EntryDetails {
    #[serde(rename = "TxDtls")]
    pub transaction: TransactionDetails,
}
#[derive(Deserialize, Debug)]
pub struct TransactionDetails {
    #[serde(rename = "RmtInf")]
    pub remittance_info: Option<RemittanceInfo>,
    #[serde(rename = "RltdPties")]
    pub related_parties: Option<RelatedParties>,
}

#[derive(Deserialize, Debug)]
pub struct RemittanceInfo {
    #[serde(rename = "Ustrd")]
    pub unstructured: String,
}

impl Display for RemittanceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.unstructured)
    }
}

#[derive(Deserialize, Debug)]
pub struct RelatedParties {
    #[serde(rename = "Cdtr")]
    pub creditor: Option<PartyId>,
    #[serde(rename = "CdtrAcct")]
    pub creditor_account: Option<PartyAccount>,
    #[serde(rename = "Dbtr")]
    pub debitor: Option<PartyId>,
    #[serde(rename = "DbtrAcct")]
    pub debitor_account: Option<PartyAccount>,
}

#[derive(Deserialize, Debug)]
pub struct PartyId {
    #[serde(rename = "Nm")]
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct PartyAccount {
    #[serde(rename = "Id")]
    pub id: AccountId,
}

#[derive(Deserialize, Debug)]
pub struct AccountId {
    #[serde(rename = "$value")]
    pub identification: AccountIdentification,
}

#[derive(Deserialize, Debug)]
pub enum AccountIdentification {
    IBAN(String),
}
