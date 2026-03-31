interface StatusBadgeProps {
  status: string;
}

const statusStyles: Record<string, string> = {
  completed: 'bg-emerald-50 text-emerald-700 ring-emerald-600/20',
  success: 'bg-emerald-50 text-emerald-700 ring-emerald-600/20',
  active: 'bg-emerald-50 text-emerald-700 ring-emerald-600/20',
  pending: 'bg-amber-50 text-amber-700 ring-amber-600/20',
  failed: 'bg-red-50 text-red-700 ring-red-600/20',
  cancelled: 'bg-gray-50 text-gray-700 ring-gray-600/20',
  expired: 'bg-gray-50 text-gray-700 ring-gray-600/20',
  refunded: 'bg-blue-50 text-blue-700 ring-blue-600/20',
};

export default function StatusBadge({ status }: StatusBadgeProps) {
  const style = statusStyles[status] || 'bg-gray-50 text-gray-700 ring-gray-600/20';
  return (
    <span className={`inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold ring-1 ring-inset capitalize ${style}`}>
      {status.replace('_', ' ')}
    </span>
  );
}
