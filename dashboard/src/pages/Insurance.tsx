import { useState } from 'react';
import { Search } from 'lucide-react';
import StatusBadge from '../components/StatusBadge';
import { insurancePolicies } from '../data/mockData';

const typeLabels: Record<string, string> = {
  health: 'Health',
  life: 'Life',
  accident: 'Accident',
  critical_illness: 'Critical Illness',
};

const typeStyles: Record<string, string> = {
  health: 'bg-emerald-50 text-emerald-700',
  life: 'bg-blue-50 text-blue-700',
  accident: 'bg-orange-50 text-orange-700',
  critical_illness: 'bg-purple-50 text-purple-700',
};

export default function Insurance() {
  const [search, setSearch] = useState('');
  const [statusFilter, setStatusFilter] = useState<string>('all');
  const [typeFilter, setTypeFilter] = useState<string>('all');

  const filtered = insurancePolicies.filter((pol) => {
    const matchesSearch =
      pol.userName.toLowerCase().includes(search.toLowerCase()) ||
      pol.phoneNumber.includes(search) ||
      pol.policyName.toLowerCase().includes(search.toLowerCase()) ||
      pol.id.toLowerCase().includes(search.toLowerCase());
    const matchesStatus = statusFilter === 'all' || pol.status === statusFilter;
    const matchesType = typeFilter === 'all' || pol.type === typeFilter;
    return matchesSearch && matchesStatus && matchesType;
  });

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-gray-900">Insurance Policies</h1>
        <p className="text-gray-500 mt-1">Manage and view all user insurance policies.</p>
      </div>

      <div className="flex flex-col sm:flex-row gap-4">
        <div className="relative flex-1">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
          <input
            type="text"
            placeholder="Search by name, phone, policy name, or ID..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-full pl-10 pr-4 py-2.5 border border-gray-200 rounded-xl bg-white text-sm focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary"
          />
        </div>
        <select
          value={typeFilter}
          onChange={(e) => setTypeFilter(e.target.value)}
          className="px-4 py-2.5 border border-gray-200 rounded-xl bg-white text-sm focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary"
        >
          <option value="all">All Types</option>
          <option value="health">Health</option>
          <option value="life">Life</option>
          <option value="accident">Accident</option>
          <option value="critical_illness">Critical Illness</option>
        </select>
        <select
          value={statusFilter}
          onChange={(e) => setStatusFilter(e.target.value)}
          className="px-4 py-2.5 border border-gray-200 rounded-xl bg-white text-sm focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary"
        >
          <option value="all">All Status</option>
          <option value="active">Active</option>
          <option value="expired">Expired</option>
          <option value="pending">Pending</option>
          <option value="cancelled">Cancelled</option>
        </select>
      </div>

      <div className="bg-white rounded-2xl shadow-sm border border-gray-100 overflow-hidden">
        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="bg-gray-50/80">
                <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-4">Policy ID</th>
                <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-4">User</th>
                <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-4">Phone</th>
                <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-4">Policy Name</th>
                <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-4">Type</th>
                <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-4">Premium</th>
                <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-4">Cover</th>
                <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-4">Status</th>
                <th className="text-left text-xs font-semibold text-gray-500 uppercase tracking-wider px-6 py-4">Validity</th>
              </tr>
            </thead>
            <tbody className="divide-y divide-gray-50">
              {filtered.map((pol) => (
                <tr key={pol.id} className="hover:bg-gray-50/50 transition-colors">
                  <td className="px-6 py-4 text-sm font-mono font-medium text-primary">{pol.id}</td>
                  <td className="px-6 py-4 text-sm font-medium text-gray-900">{pol.userName}</td>
                  <td className="px-6 py-4 text-sm font-mono text-gray-500">{pol.phoneNumber}</td>
                  <td className="px-6 py-4 text-sm text-gray-700 font-medium">{pol.policyName}</td>
                  <td className="px-6 py-4">
                    <span className={`inline-flex items-center text-xs font-semibold px-2.5 py-0.5 rounded-full ${typeStyles[pol.type]}`}>
                      {typeLabels[pol.type]}
                    </span>
                  </td>
                  <td className="px-6 py-4 text-sm text-gray-900">₹{pol.premium.toLocaleString()}/yr</td>
                  <td className="px-6 py-4 text-sm font-semibold text-gray-900">₹{(pol.coverAmount / 100000).toFixed(0)}L</td>
                  <td className="px-6 py-4"><StatusBadge status={pol.status} /></td>
                  <td className="px-6 py-4 text-xs text-gray-500">
                    {pol.startDate}<br />to {pol.endDate}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
        {filtered.length === 0 && (
          <div className="text-center py-12 text-gray-400">
            <p className="text-lg">No policies found</p>
            <p className="text-sm mt-1">Try adjusting your search or filters</p>
          </div>
        )}
      </div>
    </div>
  );
}
