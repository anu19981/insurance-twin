import { useState } from 'react';
import Sidebar from './components/Sidebar';
import Overview from './pages/Overview';
import Transactions from './pages/Transactions';
import Payments from './pages/Payments';
import Insurance from './pages/Insurance';
import UsersPage from './pages/UsersPage';

const pages: Record<string, React.FC> = {
  overview: Overview,
  transactions: Transactions,
  payments: Payments,
  insurance: Insurance,
  users: UsersPage,
};

function App() {
  const [activePage, setActivePage] = useState('overview');
  const Page = pages[activePage] || Overview;

  return (
    <div className="flex min-h-screen bg-gray-50/50">
      <Sidebar activePage={activePage} onNavigate={setActivePage} />
      <main className="flex-1 p-8 overflow-auto">
        <Page />
      </main>
    </div>
  );
}

export default App;
