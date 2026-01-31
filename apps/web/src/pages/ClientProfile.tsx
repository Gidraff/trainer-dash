import { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { ArrowLeft, Target, TrendingUp, MessageSquare } from 'lucide-react';
import { getClients } from '../api'; // Or your specific getClientById if you have it
import SessionHistory from '../components/SessionHistory';

const ClientProfile = () => {
    const { id } = useParams();
    const navigate = useNavigate();
    const [client, setClient] = useState<any>(null);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        const loadClient = async () => {
            try {
                const allClients = await getClients();
                const found = allClients.find((c: any) => c.id === id);
                setClient(found);
            } finally {
                setLoading(false);
            }
        };
        loadClient();
    }, [id]);

    if (loading) return <div className="loading-screen"><div className="loading-spinner"></div></div>;
    if (!client) return <div className="main-content">Athlete not found.</div>;

    return (
        <>
            <header className="page-header">
                <button onClick={() => navigate(-1)} className="icon-btn" style={{ marginBottom: '1rem' }}>
                    <ArrowLeft size={20} />
                </button>
                <div className="header-content">
                    <h1>{client.name}</h1>
                    <p className="subtitle">Athlete Profile & Performance History</p>
                </div>
            </header>

            <div className="stats-grid">
                <div className="stat-card">
                    <div className="stat-icon blue"><Target size={24} /></div>
                    <div className="stat-details">
                        <div className="stat-value">{client.goal || 'No Goal'}</div>
                        <div className="stat-label">Current Objective</div>
                    </div>
                </div>
                <div className="stat-card">
                    <div className="stat-icon purple"><TrendingUp size={24} /></div>
                    <div className="stat-details">
                        <div className="stat-value">Active</div>
                        <div className="stat-label">Status</div>
                    </div>
                </div>
            </div>

            <section className="client-card" style={{ marginTop: '2rem', padding: '2rem' }}>
                <h3 style={{ marginBottom: '1rem', display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <MessageSquare size={20} color="var(--primary)" /> Coach Notes
                </h3>
                <p className="athlete-notes" style={{ WebkitLineClamp: 'unset', fontSize: '1rem' }}>
                    {client.profile || "No profile notes recorded for this athlete."}
                </p>
            </section>

            <div style={{ marginTop: '2rem' }}>
                <SessionHistory />
            </div>
        </>
    );
};

export default ClientProfile;