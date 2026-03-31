CREATE TABLE nominees (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    policy_id UUID NOT NULL REFERENCES policies(id) ON DELETE CASCADE,
    full_name VARCHAR(255) NOT NULL,
    relationship VARCHAR(50) NOT NULL,  -- spouse, son, daughter, parent, sibling, other
    date_of_birth DATE NOT NULL,
    share_percentage DECIMAL(5, 2) NOT NULL DEFAULT 100.00,
    phone VARCHAR(20),
    email VARCHAR(255),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_nominees_policy_id ON nominees(policy_id);
