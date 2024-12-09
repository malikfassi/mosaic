FROM node:18-alpine

WORKDIR /app

# Add dependencies for development
RUN apk add --no-cache libc6-compat

# Install dependencies first (caching)
COPY package*.json ./
RUN npm ci

# Set up for development
ENV NODE_ENV=development
ENV NEXT_TELEMETRY_DISABLED=1

# Expose the port
EXPOSE 3000

# Start development server with host set for Docker
CMD ["npm", "run", "dev", "--", "--host", "0.0.0.0"] 