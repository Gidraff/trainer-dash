import { useEffect, useState } from 'react';
import { Users, Target, TrendingUp } from 'lucide-react';
import { getClients } from '../api';

const Dashboard = () => {
    const [stats, setStats] = useState({ total: 0, activeGoals: 0 });
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const loadStats = async () => {
            try {
                const data = await getClients();
                setStats({
                    total: data.length,
                    activeGoals: data.filter((c: any) => c.goal).length
                });
            } finally {
                setLoading(false);
            }
        };
        loadStats();
    }, []);

    if (loading) return <div className="loading-spinner"></div>;

    return (
        <>
            <div className="page-header">
                <div className="header-content">
                    <h1>Athlete Overview</h1>
                    <p className="subtitle">Manage and track your athletes' performance</p>
                </div>
            </div>

            <div className="stats-grid">
                <div className="stat-card">
                    <div className="stat-icon blue"><Users size={24} /></div>
                    <div className="stat-details">
                        <div className="stat-value">{stats.total}</div>
                        <div className="stat-label">Total Athletes</div>
                    </div>
                </div>

                <div className="stat-card">
                    <div className="stat-icon green"><Target size={24} /></div>
                    <div className="stat-details">
                        <div className="stat-value">{stats.activeGoals}</div>
                        <div className="stat-label">Active Goals</div>
                    </div>
                </div>

                <div className="stat-card">
                    <div className="stat-icon purple"><TrendingUp size={24} /></div>
                    <div className="stat-details">
                        <div className="stat-value">92%</div>
                        <div className="stat-label">Avg. Completion</div>
                    </div>
                </div>
            </div>

            <div className="empty-state">
                <p>Recent activity charts will appear here.</p>
            </div>
        </>
    );
};

export default Dashboard;