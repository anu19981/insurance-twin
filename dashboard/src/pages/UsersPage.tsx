import { useState } from 'react';
import { Search, Wallet, ArrowLeftRight, Shield } from 'lucide-react';
import { users } from '../data/mockData';

export default function UsersPage() {
  const [search, setSearch] = useState('');

  const filtered = users.filter((user) =>
    user.name.toLowerCase().includes(search.toLowerCase()) ||
    user.phoneNumber.includes(search) ||
    user.email.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Users</h1>
        <p className="text-gray-500 mt-1">Manage all registered users. Phone number is the primary identifier.</p>
      </div>

      <div className="relative max-w-md">
        <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
        <input
          type="text"
          placeholder="Search by name, phone, or email..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-full pl-10 pr-4 py-2.5 border border-gray-200 rounded-xl bg-white text-sm focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary"
        />
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-5">
        {filtered.map((user) => (
          <div key={user.id} className="bg-white rounded-2xl shadow-sm border border-gray-100 p-6 hover:shadow-md transition-shadow">
            <div className="flex items-start justify-between mb-4">
              <div>
                <h3 className="text-lg font-semibold text-gray-900">{user.name}</h3>
                <p className="text-sm font-mono text-primary font-medium mt-0.5">{user.phoneNumber}</p>
                <p className="text-xs text-gray-400 mt-0.5">{user.email}</p>
              </div>
              <div className="w-10 h-10 rounded-full bg-gradient-to-br from-primary to-primary-light flex items-center justify-center text-white font-bold text-sm">
                {user.name.split(' ').map(n => n[0]).join('')}
              </div>
            </div>

            <div className="grid grid-cols-3 gap-3 pt-4 border-t border-gray-100">
              <div className="text-center">
                <div className="flex items-center justify-center mb-1">
                  <Wallet className="w-4 h-4 text-emerald-500" />
                </div>
                <p className="text-sm font-bold text-gray-900">₹{user.walletBalance.toLocaleString()}</p>
                <p className="text-xs text-gray-400">Balance</p>
              </div>
              <div className="text-center">
                <div className="flex items-center justify-center mb-1">
                  <ArrowLeftRight className="w-4 h-4 text-blue-500" />
                </div>
                <p className="text-sm font-bold text-gray-900">{user.totalTransactions}</p>
                <p className="text-xs text-gray-400">Transactions</p>
              </div>
              <div className="text-center">
                <div className="flex items-center justify-center mb-1">
                  <Shield className="w-4 h-4 text-violet-500" />
                </div>
                <p className="text-sm font-bold text-gray-900">{user.activePolicies}</p>
                <p className="text-xs text-gray-400">Policies</p>
              </div>
            </div>

            <p className="text-xs text-gray-400 mt-4">Joined {user.joinedDate}</p>
          </div>
        ))}
      </div>

      {filtered.length === 0 && (
        <div className="text-center py-12 text-gray-400">
          <p className="text-lg">No users found</p>
          <p className="text-sm mt-1">Try adjusting your search</p>
        </div>
      )}
    </div>
  );
}
