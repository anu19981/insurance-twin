CREATE TABLE benefit_usages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    policy_id UUID NOT NULL REFERENCES policies(id),
    benefit_id UUID NOT NULL REFERENCES plan_benefits(id),
    claim_id UUID REFERENCES claims(id),
    usage_date DATE NOT NULL DEFAULT CURRENT_DATE,
    amount_used DECIMAL(12, 2) NOT NULL DEFAULT 0,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_benefit_usages_policy_id ON benefit_usages(policy_id);
CREATE INDEX idx_benefit_usages_benefit_id ON benefit_usages(benefit_id);
