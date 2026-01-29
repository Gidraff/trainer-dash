import React, { useState } from 'react';

export const ClientForm = ({ onClientAdded }: { onClientAdded: () => void }) => {
    const [formData, setFormData] = useState({ name: '', goal: '', profile: '' });

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        // Use the api.ts createClient function here
        try {
            await createClient(formData);
            setFormData({ name: '', goal: '', profile: '' });
            onClientAdded();
        } catch (err) {
            alert("Failed to add client. Check console.");
        }
    };

    return (
        <form onSubmit={handleSubmit} style={{ marginBottom: '20px', display: 'flex', gap: '10px' }}>
            <input
                placeholder="Client Name"
                value={formData.name}
                onChange={e => setFormData({ ...formData, name: e.target.value })}
                required
            />
            <input
                placeholder="Goal"
                value={formData.goal}
                onChange={e => setFormData({ ...formData, goal: e.target.value })}
            />
            <button type="submit">Add Real Client</button>
        </form>
    );
};