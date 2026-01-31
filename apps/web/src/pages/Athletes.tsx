import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { Search, Plus, Edit3, Trash2, Target, Users } from 'lucide-react';
import { getClients, deleteClient } from '../api';
import { toast } from 'react-toastify';
import AddClientForm from '../components/AddClientForm';

const Athletes = () => {
    const [clients, setClients] = useState([]);
    const [loading, setLoading] = useState(true);
    const [searchQuery, setSearchQuery] = useState('');
    const [modal, setModal] = useState<{ show: boolean, client?: any }>({ show: false });

    const loadData = async () => {
        try {
            const data = await getClients();
            setClients(data);
        } catch (err) {
            toast.error("Failed to load athletes.");
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => { loadData(); }, []);

    const handleRemove = async (id: string) => {
        if (!window.confirm("Remove athlete profile? This action cannot be undone.")) return;
        try {
            await deleteClient(id);
            toast.success("Athlete removed successfully");
            loadData();
        } catch {
            toast.error("Failed to delete athlete.");
        }
    };

    const filteredClients = clients.filter((c: any) =>
        c.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        c.goal?.toLowerCase().includes(searchQuery.toLowerCase())
    );

    if (loading) return <div className="loading-screen"><div className="loading-spinner"></div></div>;

    return (
        <>
            <div className="page-header">
                <div className="header-content" style={{ display: 'flex', justifyContent: 'space-between', width: '100%', alignItems: 'center' }}>
                    <div>
                        <h1>My Athletes</h1>
                        <p className="subtitle">Manage and edit your athlete directory</p>
                    </div>
                    <button className="add-btn" onClick={() => setModal({ show: true })}>
                        <Plus size={18} /> New Athlete
                    </button>
                </div>
            </div>

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
            </div>

            {filteredClients.length === 0 ? (
                <div className="empty-state">
                    <div className="empty-icon"><Users size={64} /></div>
                    <h3>No athletes found</h3>
                    <p>Try adjusting your search or add a new athlete.</p>
                </div>
            ) : (
                <section className="client-grid">
                    {filteredClients.map((c: any) => (
                        <div key={c.id} className="client-card">
                            <div className="card-header">
                                <div className="athlete-avatar">{c.name.charAt(0).toUpperCase()}</div>
                                <div className="card-actions">
                                    <button onClick={() => setModal({ show: true, client: c })} className="icon-btn edit"><Edit3 size={16} /></button>
                                    <button onClick={() => handleRemove(c.id)} className="icon-btn delete"><Trash2 size={16} /></button>
                                </div>
                            </div>
                            <div className="card-body">
                                <Link to={`/client/${c.id}`} className="athlete-name">{c.name}</Link>
                                <div className="athlete-goal"><Target size={14} /><span>{c.goal || 'No goal set'}</span></div>
                                {c.profile && <p className="athlete-notes">{c.profile}</p>}
                            </div>
                            <div className="card-footer">
                                <Link to={`/client/${c.id}`} className="view-profile-btn">View Profile →</Link>
                            </div>
                        </div>
                    ))}
                </section>
            )}

            {modal.show && (
                <AddClientForm
                    clientToEdit={modal.client}
                    onClose={() => setModal({ show: false })}
                    onSuccess={() => { loadData(); setModal({ show: false }); }}
                />
            )}
        </>
    );
};

export default Athletes;