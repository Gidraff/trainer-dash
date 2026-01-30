import React from 'react';
import { Outlet, Link, useLocation } from 'react-router-dom';
import { LayoutDashboard, Users, Activity, LogOut, TrendingUp } from 'lucide-react';
import { ToastContainer } from 'react-toastify';
import keycloak from '../auth';
import 'react-toastify/dist/ReactToastify.css';

const MainLayout = () => {
    const location = useLocation();

    return (
        <div className="dashboard-layout">
            <ToastContainer theme="dark" position="bottom-right" />

            <aside className="sidebar">
                <div className="sidebar-brand">
                    <Activity className="brand-icon" size={28} />
                    <span>FITFLOW AI</span>
                </div>

                <nav className="sidebar-nav">
                    <Link to="/" className={`nav-link ${location.pathname === '/' ? 'active' : ''}`}>
                        <LayoutDashboard size={20} />
                        <span>Dashboard</span>
                    </Link>
                    <Link to="/athletes" className={`nav-link ${location.pathname === '/athletes' ? 'active' : ''}`}>
                        <Users size={20} />
                        <span>My Athletes</span>
                    </Link>
                    <Link to="#" className="nav-link">
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
                    <button onClick={() => keycloak.logout()} className="logout-btn">
                        <LogOut size={18} />
                        <span>Logout</span>
                    </button>
                </div>
            </aside>

            <main className="main-content">
                <Outlet />
            </main>
        </div>
    );
};

export default MainLayout;