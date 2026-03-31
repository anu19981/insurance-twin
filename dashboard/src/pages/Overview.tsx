import { Users, ArrowLeftRight, IndianRupee, Shield, Clock, AlertTriangle } from 'lucide-react';
import StatCard from '../components/StatCard';
import StatusBadge from '../components/StatusBadge';
import { dashboardStats, transactions, payments } from '../data/mockData';

export default function Overview() {
  const recentTransactions = transactions.slice(0, 5);
  const recentPayments = payments.slice(0, 5);

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Dashboard Overview</h1>
        <p className="text-gray-500 mt-1">Welcome back. Here's what's happening today.</p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-5">
        <StatCard title="Total Users" value={dashboardStats.totalUsers} icon={Users} trend="12% from last month" trendUp color="indigo" />
        <StatCard title="Total Transactions" value={dashboardStats.totalTransactions} icon={ArrowLeftRight} trend="8% from last month" trendUp color="blue" />
        <StatCard title="Total Revenue" value={`₹${dashboardStats.totalRevenue.toLocaleString()}`} icon={IndianRupee} trend="5% from last month" trendUp color="emerald" />
        <StatCard title="Active Policies" value={dashboardStats.activePolicies} icon={Shield} trend="3 new this week" trendUp color="violet" />
        <StatCard title="Pending Payments" value={dashboardStats.pendingPayments} icon={Clock} color="amber" />
        <StatCard title="Failed Transactions" value={dashboardStats.failedTransactions} icon={AlertTriangle} color="red" />
      </div>

      <div className="grid grid-cols-1 xl:grid-cols-2 gap-6">
        <div className="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
          <div className="px-6 py-5 border-b border-gray-100">
            <h2 className="text-lg font-semibold text-gray-900">Recent Transactions</h2>
          </div>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="bg-gray-50/50">
                  <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-3">ID</th>
                  <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-3">User</th>
                  <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-3">Amount</th>
                  <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-3">Status</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-50">
                {recentTransactions.map((txn) => (
                  <tr key={txn.id} className="hover:bg-gray-50/50 transition-colors">
                    <td className="px-6 py-4 text-sm font-mono text-gray-600">{txn.id}</td>
                    <td className="px-6 py-4">
                      <p className="text-sm font-medium text-gray-900">{txn.userName}</p>
                      <p className="text-xs text-gray-500">{txn.phoneNumber}</p>
                    </td>
                    <td className={`px-6 py-4 text-sm font-semibold ${txn.type === 'credit' ? 'text-emerald-600' : 'text-gray-900'}`}>
                      {txn.type === 'credit' ? '+' : '-'}₹{txn.amount.toLocaleString()}
                    </td>
                    <td className="px-6 py-4"><StatusBadge status={txn.status} /></td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        <div className="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
          <div className="px-6 py-5 border-b border-gray-100">
            <h2 className="text-lg font-semibold text-gray-900">Recent Payments</h2>
          </div>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="bg-gray-50/50">
                  <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-3">User</th>
                  <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-3">Amount</th>
                  <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-3">Method</th>
                  <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-3">Status</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-50">
                {recentPayments.map((pay) => (
                  <tr key={pay.id} className="hover:bg-gray-50/50 transition-colors">
                    <td className="px-6 py-4">
                      <p className="text-sm font-medium text-gray-900">{pay.userName}</p>
                      <p className="text-xs text-gray-500">{pay.phoneNumber}</p>
                    </td>
                    <td className="px-6 py-4 text-sm font-semibold text-gray-900">₹{pay.amount.toLocaleString()}</td>
                    <td className="px-6 py-4 text-sm text-gray-600 uppercase">{pay.method}</td>
                    <td className="px-6 py-4"><StatusBadge status={pay.status} /></td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}
