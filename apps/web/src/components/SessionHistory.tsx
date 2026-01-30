import axios from 'axios';
import keycloak from './auth';

const api = axios.create({
    baseURL: 'http://localhost:8080',
});

api.interceptors.request.use(async (config) => {
    try {
        await keycloak.updateToken(30);
    } catch (error) {
        console.error("Failed to refresh token", error);
        keycloak.login();
    }
    if (keycloak.token) { 
        config.headers.Authorization = `Bearer ${keycloak.token}`;
    } 
    return config;
});

export const getClients = async () => {
    const response = await api.get('/trainer/clients');
    return response.data;
};

export const getClientById = async (id: string) => {
    const response = await api.get(`/trainer/clients/${id}`);
    return response.data;
};

export const createClient = async (clientData: { name: string; goal: string; profile: string }) => {
    const response = await api.post('/trainer/clients', clientData);
    return response.data;
};

export const deleteClient = async (id: string) => {
    await api.delete(`/trainer/clients/${id}`);
};

export const getClientSessions = async (clientId: string) => {
    const response = await api.get(`/trainer/clients/${clientId}/sessions`);
    return response.data;
};

export const addSessionFeedback = async (sessionId: string, feedback: { feedback: string; performance_rating: number }) => {
    const response = await api.patch(`/trainer/sessions/${sessionId}/feedback`, feedback);
    return response.data;
};

export { api };