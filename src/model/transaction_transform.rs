use super::transaction::{Transaction, TransactionContentSrv, TransactionSrv};

use chrono::{prelude::*, DateTime, FixedOffset, Local, Utc};
use polars::prelude::*;
use surrealdb::sql::Thing;

#[allow(unused_imports)]
use crate::model::{Error, Result};

pub async fn transform(transactions: Vec<TransactionSrv>) -> Result<()> {
    let df: DataFrame = df!(
        "id" => transactions.clone().into_iter().map(|t| t.id.id.to_raw()).collect::<Vec<String>>(),
        "amount"=> transactions.clone().into_iter().map(|t| t.data.amount).collect::<Vec<f64>>(),
        "title"=> transactions.clone().into_iter().map(|t| t.data.title).collect::<Vec<String>>(),
        "konto"=> transactions.clone()
            .into_iter()
            .map(|t| t.data.konto
                .into_iter()
                .map(|k| k.id.to_raw())
                .collect::<Vec<String>>()
                .join(", "))
            .collect::<Vec<String>>(),
        "immo"=> transactions.clone()
            .into_iter()
            .map(|t| t.data.immo
                .into_iter()
                .map(|k| k.id.to_raw())
                .collect::<Vec<String>>()
                .join(", "))
            .collect::<Vec<String>>(),
        "date"=> transactions.clone().into_iter()
            .map(|t| t.data.date.to_rfc3339())
            .collect::<Vec<String>>(),
    )
    .unwrap();

    println!("{}", df);
    Ok(())
}
