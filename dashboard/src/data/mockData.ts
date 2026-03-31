export interface Transaction {
  id: string;
  userId: string;
  userName: string;
  phoneNumber: string;
  amount: number;
  type: 'credit' | 'debit';
  status: 'completed' | 'pending' | 'failed';
  description: string;
  date: string;
}

export interface Payment {
  id: string;
  userId: string;
  userName: string;
  phoneNumber: string;
  amount: number;
  method: 'UPI' | 'card' | 'netbanking' | 'wallet';
  status: 'success' | 'pending' | 'failed' | 'refunded';
  date: string;
  referenceId: string;
}

export interface InsurancePolicy {
  id: string;
  userId: string;
  userName: string;
  phoneNumber: string;
  policyName: string;
  provider: string;
  type: 'health' | 'life' | 'accident' | 'critical_illness';
  premium: number;
  coverAmount: number;
  status: 'active' | 'expired' | 'pending' | 'cancelled';
  startDate: string;
  endDate: string;
}

export interface User {
  id: string;
  name: string;
  phoneNumber: string;
  email: string;
  walletBalance: number;
  totalTransactions: number;
  activePolicies: number;
  joinedDate: string;
}

export const users: User[] = [
  { id: 'U001', name: 'Aarav Sharma', phoneNumber: '+91 98765 43210', email: 'aarav@email.com', walletBalance: 12500, totalTransactions: 34, activePolicies: 2, joinedDate: '2025-06-15' },
  { id: 'U002', name: 'Priya Patel', phoneNumber: '+91 87654 32109', email: 'priya@email.com', walletBalance: 8200, totalTransactions: 22, activePolicies: 1, joinedDate: '2025-08-22' },
  { id: 'U003', name: 'Rohan Gupta', phoneNumber: '+91 76543 21098', email: 'rohan@email.com', walletBalance: 45000, totalTransactions: 56, activePolicies: 3, joinedDate: '2025-03-10' },
  { id: 'U004', name: 'Ananya Reddy', phoneNumber: '+91 65432 10987', email: 'ananya@email.com', walletBalance: 3200, totalTransactions: 12, activePolicies: 1, joinedDate: '2025-11-01' },
  { id: 'U005', name: 'Vikram Singh', phoneNumber: '+91 54321 09876', email: 'vikram@email.com', walletBalance: 28900, totalTransactions: 41, activePolicies: 2, joinedDate: '2025-05-18' },
  { id: 'U006', name: 'Meera Nair', phoneNumber: '+91 43210 98765', email: 'meera@email.com', walletBalance: 15600, totalTransactions: 29, activePolicies: 1, joinedDate: '2025-09-03' },
  { id: 'U007', name: 'Arjun Kumar', phoneNumber: '+91 32109 87654', email: 'arjun@email.com', walletBalance: 7800, totalTransactions: 18, activePolicies: 0, joinedDate: '2025-12-20' },
  { id: 'U008', name: 'Divya Iyer', phoneNumber: '+91 21098 76543', email: 'divya@email.com', walletBalance: 52000, totalTransactions: 63, activePolicies: 4, joinedDate: '2025-01-05' },
];

export const transactions: Transaction[] = [
  { id: 'TXN001', userId: 'U001', userName: 'Aarav Sharma', phoneNumber: '+91 98765 43210', amount: 2500, type: 'debit', status: 'completed', description: 'Insurance premium payment', date: '2026-03-30' },
  { id: 'TXN002', userId: 'U002', userName: 'Priya Patel', phoneNumber: '+91 87654 32109', amount: 5000, type: 'credit', status: 'completed', description: 'Wallet recharge', date: '2026-03-30' },
  { id: 'TXN003', userId: 'U003', userName: 'Rohan Gupta', phoneNumber: '+91 76543 21098', amount: 1200, type: 'debit', status: 'pending', description: 'Policy renewal', date: '2026-03-29' },
  { id: 'TXN004', userId: 'U004', userName: 'Ananya Reddy', phoneNumber: '+91 65432 10987', amount: 800, type: 'debit', status: 'failed', description: 'Premium payment', date: '2026-03-29' },
  { id: 'TXN005', userId: 'U005', userName: 'Vikram Singh', phoneNumber: '+91 54321 09876', amount: 10000, type: 'credit', status: 'completed', description: 'Claim settlement', date: '2026-03-28' },
  { id: 'TXN006', userId: 'U001', userName: 'Aarav Sharma', phoneNumber: '+91 98765 43210', amount: 3500, type: 'credit', status: 'completed', description: 'Wallet recharge', date: '2026-03-28' },
  { id: 'TXN007', userId: 'U006', userName: 'Meera Nair', phoneNumber: '+91 43210 98765', amount: 1500, type: 'debit', status: 'completed', description: 'Insurance premium', date: '2026-03-27' },
  { id: 'TXN008', userId: 'U007', userName: 'Arjun Kumar', phoneNumber: '+91 32109 87654', amount: 4200, type: 'credit', status: 'pending', description: 'Refund processing', date: '2026-03-27' },
  { id: 'TXN009', userId: 'U008', userName: 'Divya Iyer', phoneNumber: '+91 21098 76543', amount: 7500, type: 'debit', status: 'completed', description: 'Annual premium', date: '2026-03-26' },
  { id: 'TXN010', userId: 'U003', userName: 'Rohan Gupta', phoneNumber: '+91 76543 21098', amount: 2000, type: 'credit', status: 'completed', description: 'Cashback reward', date: '2026-03-26' },
  { id: 'TXN011', userId: 'U002', userName: 'Priya Patel', phoneNumber: '+91 87654 32109', amount: 950, type: 'debit', status: 'failed', description: 'Premium payment', date: '2026-03-25' },
  { id: 'TXN012', userId: 'U005', userName: 'Vikram Singh', phoneNumber: '+91 54321 09876', amount: 15000, type: 'credit', status: 'completed', description: 'Insurance claim payout', date: '2026-03-25' },
];

export const payments: Payment[] = [
  { id: 'PAY001', userId: 'U001', userName: 'Aarav Sharma', phoneNumber: '+91 98765 43210', amount: 2500, method: 'UPI', status: 'success', date: '2026-03-30', referenceId: 'REF98765001' },
  { id: 'PAY002', userId: 'U002', userName: 'Priya Patel', phoneNumber: '+91 87654 32109', amount: 5000, method: 'card', status: 'success', date: '2026-03-30', referenceId: 'REF87654002' },
  { id: 'PAY003', userId: 'U003', userName: 'Rohan Gupta', phoneNumber: '+91 76543 21098', amount: 1200, method: 'UPI', status: 'pending', date: '2026-03-29', referenceId: 'REF76543003' },
  { id: 'PAY004', userId: 'U004', userName: 'Ananya Reddy', phoneNumber: '+91 65432 10987', amount: 800, method: 'netbanking', status: 'failed', date: '2026-03-29', referenceId: 'REF65432004' },
  { id: 'PAY005', userId: 'U005', userName: 'Vikram Singh', phoneNumber: '+91 54321 09876', amount: 3200, method: 'wallet', status: 'success', date: '2026-03-28', referenceId: 'REF54321005' },
  { id: 'PAY006', userId: 'U006', userName: 'Meera Nair', phoneNumber: '+91 43210 98765', amount: 1500, method: 'UPI', status: 'success', date: '2026-03-27', referenceId: 'REF43210006' },
  { id: 'PAY007', userId: 'U007', userName: 'Arjun Kumar', phoneNumber: '+91 32109 87654', amount: 4200, method: 'card', status: 'refunded', date: '2026-03-27', referenceId: 'REF32109007' },
  { id: 'PAY008', userId: 'U008', userName: 'Divya Iyer', phoneNumber: '+91 21098 76543', amount: 7500, method: 'netbanking', status: 'success', date: '2026-03-26', referenceId: 'REF21098008' },
  { id: 'PAY009', userId: 'U001', userName: 'Aarav Sharma', phoneNumber: '+91 98765 43210', amount: 3500, method: 'UPI', status: 'success', date: '2026-03-25', referenceId: 'REF98765009' },
  { id: 'PAY010', userId: 'U003', userName: 'Rohan Gupta', phoneNumber: '+91 76543 21098', amount: 6000, method: 'card', status: 'success', date: '2026-03-24', referenceId: 'REF76543010' },
];

export const insurancePolicies: InsurancePolicy[] = [
  { id: 'POL001', userId: 'U001', userName: 'Aarav Sharma', phoneNumber: '+91 98765 43210', policyName: 'Aarokya Health Shield', provider: 'Aarokya Insurance', type: 'health', premium: 12000, coverAmount: 500000, status: 'active', startDate: '2025-06-15', endDate: '2026-06-14' },
  { id: 'POL002', userId: 'U001', userName: 'Aarav Sharma', phoneNumber: '+91 98765 43210', policyName: 'Aarokya Life Secure', provider: 'Aarokya Insurance', type: 'life', premium: 18000, coverAmount: 2500000, status: 'active', startDate: '2025-06-15', endDate: '2035-06-14' },
  { id: 'POL003', userId: 'U002', userName: 'Priya Patel', phoneNumber: '+91 87654 32109', policyName: 'Aarokya Health Basic', provider: 'Aarokya Insurance', type: 'health', premium: 8000, coverAmount: 300000, status: 'active', startDate: '2025-08-22', endDate: '2026-08-21' },
  { id: 'POL004', userId: 'U003', userName: 'Rohan Gupta', phoneNumber: '+91 76543 21098', policyName: 'Aarokya Health Premium', provider: 'Aarokya Insurance', type: 'health', premium: 25000, coverAmount: 1000000, status: 'active', startDate: '2025-03-10', endDate: '2026-03-09' },
  { id: 'POL005', userId: 'U003', userName: 'Rohan Gupta', phoneNumber: '+91 76543 21098', policyName: 'Aarokya Accident Guard', provider: 'Aarokya Insurance', type: 'accident', premium: 5000, coverAmount: 1000000, status: 'active', startDate: '2025-03-10', endDate: '2026-03-09' },
  { id: 'POL006', userId: 'U003', userName: 'Rohan Gupta', phoneNumber: '+91 76543 21098', policyName: 'Aarokya Critical Care', provider: 'Aarokya Insurance', type: 'critical_illness', premium: 15000, coverAmount: 2000000, status: 'pending', startDate: '2026-04-01', endDate: '2027-03-31' },
  { id: 'POL007', userId: 'U004', userName: 'Ananya Reddy', phoneNumber: '+91 65432 10987', policyName: 'Aarokya Health Basic', provider: 'Aarokya Insurance', type: 'health', premium: 8000, coverAmount: 300000, status: 'expired', startDate: '2024-11-01', endDate: '2025-10-31' },
  { id: 'POL008', userId: 'U005', userName: 'Vikram Singh', phoneNumber: '+91 54321 09876', policyName: 'Aarokya Health Shield', provider: 'Aarokya Insurance', type: 'health', premium: 12000, coverAmount: 500000, status: 'active', startDate: '2025-05-18', endDate: '2026-05-17' },
  { id: 'POL009', userId: 'U005', userName: 'Vikram Singh', phoneNumber: '+91 54321 09876', policyName: 'Aarokya Life Secure', provider: 'Aarokya Insurance', type: 'life', premium: 18000, coverAmount: 2500000, status: 'active', startDate: '2025-05-18', endDate: '2035-05-17' },
  { id: 'POL010', userId: 'U006', userName: 'Meera Nair', phoneNumber: '+91 43210 98765', policyName: 'Aarokya Health Basic', provider: 'Aarokya Insurance', type: 'health', premium: 8000, coverAmount: 300000, status: 'active', startDate: '2025-09-03', endDate: '2026-09-02' },
  { id: 'POL011', userId: 'U008', userName: 'Divya Iyer', phoneNumber: '+91 21098 76543', policyName: 'Aarokya Health Premium', provider: 'Aarokya Insurance', type: 'health', premium: 25000, coverAmount: 1000000, status: 'active', startDate: '2025-01-05', endDate: '2026-01-04' },
  { id: 'POL012', userId: 'U008', userName: 'Divya Iyer', phoneNumber: '+91 21098 76543', policyName: 'Aarokya Life Secure', provider: 'Aarokya Insurance', type: 'life', premium: 18000, coverAmount: 2500000, status: 'active', startDate: '2025-01-05', endDate: '2035-01-04' },
  { id: 'POL013', userId: 'U008', userName: 'Divya Iyer', phoneNumber: '+91 21098 76543', policyName: 'Aarokya Critical Care', provider: 'Aarokya Insurance', type: 'critical_illness', premium: 15000, coverAmount: 2000000, status: 'active', startDate: '2025-01-05', endDate: '2026-01-04' },
  { id: 'POL014', userId: 'U008', userName: 'Divya Iyer', phoneNumber: '+91 21098 76543', policyName: 'Aarokya Accident Guard', provider: 'Aarokya Insurance', type: 'accident', premium: 5000, coverAmount: 1000000, status: 'cancelled', startDate: '2025-01-05', endDate: '2026-01-04' },
];

export const dashboardStats = {
  totalUsers: users.length,
  totalTransactions: transactions.length,
  totalRevenue: transactions.filter(t => t.type === 'debit' && t.status === 'completed').reduce((sum, t) => sum + t.amount, 0),
  activePolicies: insurancePolicies.filter(p => p.status === 'active').length,
  pendingPayments: payments.filter(p => p.status === 'pending').length,
  failedTransactions: transactions.filter(t => t.status === 'failed').length,
};
