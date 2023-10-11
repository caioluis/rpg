use sqlx::postgres::PgPool;
use axum::{extract::{self, State}, Json, Router, routing::{post,get}};
use serde::{Deserialize, Serialize};

use domain::banking::{Account, AccountData, Statement, Transaction, Transfer, TransferRecap};

pub struct BankingRouter;

impl BankingRouter {
    pub fn new_router(pool: PgPool) -> Router {
        Router::new()
            .route("/create-account", post(create_account))
            .route("/get-account-data", get(get_account_data))
            .route("/transfer", post(transfer))
            .route("/generate-statements", post(generate_statements))
            .route("/get-transactions", get(get_transactions))
            .with_state(pool)
    }
}

#[derive(Serialize, Deserialize)]
pub struct TransactionIds {
    ids: Vec<i64>,
}

pub async fn get_transactions(State(pool): State<PgPool>, extract::Json(payload): extract::Json<TransactionIds>) -> Json<Vec<Transaction>> {
    let transactions = Transaction::get_transactions(pool, &payload.ids).await.unwrap();
    Json(transactions)
}

pub async fn create_account(State(pool): State<PgPool>) -> Json<i32> {
    let account_id = Account::create_account(pool).await.unwrap();
    Json(account_id)
}

#[derive(Serialize, Deserialize)]
pub struct AccountId {
    id: i32,
}
pub async fn get_account_data(State(pool): State<PgPool>, extract::Json(payload): extract::Json<AccountId>) -> Json<AccountData> {
    let account_data = Account::get_account_data(pool, payload.id.into() ).await.unwrap();
    Json(account_data)
}

#[derive(Serialize, Deserialize)]
pub struct TransferPayload {
    from_account_id: i32,
    to_account_id: i32,
    amount: i64,
    description: String,
}

pub async fn transfer(
    State(pool): State<PgPool>,
    extract::Json(payload): Json<TransferPayload>
) -> Json<TransferRecap> {
    let transfer_recap = Transfer::transfer(pool, payload.from_account_id, payload.to_account_id, payload.amount, payload.description.as_ref())
        .await
        .unwrap();
    Json(transfer_recap)
}

pub async fn generate_statements(State(pool): State<PgPool>) -> Json<()> {
    Statement::generate_statements(pool).await.unwrap();
    Json(())
}
