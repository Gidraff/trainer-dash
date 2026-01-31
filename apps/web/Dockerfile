FROM node:24-alpine as build

WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
RUN npm run build

# The second stage uses Nginx to serve the 'dist' folder
FROM nginx:alpine
COPY --from=build /app/dist /usr/share/nginx/html
# We don't need to COPY the nginx.conf inside the Dockerfile 
# because we will mount it via docker-compose for easier updates.
EXPOSE 80
EXPOSE 443