import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { getClients, deleteClient } from './api';
import keycloak from './auth';
import { 
  LayoutDashboard, 
  Users, 
  LogOut, 
  Plus, 
  Trash2, 
  Edit3,
  Search,
  TrendingUp,
  Activity,
  Target
} from 'lucide-react';
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import AddClientForm from './components/AddClientForm';
import './App.css';

const App = () => {
  const [clients, setClients] = useState([]);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState('');
  const [modal, setModal] = useState<{ show: boolean, client?: any }>({ show: false });

  const load = async () => {
    try {
      const data = await getClients();
      setClients(data);
    } catch (err) {
      toast.error("Failed to load dashboard.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { load(); }, []);

  const handleRemove = async (id: string) => {
    if (!window.confirm("Remove athlete profile? This action cannot be undone.")) return;
    try {
      await deleteClient(id);
      toast.success("Athlete removed successfully");
      load();
    } catch {
      toast.error("Failed to delete athlete. Please try again.");
    }
  };

  const filteredClients = clients.filter((c: any) =>
    c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
    c.goal?.toLowerCase().includes(searchQuery.toLowerCase())
  );

  if (loading) {
    return (
      <div className="loading-screen">
        <div className="loading-spinner"></div>
        <p>Loading FitFlow...</p>
      </div>
    );
  }

  const activeGoals = clients.filter((c: any) => c.goal).length;

  return (
    <div className="dashboard-layout">
      <ToastContainer theme="dark" position="bottom-right" />

      {/* Sidebar */}
      <aside className="sidebar">
        <div className="sidebar-brand">
          <Activity className="brand-icon" size={28} />
          <span>FITFLOW AI</span>
        </div>
        
        <nav className="sidebar-nav">
          <Link to="/" className="nav-link active">
            <LayoutDashboard size={20} />
            <span>Dashboard</span>
          </Link>
          <Link to="/athletes" className="nav-link">
            <Users size={20} />
            <span>My Athletes</span>
            <span className="nav-badge">{clients.length}</span>
          </Link>
          <Link to="/analytics" className="nav-link">
            <TrendingUp size={20} />
            <span>Analytics</span>
          </Link>
        </nav>

        <div className="sidebar-footer">
          <div className="user-profile">
            <div className="user-avatar">
              {keycloak.tokenParsed?.name?.charAt(0).toUpperCase() || 'T'}
            </div>
            <div className="user-info">
              <div className="user-name">{keycloak.tokenParsed?.name || 'Trainer'}</div>
              <div className="user-role">Coach</div>
            </div>
          </div>
          <button 
            onClick={() => keycloak.logout()} 
            className="logout-btn"
          >
            <LogOut size={18} />
            <span>Logout</span>
          </button>
        </div>
      </aside>

      {/* Main Content */}
      <main className="main-content">
        {/* Header */}
        <div className="page-header">
          <div className="header-content">
            <h1>Athlete Overview</h1>
            <p className="subtitle">Manage and track your athletes' performance</p>
          </div>
        </div>

        {/* Stats Cards */}
        <div className="stats-grid">
          <div className="stat-card">
            <div className="stat-icon blue">
              <Users size={24} />
            </div>
            <div className="stat-details">
              <div className="stat-value">{clients.length}</div>
              <div className="stat-label">Total Athletes</div>
            </div>
          </div>
          
          <div className="stat-card">
            <div className="stat-icon green">
              <Target size={24} />
            </div>
            <div className="stat-details">
              <div className="stat-value">{activeGoals}</div>
              <div className="stat-label">Active Goals</div>
            </div>
          </div>
          
          <div className="stat-card">
            <div className="stat-icon purple">
              <TrendingUp size={24} />
            </div>
            <div className="stat-details">
              <div className="stat-value">92%</div>
              <div className="stat-label">Avg. Completion</div>
            </div>
          </div>
        </div>

        {/* Search and Actions Bar */}
        <div className="actions-bar">
          <div className="search-box">
            <Search size={20} className="search-icon" />
            <input
              type="text"
              placeholder="Search athletes by name or goal..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="search-input"
            />
          </div>
          <button className="add-btn" onClick={() => setModal({ show: true })}>
            <Plus size={18} />
            New Athlete
          </button>
        </div>

        {/* Athletes Grid */}
        {filteredClients.length === 0 ? (
          <div className="empty-state">
            <div className="empty-icon">
              <Users size={64} />
            </div>
            <h3>No athletes found</h3>
            <p>
              {searchQuery 
                ? "Try adjusting your search criteria" 
                : "Get started by adding your first athlete"}
            </p>
            {!searchQuery && (
              <button className="add-btn" onClick={() => setModal({ show: true })}>
                <Plus size={18} />
                Add Your First Athlete
              </button>
            )}
          </div>
        ) : (
          <section className="client-grid">
            {filteredClients.map((c: any) => (
              <div key={c.id} className="client-card">
                <div className="card-header">
                  <div className="athlete-avatar">
                    {c.name.charAt(0).toUpperCase()}
                  </div>
                  <div className="card-actions">
                    <button 
                      onClick={() => setModal({ show: true, client: c })} 
                      className="icon-btn edit"
                      title="Edit athlete"
                    >
                      <Edit3 size={16} />
                    </button>
                    <button 
                      onClick={() => handleRemove(c.id)} 
                      className="icon-btn delete"
                      title="Delete athlete"
                    >
                      <Trash2 size={16} />
                    </button>
                  </div>
                </div>
                
                <div className="card-body">
                  <Link to={`/client/${c.id}`} className="athlete-name">
                    {c.name}
                  </Link>
                  <div className="athlete-goal">
                    <Target size={14} />
                    <span>{c.goal || 'No goal set'}</span>
                  </div>
                  {c.profile && (
                    <p className="athlete-notes">{c.profile}</p>
                  )}
                </div>
                
                <div className="card-footer">
                  <Link to={`/client/${c.id}`} className="view-profile-btn">
                    View Profile →
                  </Link>
                </div>
              </div>
            ))}
          </section>
        )}

        {/* Modal */}
        {modal.show && (
          <AddClientForm
            clientToEdit={modal.client}
            onClose={() => setModal({ show: false })}
            onSuccess={() => { load(); setModal({ show: false }); }}
          />
        )}
      </main>
    </div>
  );
};

export default App;