// import { useState } from 'react'
import './App.css'
import React, { useEffect, useState } from 'react';
import { getClients, createClient } from './api';
import keycloak from './auth';

function App() {
  const [clients, setClients] = useState([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadClients();
  }, []);

  const loadClients = async () => {
    try {
      const data = await getClients();
      setClients(data);
    } catch (err) {
      console.error("Failed to load clients", err);
    } finally {
      setLoading(false);
    }
  };

  const handleAddClient = async () => {
    await createClient({
      name: "New Athlete",
      goal: "Marathon Training",
      profile: "Intermediate"
    });
    loadClients(); // Refresh list
  };

  if (loading) return <div>Loading Trainer Dashboard...</div>;

  return (
    <div style={{ padding: '20px' }}>
      <h1>Trainer Dashboard</h1>
      <p>Logged in as: {keycloak.tokenParsed?.preferred_username || keycloak.tokenParsed?.email || 'Unknown User'}</p>
      <button onClick={handleAddClient}>Add Test Client</button>
      <button onClick={() => keycloak.logout()}>Logout</button>

      <h2>My Clients</h2>
      <ul>
        {clients.map((c: any) => (
          <li key={c.id}>
            <strong>{c.name}</strong> - {c.goal}
          </li>
        ))}
      </ul>
    </div>
  )
}

export default App
