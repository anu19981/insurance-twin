CREATE TABLE plan_benefits (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    plan_id UUID NOT NULL REFERENCES insurance_plans(id) ON DELETE CASCADE,
    benefit_name VARCHAR(255) NOT NULL,         -- e.g. "OPD Consultation", "Pharmacy", "Lab Test", "Hospitalization"
    benefit_category VARCHAR(100) NOT NULL,     -- opd, inpatient, pharmacy, lab, wellness, maternity
    description TEXT NOT NULL,
    max_usage_per_year INT,                     -- e.g. 3 for "3 OPD consultations"
    max_amount_per_usage DECIMAL(12, 2),        -- max claimable per use
    max_amount_per_year DECIMAL(12, 2),         -- annual cap
    is_cashless BOOLEAN NOT NULL DEFAULT FALSE,
    waiting_period_days INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_plan_benefits_plan_id ON plan_benefits(plan_id);
