use sqlx::postgres::PgPool;
use std::{
    iter::Extend,
    default::Default
};
use serde::Serialize;

#[derive(Serialize)]
pub struct AccountData {
    pub account: Account,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize)]
pub struct TransferRecap {
    pub amount: i64,
    pub description: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Transaction {
    pub id: i64,
    pub transaction_type: String,
    pub amount: i64,
    pub description: String,
    pub status: String,
    pub related_transaction_id: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
impl Default for Account {
    fn default() -> Self {
        Account {
            id: 0,
            status: String::new(),
            balance: 0,
        }
    }
}

impl Extend<Account> for Account {
    fn extend<T: IntoIterator<Item = Account>>(&mut self, iter: T) {
        for account in iter {
            self.id = account.id;
            self.status = account.status;
            self.balance = account.balance;
        }
    }
}
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Account {
    pub id: i32,
    pub status: String,
    pub balance: i64,
}

pub struct Transfer {}

#[derive(sqlx::FromRow, Debug)]
pub struct Statement {
    pub id: i64,
    pub account_id: i32,
    pub balance: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

type DBResult<T> = Result<T, Box<dyn std::error::Error>>;

impl Transaction {
    pub async fn get_transactions(pool: PgPool, ids: &[i64]) -> DBResult<Vec<Transaction>> {
        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT id, transaction_type, amount, description, status, related_transaction_id, created_at
            FROM banking_transactions
            WHERE id = ANY($1)
            "#,
            ids
        )
            .fetch_all(&pool)
            .await?;

        Ok(transactions)
    }
}

impl Account {
    pub async fn create_account(pool: PgPool) -> DBResult<i32> {
        let result = sqlx::query!(
        r#"
        WITH NewAccount AS (
        INSERT INTO banking.accounts (status)
            VALUES ('ACTIVE')
            RETURNING id
        )
        INSERT INTO banking_transactions (account_id, transaction_type, amount, description, status, related_transaction_id)
        VALUES
            ((SELECT id FROM NewAccount), 'DEPOSIT', 5000000, 'INITIAL_DEPOSIT', 'COMPLETE', NULL)
        RETURNING id::INTEGER;
        "#,
    )
            .fetch_one(&pool)
            .await;

        match result {
            Ok(record) => {
                match record.id {
                    Some(id) => Ok(id),
                    None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Account creation failed"))),
                }
            },
            Err(e) => Err(Box::new(e)),
        }
    }
    pub async fn get_account_data(pool: PgPool, account_id: i32) -> DBResult<AccountData> {
        let mut tx = pool.begin().await?;

        let account = sqlx::query_as!(
            Account,
            r#"
            SELECT id as "id!", status as "status!", balance as "balance!"
            FROM banking_accounts_balance
            WHERE id = $1
            "#,
            account_id
        )
            .fetch_one(&mut *tx)
            .await?;

        let transactions = sqlx::query_as!(
            Transaction,
            r#"
            SELECT id, transaction_type, amount, description, status, related_transaction_id, created_at
            FROM banking_transactions
            WHERE account_id = $1
            "#,
            account_id
        )
            .fetch_all(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(AccountData {
            account,
            transactions,
        })
    }
}

impl Transfer {
    pub async fn transfer(
        pool: PgPool,
        from_account_id: i32,
        to_account_id: i32,
        amount: i64,
        description: &str,
    ) -> DBResult<TransferRecap> {
        let transfer_recap = sqlx::query!(
            r#"
                SELECT *
                FROM transfer($1, $2, $3, $4)
            "#,
            from_account_id,
            to_account_id,
            amount,
            description
        )
            .fetch_one(&pool)
            .await?;

        Ok(
            TransferRecap {
                amount: transfer_recap.amount.unwrap_or(0),
                description: transfer_recap.description.unwrap_or("Failed.".to_string()),
                status: transfer_recap.status.unwrap_or("Failed.".to_string()),
                created_at: transfer_recap.created_at.unwrap_or(chrono::Utc::now()),
            }
        )
    }
}

impl Statement {
    pub async fn generate_statements(pool: PgPool) -> DBResult<()> {
        let mut tx = pool.begin().await?;

        sqlx::query!(
            r#"
            WITH CurrentMonthTransactions AS (
                SELECT
                    account_id,
                    COALESCE(SUM(CASE WHEN transaction_type = 'DEPOSIT' THEN amount ELSE -amount END), 0)::bigint AS current_balance
                FROM banking_transactions
                WHERE EXTRACT(MONTH FROM created_at) = EXTRACT(MONTH FROM NOW())
                  AND EXTRACT(YEAR FROM created_at) = EXTRACT(YEAR FROM NOW())
                GROUP BY account_id
            ),
            PreviousMonthBalance AS (
                SELECT
                    account_id,
                    COALESCE(balance, 0) AS previous_balance
                FROM banking_accounts_statements
                WHERE (EXTRACT(MONTH FROM created_at) = EXTRACT(MONTH FROM NOW()) - 1
                       AND EXTRACT(YEAR FROM created_at) = EXTRACT(YEAR FROM NOW()))
                    OR (EXTRACT(MONTH FROM created_at) = 12
                        AND EXTRACT(YEAR FROM created_at) = EXTRACT(YEAR FROM NOW()) - 1)
            )
            INSERT INTO banking_accounts_statements (account_id, balance)
            SELECT
                cm.account_id,
                COALESCE(cm.current_balance, 0) + COALESCE(pb.previous_balance, 0) AS balance
            FROM CurrentMonthTransactions cm
            LEFT JOIN PreviousMonthBalance pb
            ON cm.account_id = pb.account_id;
            "#,
        )
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(())
    }
}