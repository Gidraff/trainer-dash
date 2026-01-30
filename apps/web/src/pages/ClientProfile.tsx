import React, { useState } from 'react';
import { X } from 'lucide-react';
import { createClient } from '../api';

const AddClientForm = ({ onClose, onSuccess }: { onClose: () => void, onSuccess: () => void }) => {
    const [formData, setFormData] = useState({ name: '', goal: '', profile: '' });

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        try {
            await createClient(formData);
            onSuccess();
        } catch (err) {
            console.error(err);
        }
    };

    return (
        <div className="modal-overlay">
            <div className="modal-content">
                <div className="modal-header">
                    <h2>Onboard Athlete</h2>
                    <button onClick={onClose} className="icon-btn"><X size={20} /></button>
                </div>
                <form onSubmit={handleSubmit} className="add-form">
                    <div className="form-group">
                        <label>Name</label>
                        <input value={formData.name} onChange={e => setFormData({ ...formData, name: e.target.value })} required />
                    </div>
                    <div className="form-group">
                        <label>Goal</label>
                        <input value={formData.goal} onChange={e => setFormData({ ...formData, goal: e.target.value })} required />
                    </div>
                    <div className="form-group">
                        <label>Profile Notes</label>
                        <textarea value={formData.profile} onChange={e => setFormData({ ...formData, profile: e.target.value })} />
                    </div>
                    <button type="submit" className="primary-btn full-width">Create Athlete</button>
                </form>
            </div>
        </div>
    );
};

export default AddClientForm;