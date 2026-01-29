import axios from 'axios';
import keycloak from './auth';

const api = axios.create({
    baseURL: 'http://localhost:8080', // Removed trailing slash for consistency
});

api.interceptors.request.use(async (config) => {
    // 1. Ensure token is fresh (refreshes if it will expire in 30 seconds)
    try {
        await keycloak.updateToken(30);
    } catch (error) {
        console.error("Failed to refresh token", error);
        keycloak.login(); // Force login if refresh fails
    }

    // 2. Corrected Logic: If token EXISTS, send it
    if (keycloak.token) { 
        config.headers.Authorization = `Bearer ${keycloak.token}`;
    } 
    
    return config;
});

export const getClients = async () => {
    const response = await api.get('/trainer/clients');
    return response.data;
};

export const createClient = async (clientData: { name: string; goal: string; profile: string }) => {
    const response = await api.post('/trainer/clients', clientData);
    return response.data;
};