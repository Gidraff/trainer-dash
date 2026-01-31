import Keycloak from "keycloak-js";


const keycloak = new Keycloak({
    url: 'http://localhost:8081',
    realm: 'trainer-app',
    clientId: 'trainer-api',
});

export const initKeycloak = (onAuthenticated: () => void) => {
    keycloak.init({ 
        onLoad: 'login-required', 
        checkLoginIframe: false,
        // redirectUri: 'https://localhost/' 
    }).then((authenticated) => {
        if (authenticated) {
            onAuthenticated();
        }
    }).catch(console.error);
};

export default keycloak;