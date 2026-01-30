import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import ClientProfile from './pages/ClientProfile';
import { initKeycloak } from './auth';
import { BrowserRouter, Route, Routes } from 'react-router-dom';


initKeycloak(() => {
  createRoot(document.getElementById('root')!).render(
    <StrictMode>
      <BrowserRouter>
      <Routes>
        <Route path="/" element={<App />} />
        <Route path="/client/:id" element={<ClientProfile />} />
      </Routes>
      </BrowserRouter>
    </StrictMode>,
  )
});

