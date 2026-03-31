CREATE TABLE insurance_plans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    plan_type VARCHAR(50) NOT NULL,  -- health, life, motor, travel
    insurer_name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    base_premium_annual DECIMAL(12, 2) NOT NULL,
    sum_insured DECIMAL(14, 2) NOT NULL,
    min_age INT NOT NULL DEFAULT 18,
    max_age INT NOT NULL DEFAULT 65,
    coverage_duration_months INT NOT NULL DEFAULT 12,
    waiting_period_days INT NOT NULL DEFAULT 30,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
