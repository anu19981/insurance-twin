CREATE TABLE dependants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    policy_id UUID NOT NULL REFERENCES policies(id) ON DELETE CASCADE,
    full_name VARCHAR(255) NOT NULL,
    relationship VARCHAR(50) NOT NULL,  -- spouse, son, daughter, father, mother, father_in_law, mother_in_law
    date_of_birth DATE NOT NULL,
    gender VARCHAR(10) NOT NULL,
    aadhaar_number VARCHAR(12),
    is_primary_insured BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_dependants_policy_id ON dependants(policy_id);
