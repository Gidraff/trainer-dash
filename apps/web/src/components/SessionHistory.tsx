import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom';
import { Calendar, Activity, MessageSquare } from 'lucide-react';
import { getClientSessions, addSessionFeedback } from '../api'; // Pointing up to api.ts
import { toast } from 'react-toastify';

const SessionHistory = () => {
    const { id } = useParams<{ id: string }>();
    const [sessions, setSessions] = useState<any[]>([]);
    const [loading, setLoading] = useState(true);
    const [feedbackText, setFeedbackText] = useState<{ [key: string]: string }>({});

    const loadSessions = async () => {
        if (!id) return;
        try {
            const data = await getClientSessions(id);
            setSessions(data);
        } catch (err) {
            toast.error("Failed to load session history");
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        loadSessions();
    }, [id]);

    const handleFeedbackSubmit = async (sessionId: string) => {
        try {
            await addSessionFeedback(sessionId, {
                feedback: feedbackText[sessionId],
                performance_rating: 5 // Default rating, could be a slider
            });
            toast.success("Feedback saved");
            loadSessions();
        } catch (err) {
            toast.error("Failed to save feedback");
        }
    };

    if (loading) return <div className="loading-spinner"></div>;

    return (
        <section className="session-history">
            <h2 style={{ marginBottom: '1.5rem', display: 'flex', alignItems: 'center', gap: '10px' }}>
                <Activity size={24} color="var(--primary)" />
                Recent Training Sessions
            </h2>

            {sessions.length === 0 ? (
                <div className="empty-state">
                    <p>No recorded sessions for this athlete yet.</p>
                </div>
            ) : (
                <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
                    {sessions.map((session) => (
                        <div key={session.id} className="stat-card" style={{ flexDirection: 'column', alignItems: 'flex-start' }}>
                            <div style={{ display: 'flex', justifyContent: 'space-between', width: '100%', marginBottom: '1rem' }}>
                                <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                                    <Calendar size={18} color="var(--text-secondary)" />
                                    <span style={{ fontWeight: 600 }}>{new Date(session.date).toLocaleDateString()}</span>
                                </div>
                                <div className="nav-badge" style={{ background: 'var(--success-light)', color: 'var(--success)' }}>
                                    Completed
                                </div>
                            </div>

                            <div className="form-group" style={{ width: '100%', marginBottom: '0' }}>
                                <label style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                                    <MessageSquare size={14} /> Coach Feedback
                                </label>
                                <textarea
                                    className="search-input"
                                    style={{ padding: '10px', height: '80px', marginTop: '8px' }}
                                    placeholder="Add session notes..."
                                    defaultValue={session.feedback}
                                    onChange={(e) => setFeedbackText({ ...feedbackText, [session.id]: e.target.value })}
                                />
                                <button
                                    className="view-profile-btn"
                                    style={{ border: 'none', background: 'none', cursor: 'pointer', marginTop: '10px' }}
                                    onClick={() => handleFeedbackSubmit(session.id)}
                                >
                                    Update Feedback →
                                </button>
                            </div>
                        </div>
                    ))}
                </div>
            )}
        </section>
    );
};

export default SessionHistory;