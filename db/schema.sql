CREATE TABLE banking_accounts
(
    id     serial PRIMARY KEY,
    status VARCHAR(20) DEFAULT 'active' NOT NULL
);

CREATE TABLE banking_transactions
(
    id                     BIGSERIAL PRIMARY KEY,
    account_id             INTEGER                                            NOT NULL REFERENCES banking_accounts,
    transaction_type       VARCHAR(20)                                        NOT NULL,
    amount                 BIGINT                                             NOT NULL,
    created_at             TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
    description            TEXT                                               NOT NULL,
    status                 VARCHAR(20)                                        NOT NULL,
    related_transaction_id BIGINT REFERENCES banking_transactions
);

CREATE TABLE banking_accounts_statements
(
    id         BIGSERIAL PRIMARY KEY,
    account_id INTEGER                  NOT NULL REFERENCES banking_accounts,
    balance    BIGINT DEFAULT 5000000   NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL
);


CREATE OR REPLACE VIEW banking_accounts_balance AS
SELECT banking_accounts.id,
       banking_accounts.status,
       COALESCE(banking_accounts_statements.balance, 0) +
       COALESCE(SUM(banking_transactions.amount *
                    CASE WHEN banking_transactions.transaction_type = 'DEPOSIT' THEN 1 ELSE -1 END)::BIGINT,
                0) AS balance
FROM banking_accounts
         LEFT JOIN
     banking_accounts_statements ON banking_accounts.id = banking_accounts_statements.account_id
         LEFT JOIN
     banking_transactions ON banking_accounts.id = banking_transactions.account_id
GROUP BY banking_accounts.id, banking_accounts_statements.balance;


CREATE OR REPLACE FUNCTION transfer(
    p_from_account_id INTEGER,
    p_to_account_id INTEGER,
    p_amount BIGINT,
    p_description VARCHAR
)
    RETURNS TABLE
            (
                amount      BIGINT,
                description VARCHAR,
                status      VARCHAR,
                created_at  TIMESTAMP WITH TIME ZONE
            )
    LANGUAGE plpgsql
AS
$$
DECLARE
    v_from_account_balance BIGINT;
    v_to_account_balance   BIGINT;
    v_status               VARCHAR                  := 'COMPLETE';
    v_now                  TIMESTAMP WITH TIME ZONE := NOW();
    v_transaction_id       BIGINT;
BEGIN
    -- Get the balance for the from_account
    SELECT balance
    INTO v_from_account_balance
    FROM banking_accounts_balance
    WHERE id = p_from_account_id;

    -- Check if from_account exists and has sufficient funds
    IF v_from_account_balance IS NULL THEN
        RAISE 'Account % does not exist', p_from_account_id USING ERRCODE = '22023';
    ELSIF v_from_account_balance < p_amount THEN
        RAISE 'Insufficient funds' USING ERRCODE = '22023';
    END IF;

    -- Get the balance for the to_account
    SELECT balance
    INTO v_to_account_balance
    FROM banking_accounts_balance
    WHERE id = p_to_account_id;

    -- Check if to_account exists
    IF v_to_account_balance IS NULL THEN
        RAISE 'Account % does not exist', p_to_account_id USING ERRCODE = '22023';
    END IF;

    -- Insert transactions for withdrawal from from_account
    INSERT INTO banking_transactions (account_id, transaction_type, amount, description, status)
    VALUES (p_from_account_id, 'WITHDRAWAL', p_amount, p_description, v_status)
    RETURNING id INTO v_transaction_id;

    -- Insert transactions for deposit to to_account with related_transaction_id
    INSERT INTO banking_transactions (account_id, transaction_type, amount, description, status, related_transaction_id)
    VALUES (p_to_account_id, 'DEPOSIT', p_amount, p_description, v_status, v_transaction_id);

    -- Set the return values
    amount := p_amount;
    description := p_description;
    status := v_status;
    created_at := v_now;

    RETURN;
END;
$$;
