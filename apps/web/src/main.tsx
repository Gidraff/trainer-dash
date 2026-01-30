import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import keycloak from './auth';
import './App.css';

/**
 * We wrap the render logic in the Keycloak initialization.
 * This ensures the app only loads once the user is authenticated.
 */
const root = ReactDOM.createRoot(document.getElementById('root') as HTMLElement);

// Optional: Loading screen while Keycloak initializes
root.render(
  <div className="loading-screen">
    <div className="loading-spinner"></div>
    <p>Authenticating...</p>
  </div>
);

keycloak.init({
  onLoad: 'login-required',
  checkLoginIframe: false
})
  .then((authenticated) => {
    if (authenticated) {
      // Once authenticated, render the actual App with the Router
      root.render(
        <React.StrictMode>
          <App />
        </React.StrictMode>
      );
    } else {
      // If for some reason authentication fails without throwing an error
      window.location.reload();
    }
  })
  .catch((err) => {
    console.error("Keycloak initialization failed", err);
    root.render(
      <div className="loading-screen">
        <p style={{ color: 'var(--danger)' }}>
          Failed to connect to authentication server.
          Please check your connection and refresh.
        </p>
      </div>
    );
  });