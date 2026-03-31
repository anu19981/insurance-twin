#!/bin/bash
set -e

BASE="http://localhost:8080/api/v1"

echo "=========================================="
echo "  Insurance Twin API - Full Test Suite"
echo "=========================================="

# ---- HEALTH CHECK ----
echo -e "\n--- Health Check ---"
curl -s $BASE/../health | jq .

# ---- CUSTOMERS ----
echo -e "\n--- Create Customer: Rajesh Kumar ---"
CUSTOMER=$(curl -s -X POST $BASE/customers \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Rajesh Kumar",
    "email": "rajesh.kumar@gmail.com",
    "phone": "9876543210",
    "date_of_birth": "1990-05-15",
    "gender": "male",
    "pan_number": "ABCDE1234F",
    "aadhaar_number": "123456789012",
    "address_line1": "42, MG Road, Indiranagar",
    "address_line2": "Near Metro Station",
    "city": "Bengaluru",
    "state": "Karnataka",
    "pincode": "560038"
  }')
echo $CUSTOMER | jq .
CUSTOMER_ID=$(echo $CUSTOMER | jq -r '.id')
echo "CUSTOMER_ID=$CUSTOMER_ID"

echo -e "\n--- Create Customer: Sneha Sharma ---"
CUSTOMER2=$(curl -s -X POST $BASE/customers \
  -H "Content-Type: application/json" \
  -d '{
    "full_name": "Sneha Sharma",
    "email": "sneha.sharma@gmail.com",
    "phone": "9123456789",
    "date_of_birth": "1985-11-20",
    "gender": "female",
    "pan_number": "FGHIJ5678K",
    "aadhaar_number": "998877665544",
    "address_line1": "15, Bandra West",
    "city": "Mumbai",
    "state": "Maharashtra",
    "pincode": "400050"
  }')
echo $CUSTOMER2 | jq .
CUSTOMER2_ID=$(echo $CUSTOMER2 | jq -r '.id')
echo "CUSTOMER2_ID=$CUSTOMER2_ID"

echo -e "\n--- List All Customers ---"
curl -s $BASE/customers | jq .

echo -e "\n--- Get Customer by ID ---"
curl -s $BASE/customers/$CUSTOMER_ID | jq .

echo -e "\n--- Update Customer: Rajesh Kumar ---"
curl -s -X PUT $BASE/customers/$CUSTOMER_ID \
  -H "Content-Type: application/json" \
  -d '{
    "phone": "9988776655",
    "address_line1": "100, Koramangala 5th Block",
    "city": "Bengaluru",
    "pincode": "560095"
  }' | jq .

# ---- INSURANCE PLANS ----
echo -e "\n--- Create Plan: Star Health Gold ---"
PLAN=$(curl -s -X POST $BASE/plans \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Star Health Gold",
    "plan_type": "health",
    "insurer_name": "Star Health Insurance",
    "description": "Comprehensive health plan with OPD, pharmacy, lab tests and hospitalization cover for individuals and families",
    "base_premium_annual": 12000.00,
    "sum_insured": 500000.00,
    "min_age": 18,
    "max_age": 65,
    "coverage_duration_months": 12,
    "waiting_period_days": 30
  }')
echo $PLAN | jq .
PLAN_ID=$(echo $PLAN | jq -r '.id')
echo "PLAN_ID=$PLAN_ID"

echo -e "\n--- Create Plan: HDFC Life Protect ---"
PLAN2=$(curl -s -X POST $BASE/plans \
  -H "Content-Type: application/json" \
  -d '{
    "name": "HDFC Life Protect",
    "plan_type": "life",
    "insurer_name": "HDFC Life Insurance",
    "description": "Term life insurance with accidental death and critical illness rider",
    "base_premium_annual": 8500.00,
    "sum_insured": 10000000.00,
    "min_age": 21,
    "max_age": 60,
    "coverage_duration_months": 240,
    "waiting_period_days": 0
  }')
echo $PLAN2 | jq .
PLAN2_ID=$(echo $PLAN2 | jq -r '.id')
echo "PLAN2_ID=$PLAN2_ID"

echo -e "\n--- List Active Plans ---"
curl -s $BASE/plans | jq .

echo -e "\n--- Get Plan by ID ---"
curl -s $BASE/plans/$PLAN_ID | jq .

echo -e "\n--- Update Plan: increase premium ---"
curl -s -X PUT $BASE/plans/$PLAN_ID \
  -H "Content-Type: application/json" \
  -d '{
    "base_premium_annual": 13500.00
  }' | jq .

# ---- PLAN BENEFITS ----
echo -e "\n--- Create Benefit: OPD Consultation (3/year) ---"
BENEFIT_OPD=$(curl -s -X POST $BASE/plan-benefits \
  -H "Content-Type: application/json" \
  -d '{
    "plan_id": "'$PLAN_ID'",
    "benefit_name": "OPD Consultation",
    "benefit_category": "opd",
    "description": "Outpatient doctor consultation at network hospitals and clinics",
    "max_usage_per_year": 3,
    "max_amount_per_usage": 1000.00,
    "max_amount_per_year": 3000.00,
    "is_cashless": true,
    "waiting_period_days": 0
  }')
echo $BENEFIT_OPD | jq .
BENEFIT_OPD_ID=$(echo $BENEFIT_OPD | jq -r '.id')
echo "BENEFIT_OPD_ID=$BENEFIT_OPD_ID"

echo -e "\n--- Create Benefit: Pharmacy (12/year) ---"
BENEFIT_PHARMA=$(curl -s -X POST $BASE/plan-benefits \
  -H "Content-Type: application/json" \
  -d '{
    "plan_id": "'$PLAN_ID'",
    "benefit_name": "Pharmacy",
    "benefit_category": "pharmacy",
    "description": "Prescribed medicines from network pharmacies across India",
    "max_usage_per_year": 12,
    "max_amount_per_usage": 2000.00,
    "max_amount_per_year": 15000.00,
    "is_cashless": true,
    "waiting_period_days": 0
  }')
echo $BENEFIT_PHARMA | jq .
BENEFIT_PHARMA_ID=$(echo $BENEFIT_PHARMA | jq -r '.id')
echo "BENEFIT_PHARMA_ID=$BENEFIT_PHARMA_ID"

echo -e "\n--- Create Benefit: Lab Test (6/year) ---"
BENEFIT_LAB=$(curl -s -X POST $BASE/plan-benefits \
  -H "Content-Type: application/json" \
  -d '{
    "plan_id": "'$PLAN_ID'",
    "benefit_name": "Lab Test",
    "benefit_category": "lab",
    "description": "Diagnostic lab tests including blood work, imaging, and pathology",
    "max_usage_per_year": 6,
    "max_amount_per_usage": 3000.00,
    "max_amount_per_year": 10000.00,
    "is_cashless": true,
    "waiting_period_days": 0
  }')
echo $BENEFIT_LAB | jq .
BENEFIT_LAB_ID=$(echo $BENEFIT_LAB | jq -r '.id')
echo "BENEFIT_LAB_ID=$BENEFIT_LAB_ID"

echo -e "\n--- Create Benefit: Hospitalization ---"
BENEFIT_HOSP=$(curl -s -X POST $BASE/plan-benefits \
  -H "Content-Type: application/json" \
  -d '{
    "plan_id": "'$PLAN_ID'",
    "benefit_name": "Hospitalization",
    "benefit_category": "inpatient",
    "description": "Inpatient hospitalization including room rent, ICU, surgery, and post-op care",
    "max_amount_per_year": 500000.00,
    "is_cashless": true,
    "waiting_period_days": 30
  }')
echo $BENEFIT_HOSP | jq .
BENEFIT_HOSP_ID=$(echo $BENEFIT_HOSP | jq -r '.id')
echo "BENEFIT_HOSP_ID=$BENEFIT_HOSP_ID"

echo -e "\n--- Create Benefit: Maternity ---"
BENEFIT_MAT=$(curl -s -X POST $BASE/plan-benefits \
  -H "Content-Type: application/json" \
  -d '{
    "plan_id": "'$PLAN_ID'",
    "benefit_name": "Maternity Cover",
    "benefit_category": "maternity",
    "description": "Normal and C-section delivery, pre and post natal expenses",
    "max_amount_per_year": 75000.00,
    "is_cashless": true,
    "waiting_period_days": 270
  }')
echo $BENEFIT_MAT | jq .
BENEFIT_MAT_ID=$(echo $BENEFIT_MAT | jq -r '.id')
echo "BENEFIT_MAT_ID=$BENEFIT_MAT_ID"

echo -e "\n--- Create Benefit: Wellness / Annual Checkup ---"
BENEFIT_WELL=$(curl -s -X POST $BASE/plan-benefits \
  -H "Content-Type: application/json" \
  -d '{
    "plan_id": "'$PLAN_ID'",
    "benefit_name": "Annual Health Checkup",
    "benefit_category": "wellness",
    "description": "Preventive annual health checkup at network diagnostic centres",
    "max_usage_per_year": 1,
    "max_amount_per_usage": 5000.00,
    "max_amount_per_year": 5000.00,
    "is_cashless": true,
    "waiting_period_days": 0
  }')
echo $BENEFIT_WELL | jq .

echo -e "\n--- List All Benefits for Plan ---"
curl -s $BASE/plan-benefits/by-plan/$PLAN_ID | jq .

echo -e "\n--- Get Benefit by ID ---"
curl -s $BASE/plan-benefits/$BENEFIT_OPD_ID | jq .

# ---- POLICIES (PURCHASE) ----
echo -e "\n--- Purchase Policy: Rajesh buys Star Health Gold ---"
POLICY=$(curl -s -X POST $BASE/policies \
  -H "Content-Type: application/json" \
  -d '{
    "policy_number": "SHG-2024-000001",
    "customer_id": "'$CUSTOMER_ID'",
    "plan_id": "'$PLAN_ID'",
    "premium_amount": 13500.00,
    "sum_insured": 500000.00,
    "start_date": "2024-04-01",
    "end_date": "2025-03-31",
    "payment_frequency": "annual",
    "auto_renew": true
  }')
echo $POLICY | jq .
POLICY_ID=$(echo $POLICY | jq -r '.id')
echo "POLICY_ID=$POLICY_ID"

echo -e "\n--- Purchase Policy: Sneha buys HDFC Life ---"
POLICY2=$(curl -s -X POST $BASE/policies \
  -H "Content-Type: application/json" \
  -d '{
    "policy_number": "HLP-2024-000001",
    "customer_id": "'$CUSTOMER2_ID'",
    "plan_id": "'$PLAN2_ID'",
    "premium_amount": 8500.00,
    "sum_insured": 10000000.00,
    "start_date": "2024-01-01",
    "end_date": "2044-01-01",
    "payment_frequency": "annual",
    "auto_renew": false
  }')
echo $POLICY2 | jq .
POLICY2_ID=$(echo $POLICY2 | jq -r '.id')
echo "POLICY2_ID=$POLICY2_ID"

echo -e "\n--- List All Policies ---"
curl -s $BASE/policies | jq .

echo -e "\n--- Get Policy by ID ---"
curl -s $BASE/policies/$POLICY_ID | jq .

echo -e "\n--- List Policies by Customer ---"
curl -s $BASE/policies/by-customer/$CUSTOMER_ID | jq .

echo -e "\n--- Update Policy: change to monthly payment ---"
curl -s -X PUT $BASE/policies/$POLICY_ID \
  -H "Content-Type: application/json" \
  -d '{
    "payment_frequency": "monthly"
  }' | jq .

# ---- DEPENDANTS ----
echo -e "\n--- Add Dependant: Spouse (Priya) ---"
DEP_SPOUSE=$(curl -s -X POST $BASE/dependants \
  -H "Content-Type: application/json" \
  -d '{
    "policy_id": "'$POLICY_ID'",
    "full_name": "Priya Kumar",
    "relationship": "spouse",
    "date_of_birth": "1992-08-20",
    "gender": "female",
    "aadhaar_number": "987654321012",
    "is_primary_insured": false
  }')
echo $DEP_SPOUSE | jq .
DEP_SPOUSE_ID=$(echo $DEP_SPOUSE | jq -r '.id')
echo "DEP_SPOUSE_ID=$DEP_SPOUSE_ID"

echo -e "\n--- Add Dependant: Son (Arjun) ---"
DEP_SON=$(curl -s -X POST $BASE/dependants \
  -H "Content-Type: application/json" \
  -d '{
    "policy_id": "'$POLICY_ID'",
    "full_name": "Arjun Kumar",
    "relationship": "son",
    "date_of_birth": "2018-03-10",
    "gender": "male"
  }')
echo $DEP_SON | jq .
DEP_SON_ID=$(echo $DEP_SON | jq -r '.id')
echo "DEP_SON_ID=$DEP_SON_ID"

echo -e "\n--- Add Dependant: Father (Suresh) ---"
DEP_FATHER=$(curl -s -X POST $BASE/dependants \
  -H "Content-Type: application/json" \
  -d '{
    "policy_id": "'$POLICY_ID'",
    "full_name": "Suresh Kumar",
    "relationship": "father",
    "date_of_birth": "1960-01-05",
    "gender": "male",
    "aadhaar_number": "112233445566"
  }')
echo $DEP_FATHER | jq .
DEP_FATHER_ID=$(echo $DEP_FATHER | jq -r '.id')
echo "DEP_FATHER_ID=$DEP_FATHER_ID"

echo -e "\n--- Add Dependant: Mother (Kamala) ---"
DEP_MOTHER=$(curl -s -X POST $BASE/dependants \
  -H "Content-Type: application/json" \
  -d '{
    "policy_id": "'$POLICY_ID'",
    "full_name": "Kamala Kumar",
    "relationship": "mother",
    "date_of_birth": "1963-07-12",
    "gender": "female",
    "aadhaar_number": "665544332211"
  }')
echo $DEP_MOTHER | jq .

echo -e "\n--- List Dependants for Policy ---"
curl -s $BASE/dependants/by-policy/$POLICY_ID | jq .

echo -e "\n--- Get Dependant by ID ---"
curl -s $BASE/dependants/$DEP_SPOUSE_ID | jq .

# ---- NOMINEES ----
echo -e "\n--- Add Nominee: Priya (60% share) ---"
NOM1=$(curl -s -X POST $BASE/nominees \
  -H "Content-Type: application/json" \
  -d '{
    "policy_id": "'$POLICY_ID'",
    "full_name": "Priya Kumar",
    "relationship": "spouse",
    "date_of_birth": "1992-08-20",
    "share_percentage": 60.00,
    "phone": "9876543211",
    "email": "priya.kumar@gmail.com"
  }')
echo $NOM1 | jq .
NOM1_ID=$(echo $NOM1 | jq -r '.id')
echo "NOM1_ID=$NOM1_ID"

echo -e "\n--- Add Nominee: Arjun (40% share) ---"
NOM2=$(curl -s -X POST $BASE/nominees \
  -H "Content-Type: application/json" \
  -d '{
    "policy_id": "'$POLICY_ID'",
    "full_name": "Arjun Kumar",
    "relationship": "son",
    "date_of_birth": "2018-03-10",
    "share_percentage": 40.00
  }')
echo $NOM2 | jq .

echo -e "\n--- Add Nominee for Sneha's policy ---"
curl -s -X POST $BASE/nominees \
  -H "Content-Type: application/json" \
  -d '{
    "policy_id": "'$POLICY2_ID'",
    "full_name": "Amit Sharma",
    "relationship": "spouse",
    "date_of_birth": "1983-04-15",
    "share_percentage": 100.00,
    "phone": "9111222333",
    "email": "amit.sharma@gmail.com"
  }' | jq .

echo -e "\n--- List Nominees for Policy ---"
curl -s $BASE/nominees/by-policy/$POLICY_ID | jq .

echo -e "\n--- Get Nominee by ID ---"
curl -s $BASE/nominees/$NOM1_ID | jq .

# ---- CLAIMS ----
echo -e "\n--- Submit Claim #1: OPD consultation (Rajesh himself) ---"
CLAIM1=$(curl -s -X POST $BASE/claims \
  -H "Content-Type: application/json" \
  -d '{
    "claim_number": "CLM-2024-000001",
    "policy_id": "'$POLICY_ID'",
    "benefit_id": "'$BENEFIT_OPD_ID'",
    "claim_type": "cashless",
    "claim_amount": 800.00,
    "diagnosis_summary": "General physician consultation for seasonal fever and cold",
    "hospital_name": "Apollo Clinic, Indiranagar, Bengaluru"
  }')
echo $CLAIM1 | jq .
CLAIM1_ID=$(echo $CLAIM1 | jq -r '.id')
echo "CLAIM1_ID=$CLAIM1_ID"

echo -e "\n--- Submit Claim #2: Lab test (Rajesh) ---"
CLAIM2=$(curl -s -X POST $BASE/claims \
  -H "Content-Type: application/json" \
  -d '{
    "claim_number": "CLM-2024-000002",
    "policy_id": "'$POLICY_ID'",
    "benefit_id": "'$BENEFIT_LAB_ID'",
    "claim_type": "cashless",
    "claim_amount": 2500.00,
    "diagnosis_summary": "Complete blood count, thyroid panel, and lipid profile",
    "hospital_name": "SRL Diagnostics, Koramangala"
  }')
echo $CLAIM2 | jq .
CLAIM2_ID=$(echo $CLAIM2 | jq -r '.id')
echo "CLAIM2_ID=$CLAIM2_ID"

echo -e "\n--- Submit Claim #3: Hospitalization for dependant (Arjun - son) ---"
CLAIM3=$(curl -s -X POST $BASE/claims \
  -H "Content-Type: application/json" \
  -d '{
    "claim_number": "CLM-2024-000003",
    "policy_id": "'$POLICY_ID'",
    "claimant_dependant_id": "'$DEP_SON_ID'",
    "benefit_id": "'$BENEFIT_HOSP_ID'",
    "claim_type": "cashless",
    "claim_amount": 85000.00,
    "diagnosis_summary": "Dengue fever - 3 day hospitalization with platelet transfusion",
    "hospital_name": "Manipal Hospital, Old Airport Road, Bengaluru",
    "admission_date": "2024-07-10",
    "discharge_date": "2024-07-13"
  }')
echo $CLAIM3 | jq .
CLAIM3_ID=$(echo $CLAIM3 | jq -r '.id')
echo "CLAIM3_ID=$CLAIM3_ID"

echo -e "\n--- Submit Claim #4: Pharmacy reimbursement ---"
CLAIM4=$(curl -s -X POST $BASE/claims \
  -H "Content-Type: application/json" \
  -d '{
    "claim_number": "CLM-2024-000004",
    "policy_id": "'$POLICY_ID'",
    "benefit_id": "'$BENEFIT_PHARMA_ID'",
    "claim_type": "reimbursement",
    "claim_amount": 1450.00,
    "diagnosis_summary": "Prescribed medicines: antibiotics, paracetamol, multivitamins"
  }')
echo $CLAIM4 | jq .
CLAIM4_ID=$(echo $CLAIM4 | jq -r '.id')
echo "CLAIM4_ID=$CLAIM4_ID"

echo -e "\n--- Submit Claim #5: OPD for spouse (Priya) ---"
CLAIM5=$(curl -s -X POST $BASE/claims \
  -H "Content-Type: application/json" \
  -d '{
    "claim_number": "CLM-2024-000005",
    "policy_id": "'$POLICY_ID'",
    "claimant_dependant_id": "'$DEP_SPOUSE_ID'",
    "benefit_id": "'$BENEFIT_OPD_ID'",
    "claim_type": "cashless",
    "claim_amount": 900.00,
    "diagnosis_summary": "Dermatology consultation for skin allergy",
    "hospital_name": "Fortis Clinic, Bannerghatta Road"
  }')
echo $CLAIM5 | jq .
CLAIM5_ID=$(echo $CLAIM5 | jq -r '.id')
echo "CLAIM5_ID=$CLAIM5_ID"

echo -e "\n--- List All Claims ---"
curl -s $BASE/claims | jq .

echo -e "\n--- Get Claim by ID ---"
curl -s $BASE/claims/$CLAIM1_ID | jq .

echo -e "\n--- List Claims by Policy ---"
curl -s $BASE/claims/by-policy/$POLICY_ID | jq .

echo -e "\n--- Approve Claim #1 (OPD) ---"
curl -s -X PUT $BASE/claims/$CLAIM1_ID \
  -H "Content-Type: application/json" \
  -d '{
    "status": "approved",
    "approved_amount": 800.00,
    "reviewed_at": "2024-06-16T10:30:00"
  }' | jq .

echo -e "\n--- Settle Claim #1 (OPD) ---"
curl -s -X PUT $BASE/claims/$CLAIM1_ID \
  -H "Content-Type: application/json" \
  -d '{
    "status": "settled",
    "settled_at": "2024-06-17T14:00:00"
  }' | jq .

echo -e "\n--- Approve Claim #2 (Lab) ---"
curl -s -X PUT $BASE/claims/$CLAIM2_ID \
  -H "Content-Type: application/json" \
  -d '{
    "status": "approved",
    "approved_amount": 2500.00,
    "reviewed_at": "2024-06-18T09:00:00"
  }' | jq .

echo -e "\n--- Approve Claim #3 (Hospitalization) ---"
curl -s -X PUT $BASE/claims/$CLAIM3_ID \
  -H "Content-Type: application/json" \
  -d '{
    "status": "approved",
    "approved_amount": 82000.00,
    "reviewed_at": "2024-07-15T11:00:00"
  }' | jq .

echo -e "\n--- Reject Claim #4 (Pharmacy - missing prescription) ---"
curl -s -X PUT $BASE/claims/$CLAIM4_ID \
  -H "Content-Type: application/json" \
  -d '{
    "status": "rejected",
    "rejection_reason": "Original prescription not attached. Please resubmit with doctor prescription.",
    "reviewed_at": "2024-06-20T15:30:00"
  }' | jq .

echo -e "\n--- Approve Claim #5 (OPD spouse) ---"
curl -s -X PUT $BASE/claims/$CLAIM5_ID \
  -H "Content-Type: application/json" \
  -d '{
    "status": "approved",
    "approved_amount": 900.00,
    "reviewed_at": "2024-06-22T10:00:00"
  }' | jq .

# ---- BENEFIT USAGES ----
echo -e "\n--- Record Usage: OPD visit 1 (Rajesh) ---"
curl -s -X POST $BASE/benefit-usages \
  -H "Content-Type: application/json" \
  -d '{
    "policy_id": "'$POLICY_ID'",
    "benefit_id": "'$BENEFIT_OPD_ID'",
    "claim_id": "'$CLAIM1_ID'",
    "usage_date": "2024-06-15",
    "amount_used": 800.00,
    "notes": "OPD visit 1 of 3 - general physician consultation"
  }' | jq .

echo -e "\n--- Record Usage: OPD visit 2 (Priya - spouse) ---"
curl -s -X POST $BASE/benefit-usages \
  -H "Content-Type: application/json" \
  -d '{
    "policy_id": "'$POLICY_ID'",
    "benefit_id": "'$BENEFIT_OPD_ID'",
    "claim_id": "'$CLAIM5_ID'",
    "usage_date": "2024-06-22",
    "amount_used": 900.00,
    "notes": "OPD visit 2 of 3 - dermatology consultation (spouse)"
  }' | jq .

echo -e "\n--- Record Usage: Lab test ---"
curl -s -X POST $BASE/benefit-usages \
  -H "Content-Type: application/json" \
  -d '{
    "policy_id": "'$POLICY_ID'",
    "benefit_id": "'$BENEFIT_LAB_ID'",
    "claim_id": "'$CLAIM2_ID'",
    "usage_date": "2024-06-18",
    "amount_used": 2500.00,
    "notes": "Lab test 1 of 6 - CBC, thyroid, lipid profile"
  }' | jq .

echo -e "\n--- Record Usage: Hospitalization ---"
curl -s -X POST $BASE/benefit-usages \
  -H "Content-Type: application/json" \
  -d '{
    "policy_id": "'$POLICY_ID'",
    "benefit_id": "'$BENEFIT_HOSP_ID'",
    "claim_id": "'$CLAIM3_ID'",
    "usage_date": "2024-07-13",
    "amount_used": 82000.00,
    "notes": "Hospitalization - dengue treatment for son Arjun"
  }' | jq .

echo -e "\n--- List All Benefit Usages for Policy ---"
curl -s $BASE/benefit-usages/by-policy/$POLICY_ID | jq .

echo -e "\n--- Usage Summary: OPD (2 of 3 used, Rs 1700 of 3000) ---"
curl -s $BASE/benefit-usages/summary/$POLICY_ID/$BENEFIT_OPD_ID | jq .

echo -e "\n--- Usage Summary: Lab Test (1 of 6 used) ---"
curl -s $BASE/benefit-usages/summary/$POLICY_ID/$BENEFIT_LAB_ID | jq .

echo -e "\n--- Usage Summary: Hospitalization ---"
curl -s $BASE/benefit-usages/summary/$POLICY_ID/$BENEFIT_HOSP_ID | jq .

echo -e "\n--- Usage Summary: Pharmacy (0 used - claim was rejected) ---"
curl -s $BASE/benefit-usages/summary/$POLICY_ID/$BENEFIT_PHARMA_ID | jq .

# ---- CLEANUP DEMOS (DELETE) ----
echo -e "\n--- Delete Nominee by ID ---"
curl -s -X DELETE $BASE/nominees/$NOM1_ID -w "\nHTTP Status: %{http_code}\n"

echo -e "\n--- Delete Dependant: Father ---"
curl -s -X DELETE $BASE/dependants/$DEP_FATHER_ID -w "\nHTTP Status: %{http_code}\n"

echo -e "\n--- Delete Benefit: Maternity ---"
curl -s -X DELETE $BASE/plan-benefits/$BENEFIT_MAT_ID -w "\nHTTP Status: %{http_code}\n"

echo -e "\n--- Delete Plan: HDFC Life ---"
curl -s -X DELETE $BASE/plans/$PLAN2_ID -w "\nHTTP Status: %{http_code}\n"

echo -e "\n--- Delete Customer: Sneha ---"
# This will fail if she has policies referencing her (FK constraint) - expected!
curl -s -X DELETE $BASE/customers/$CUSTOMER2_ID -w "\nHTTP Status: %{http_code}\n"

echo -e "\n=========================================="
echo "  Done! All endpoints tested."
echo "=========================================="
