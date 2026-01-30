import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import MainLayout from './layouts/MainLayout';
import Dashboard from './pages/Dashboard';
import Athletes from './pages/Athletes';
import ClientProfile from './pages/ClientProfile';

const App = () => {
  return (
    <Router>
      <Routes>
        <Route element={<MainLayout />}>
          {/* Default view is the Dashboard Overview */}
          <Route path="/" element={<Dashboard />} />
          {/* Management view for the Athlete List */}
          <Route path="/athletes" element={<Athletes />} />
          {/* Detail view for a specific athlete */}
          <Route path="/client/:id" element={<ClientProfile />} />

          {/* Catch-all redirect to Dashboard */}
          <Route path="*" element={<Navigate to="/" replace />} />
        </Route>
      </Routes>
    </Router>
  );
};

export default App;