import React, { useState, useEffect } from 'react';
import { X, User, Target, FileText } from 'lucide-react';
import { createClient, updateClient } from '../api';
import { toast } from 'react-toastify';

interface Props {
    onClose: () => void;
    onSuccess: () => void;
    clientToEdit?: any;
}

const AddClientForm = ({ onClose, onSuccess, clientToEdit }: Props) => {
    const [formData, setFormData] = useState({ name: '', goal: '', profile: '' });
    const [loading, setLoading] = useState(false);

    useEffect(() => {
        if (clientToEdit) {
            setFormData({
                name: clientToEdit.name,
                goal: clientToEdit.goal || '',
                profile: clientToEdit.profile || ''
            });
        }
    }, [clientToEdit]);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setLoading(true);

        try {
            if (clientToEdit) {
                await updateClient(clientToEdit.id, formData);
                toast.success("✅ Athlete profile updated successfully");
            } else {
                await createClient(formData);
                toast.success("✅ New athlete added successfully");
            }
            onSuccess();
        } catch (err) {
            toast.error("❌ Failed to save athlete. Please try again.");
        } finally {
            setLoading(false);
        }
    };

    return (
        <div className="modal-overlay" onClick={onClose}>
            <div className="modal-content" onClick={(e) => e.stopPropagation()}>
                <div className="modal-header">
                    <h2>{clientToEdit ? 'Edit Athlete Profile' : 'Add New Athlete'}</h2>
                    <button onClick={onClose} aria-label="Close modal">
                        <X size={20} />
                    </button>
                </div>

                <form onSubmit={handleSubmit} className="add-form">
                    <div className="form-group">
                        <label htmlFor="name">
                            <User size={16} style={{ display: 'inline', marginRight: '6px' }} />
                            Full Name *
                        </label>
                        <input
                            id="name"
                            type="text"
                            required
                            placeholder="e.g., John Smith"
                            value={formData.name}
                            onChange={e => setFormData({ ...formData, name: e.target.value })}
                            disabled={loading}
                        />
                    </div>

                    <div className="form-group">
                        <label htmlFor="goal">
                            <Target size={16} style={{ display: 'inline', marginRight: '6px' }} />
                            Primary Goal *
                        </label>
                        <input
                            id="goal"
                            type="text"
                            required
                            placeholder="e.g., Build muscle, Lose weight, Marathon training"
                            value={formData.goal}
                            onChange={e => setFormData({ ...formData, goal: e.target.value })}
                            disabled={loading}
                        />
                    </div>

                    <div className="form-group">
                        <label htmlFor="profile">
                            <FileText size={16} style={{ display: 'inline', marginRight: '6px' }} />
                            Notes (Optional)
                        </label>
                        <textarea
                            id="profile"
                            rows={4}
                            placeholder="Add any relevant information about the athlete..."
                            value={formData.profile}
                            onChange={e => setFormData({ ...formData, profile: e.target.value })}
                            disabled={loading}
                        />
                    </div>

                    <button type="submit" className="primary-btn" disabled={loading}>
                        {loading ? 'Saving...' : clientToEdit ? 'Save Changes' : 'Create Athlete Profile'}
                    </button>
                </form>
            </div>
        </div>
    );
};

export default AddClientForm;