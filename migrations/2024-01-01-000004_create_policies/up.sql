CREATE TABLE policies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    policy_number VARCHAR(50) NOT NULL UNIQUE,
    customer_id UUID NOT NULL REFERENCES customers(id),
    plan_id UUID NOT NULL REFERENCES insurance_plans(id),
    status VARCHAR(30) NOT NULL DEFAULT 'active',  -- active, lapsed, cancelled, expired, pending
    premium_amount DECIMAL(12, 2) NOT NULL,
    sum_insured DECIMAL(14, 2) NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL,
    payment_frequency VARCHAR(20) NOT NULL DEFAULT 'annual',  -- monthly, quarterly, half_yearly, annual
    auto_renew BOOLEAN NOT NULL DEFAULT FALSE,
    purchased_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_policies_customer_id ON policies(customer_id);
CREATE INDEX idx_policies_plan_id ON policies(plan_id);
CREATE INDEX idx_policies_policy_number ON policies(policy_number);
