CREATE SCHEMA IF NOT EXISTS banking;
SET search_path TO banking;

create table accounts
(
    id           serial
        primary key,
    status       varchar(20) default 'active'::character varying not null
);

create table transactions
(
    id                     bigserial
        primary key,
    account_id integer                                                        not null
        references accounts,
    transaction_type       varchar(20)                                        not null,
    amount                 bigint                                             not null,
    created_at             timestamp with time zone default CURRENT_TIMESTAMP not null,
    description            text                                               not null,
    status                 varchar(20)                                        not null,
    related_transaction_id bigint
        constraint transactions_transactions__fk
            references transactions
);

create table accounts_statements
(
    id           bigserial
        primary key,
    account_id integer             not null
        references accounts,
    balance bigint default 5000000 not null,
    created_at   timestamp with time zone not null
);

create view accounts_balance as
select
    accounts.id,
    accounts.status,
    coalesce(statements.balance, 0) +
    coalesce(sum(transactions.amount * case when transactions.transaction_type = 'DEPOSIT' then 1 else -1 end)::bigint, 0) -- potentially dangerous casting to bigint if transactions for a given account overflow.
        as balance
from accounts
         left join accounts_statements statements on accounts.id = statements.account_id
         left join transactions on accounts.id = transactions.account_id
group by accounts.id, statements.balance;



CREATE OR REPLACE FUNCTION banking.transfer(
    p_from_account_id INTEGER,
    p_to_account_id INTEGER,
    p_amount BIGINT,
    p_description VARCHAR
)
    RETURNS TABLE (
                      amount BIGINT,
                      description VARCHAR,
                      status VARCHAR,
                      created_at TIMESTAMP WITH TIME ZONE
                  )
    LANGUAGE plpgsql AS $$
DECLARE
    v_from_account_balance BIGINT;
    v_to_account_balance BIGINT;
    v_status VARCHAR := 'COMPLETE';
    v_now TIMESTAMP WITH TIME ZONE := NOW();
    v_transaction_id BIGINT;
BEGIN
    -- Get the balance for the from_account
    SELECT balance INTO v_from_account_balance
    FROM banking.accounts_balance
    WHERE id = p_from_account_id;

    -- Check if from_account exists and has sufficient funds
    IF v_from_account_balance IS NULL THEN
        RAISE 'Account % does not exist', p_from_account_id USING ERRCODE = '22023';
    ELSIF v_from_account_balance < p_amount THEN
        RAISE 'Insufficient funds' USING ERRCODE = '22023';
    END IF;

    -- Get the balance for the to_account
    SELECT balance INTO v_to_account_balance
    FROM banking.accounts_balance
    WHERE id = p_to_account_id;

    -- Check if to_account exists
    IF v_to_account_balance IS NULL THEN
        RAISE 'Account % does not exist', p_to_account_id USING ERRCODE = '22023';
    END IF;

    -- Insert transactions for withdrawal from from_account
    INSERT INTO banking.transactions (account_id, transaction_type, amount, description, status)
    VALUES
        (p_from_account_id, 'WITHDRAWAL', p_amount, p_description, v_status)
    RETURNING id INTO v_transaction_id;

    -- Insert transactions for deposit to to_account with related_transaction_id
    INSERT INTO banking.transactions (account_id, transaction_type, amount, description, status, related_transaction_id)
    VALUES
        (p_to_account_id, 'DEPOSIT', p_amount, p_description, v_status, v_transaction_id);

    -- Set the return values
    amount := p_amount;
    description := p_description;
    status := v_status;
    created_at := v_now;

    RETURN;
END;
$$;