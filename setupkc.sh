#!/bin/bash

# FitFlow AI - Automated Keycloak Setup Script
# This script automates the Keycloak realm configuration

set -e  # Exit on any error

echo "╔══════════════════════════════════════════════╗"
echo "║  FitFlow AI - Keycloak Setup Automation     ║"
echo "╚══════════════════════════════════════════════╝"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
KEYCLOAK_URL="http://localhost:8081"
ADMIN_USER="admin"
ADMIN_PASS="admin"
REALM_FILE="trainer-app-realm.json"

# Check if Keycloak is running
echo -n "Checking if Keycloak is running... "
if curl -s -f "${KEYCLOAK_URL}" > /dev/null; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    echo ""
    echo "Error: Keycloak is not accessible at ${KEYCLOAK_URL}"
    echo "Please start it with: docker-compose up -d keycloak"
    exit 1
fi

# Wait for Keycloak to be fully ready
echo -n "Waiting for Keycloak to be ready... "
MAX_ATTEMPTS=30
ATTEMPT=0
while [ $ATTEMPT -lt $MAX_ATTEMPTS ]; do
    if curl -s -f "${KEYCLOAK_URL}/health/ready" > /dev/null 2>&1; then
        echo -e "${GREEN}✓${NC}"
        break
    fi
    ATTEMPT=$((ATTEMPT + 1))
    if [ $ATTEMPT -eq $MAX_ATTEMPTS ]; then
        echo -e "${RED}✗${NC}"
        echo "Timeout waiting for Keycloak to be ready"
        exit 1
    fi
    sleep 2
    echo -n "."
done

# Get admin access token
echo -n "Authenticating with Keycloak... "
TOKEN_RESPONSE=$(curl -s -X POST "${KEYCLOAK_URL}/realms/master/protocol/openid-connect/token" \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d "username=${ADMIN_USER}" \
    -d "password=${ADMIN_PASS}" \
    -d "grant_type=password" \
    -d "client_id=admin-cli")

ACCESS_TOKEN=$(echo $TOKEN_RESPONSE | grep -o '"access_token":"[^"]*' | cut -d'"' -f4)

if [ -z "$ACCESS_TOKEN" ]; then
    echo -e "${RED}✗${NC}"
    echo "Failed to get access token. Check admin credentials."
    exit 1
fi
echo -e "${GREEN}✓${NC}"

# Check if realm already exists
echo -n "Checking if realm already exists... "
REALM_EXISTS=$(curl -s -o /dev/null -w "%{http_code}" \
    "${KEYCLOAK_URL}/admin/realms/trainer-app" \
    -H "Authorization: Bearer ${ACCESS_TOKEN}")

if [ "$REALM_EXISTS" = "200" ]; then
    echo -e "${YELLOW}Realm already exists${NC}"
    read -p "Do you want to delete and recreate it? (y/N): " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo -n "Deleting existing realm... "
        curl -s -X DELETE "${KEYCLOAK_URL}/admin/realms/trainer-app" \
            -H "Authorization: Bearer ${ACCESS_TOKEN}"
        echo -e "${GREEN}✓${NC}"
    else
        echo "Keeping existing realm. Exiting."
        exit 0
    fi
else
    echo -e "${GREEN}No existing realm${NC}"
fi

# Import realm
echo -n "Creating realm from configuration... "
if [ -f "$REALM_FILE" ]; then
    IMPORT_RESPONSE=$(curl -s -o /dev/null -w "%{http_code}" -X POST \
        "${KEYCLOAK_URL}/admin/realms" \
        -H "Authorization: Bearer ${ACCESS_TOKEN}" \
        -H "Content-Type: application/json" \
        -d @"${REALM_FILE}")
    
    if [ "$IMPORT_RESPONSE" = "201" ]; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${RED}✗${NC}"
        echo "Failed to create realm. HTTP status: $IMPORT_RESPONSE"
        exit 1
    fi
else
    echo -e "${RED}✗${NC}"
    echo "Realm configuration file not found: ${REALM_FILE}"
    exit 1
fi

# Verify JWKS endpoint
echo -n "Verifying JWKS endpoint... "
JWKS_RESPONSE=$(curl -s "${KEYCLOAK_URL}/realms/trainer-app/protocol/openid-connect/certs")
if echo "$JWKS_RESPONSE" | grep -q '"keys"'; then
    echo -e "${GREEN}✓${NC}"
else
    echo -e "${RED}✗${NC}"
    echo "JWKS endpoint is not returning expected data"
    exit 1
fi

# Summary
echo ""
echo "╔══════════════════════════════════════════════╗"
echo "║         Setup Completed Successfully!        ║"
echo "╚══════════════════════════════════════════════╝"
echo ""
echo "Keycloak Configuration:"
echo "  • Realm: trainer-app"
echo "  • Client ID: trainer-api"
echo "  • Admin Console: ${KEYCLOAK_URL}"
echo ""
echo "Test Users Created:"
echo "  1. Username: trainer1"
echo "     Password: password123"
echo "     Email: trainer1@fitflow.ai"
echo ""
echo "  2. Username: admin-trainer"
echo "     Password: admin123"
echo "     Email: admin@fitflow.ai"
echo ""
echo "Important URLs:"
echo "  • JWKS: ${KEYCLOAK_URL}/realms/trainer-app/protocol/openid-connect/certs"
echo "  • Auth: ${KEYCLOAK_URL}/realms/trainer-app/protocol/openid-connect/auth"
echo ""
echo "Next Steps:"
echo "  1. Start your API: cargo run"
echo "  2. Start your frontend: npm run dev"
echo "  3. Open browser: http://localhost:5173"
echo "  4. Login with: trainer1 / password123"
echo ""
echo -e "${GREEN}Ready to go! 🚀${NC}"