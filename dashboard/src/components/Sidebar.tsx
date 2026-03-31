import { LayoutDashboard, ArrowLeftRight, CreditCard, Shield, Users, Heart } from 'lucide-react';

const navItems = [
  { id: 'overview', label: 'Overview', icon: LayoutDashboard },
  { id: 'transactions', label: 'Transactions', icon: ArrowLeftRight },
  { id: 'payments', label: 'Payments', icon: CreditCard },
  { id: 'insurance', label: 'Insurance Policies', icon: Shield },
  { id: 'users', label: 'Users', icon: Users },
];

interface SidebarProps {
  activePage: string;
  onNavigate: (page: string) => void;
}

export default function Sidebar({ activePage, onNavigate }: SidebarProps) {
  return (
    <aside className="w-64 bg-sidebar min-h-screen flex flex-col">
      <div className="p-6 flex items-center gap-3">
        <div className="w-9 h-9 bg-primary rounded-lg flex items-center justify-center">
          <Heart className="w-5 h-5 text-white" />
        </div>
        <span className="text-white text-xl font-bold tracking-tight">Aarokya</span>
      </div>

      <nav className="flex-1 px-3 mt-2">
        {navItems.map((item) => {
          const Icon = item.icon;
          const isActive = activePage === item.id;
          return (
            <button
              key={item.id}
              onClick={() => onNavigate(item.id)}
              className={`w-full flex items-center gap-3 px-4 py-3 rounded-lg mb-1 text-sm font-medium transition-all cursor-pointer ${
                isActive
                  ? 'bg-primary text-white shadow-lg shadow-primary/30'
                  : 'text-indigo-200 hover:bg-sidebar-hover hover:text-white'
              }`}
            >
              <Icon className="w-5 h-5" />
              {item.label}
            </button>
          );
        })}
      </nav>

      <div className="p-4 mx-3 mb-4 bg-sidebar-hover rounded-xl">
        <p className="text-indigo-200 text-xs">Admin Panel</p>
        <p className="text-white text-sm font-medium mt-1">Aarokya Health</p>
        <p className="text-indigo-300 text-xs mt-0.5">v1.0.0</p>
      </div>
    </aside>
  );
}
