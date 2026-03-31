// @generated automatically by Diesel CLI.

diesel::table! {
    benefit_usages (id) {
        id -> Uuid,
        policy_id -> Uuid,
        benefit_id -> Uuid,
        claim_id -> Nullable<Uuid>,
        usage_date -> Date,
        amount_used -> Numeric,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    claims (id) {
        id -> Uuid,
        #[max_length = 50]
        claim_number -> Varchar,
        policy_id -> Uuid,
        claimant_dependant_id -> Nullable<Uuid>,
        benefit_id -> Uuid,
        #[max_length = 30]
        status -> Varchar,
        #[max_length = 50]
        claim_type -> Varchar,
        claim_amount -> Numeric,
        approved_amount -> Nullable<Numeric>,
        diagnosis_summary -> Nullable<Text>,
        #[max_length = 255]
        hospital_name -> Nullable<Varchar>,
        admission_date -> Nullable<Date>,
        discharge_date -> Nullable<Date>,
        rejection_reason -> Nullable<Text>,
        submitted_at -> Timestamp,
        reviewed_at -> Nullable<Timestamp>,
        settled_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    customers (id) {
        id -> Uuid,
        #[max_length = 255]
        full_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 20]
        phone -> Varchar,
        date_of_birth -> Date,
        #[max_length = 10]
        gender -> Varchar,
        #[max_length = 10]
        pan_number -> Nullable<Varchar>,
        #[max_length = 12]
        aadhaar_number -> Nullable<Varchar>,
        #[max_length = 500]
        address_line1 -> Varchar,
        #[max_length = 500]
        address_line2 -> Nullable<Varchar>,
        #[max_length = 100]
        city -> Varchar,
        #[max_length = 100]
        state -> Varchar,
        #[max_length = 6]
        pincode -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    dependants (id) {
        id -> Uuid,
        policy_id -> Uuid,
        #[max_length = 255]
        full_name -> Varchar,
        #[max_length = 50]
        relationship -> Varchar,
        date_of_birth -> Date,
        #[max_length = 10]
        gender -> Varchar,
        #[max_length = 12]
        aadhaar_number -> Nullable<Varchar>,
        is_primary_insured -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    insurance_plans (id) {
        id -> Uuid,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        plan_type -> Varchar,
        #[max_length = 255]
        insurer_name -> Varchar,
        description -> Text,
        base_premium_annual -> Numeric,
        sum_insured -> Numeric,
        min_age -> Int4,
        max_age -> Int4,
        coverage_duration_months -> Int4,
        waiting_period_days -> Int4,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    nominees (id) {
        id -> Uuid,
        policy_id -> Uuid,
        #[max_length = 255]
        full_name -> Varchar,
        #[max_length = 50]
        relationship -> Varchar,
        date_of_birth -> Date,
        share_percentage -> Numeric,
        #[max_length = 20]
        phone -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    plan_benefits (id) {
        id -> Uuid,
        plan_id -> Uuid,
        #[max_length = 255]
        benefit_name -> Varchar,
        #[max_length = 100]
        benefit_category -> Varchar,
        description -> Text,
        max_usage_per_year -> Nullable<Int4>,
        max_amount_per_usage -> Nullable<Numeric>,
        max_amount_per_year -> Nullable<Numeric>,
        is_cashless -> Bool,
        waiting_period_days -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    policies (id) {
        id -> Uuid,
        #[max_length = 50]
        policy_number -> Varchar,
        customer_id -> Uuid,
        plan_id -> Uuid,
        #[max_length = 30]
        status -> Varchar,
        premium_amount -> Numeric,
        sum_insured -> Numeric,
        start_date -> Date,
        end_date -> Date,
        #[max_length = 20]
        payment_frequency -> Varchar,
        auto_renew -> Bool,
        purchased_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(benefit_usages -> claims (claim_id));
diesel::joinable!(benefit_usages -> plan_benefits (benefit_id));
diesel::joinable!(benefit_usages -> policies (policy_id));
diesel::joinable!(claims -> dependants (claimant_dependant_id));
diesel::joinable!(claims -> plan_benefits (benefit_id));
diesel::joinable!(claims -> policies (policy_id));
diesel::joinable!(dependants -> policies (policy_id));
diesel::joinable!(nominees -> policies (policy_id));
diesel::joinable!(plan_benefits -> insurance_plans (plan_id));
diesel::joinable!(policies -> customers (customer_id));
diesel::joinable!(policies -> insurance_plans (plan_id));

diesel::allow_tables_to_appear_in_same_query!(
    benefit_usages,
    claims,
    customers,
    dependants,
    insurance_plans,
    nominees,
    plan_benefits,
    policies,
);
