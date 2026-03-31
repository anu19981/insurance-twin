CREATE TABLE claims (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    claim_number VARCHAR(50) NOT NULL UNIQUE,
    policy_id UUID NOT NULL REFERENCES policies(id),
    claimant_dependant_id UUID REFERENCES dependants(id),  -- NULL if policyholder themselves
    benefit_id UUID NOT NULL REFERENCES plan_benefits(id),
    status VARCHAR(30) NOT NULL DEFAULT 'submitted',  -- submitted, under_review, approved, rejected, settled, withdrawn
    claim_type VARCHAR(50) NOT NULL,       -- cashless, reimbursement
    claim_amount DECIMAL(12, 2) NOT NULL,
    approved_amount DECIMAL(12, 2),
    diagnosis_summary TEXT,
    hospital_name VARCHAR(255),
    admission_date DATE,
    discharge_date DATE,
    rejection_reason TEXT,
    submitted_at TIMESTAMP NOT NULL DEFAULT NOW(),
    reviewed_at TIMESTAMP,
    settled_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_claims_policy_id ON claims(policy_id);
CREATE INDEX idx_claims_claim_number ON claims(claim_number);
