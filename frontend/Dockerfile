FROM node:lts-alpine AS deps

WORKDIR /frontend
COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

# Build source code
FROM node:lts-alpine AS builder

ENV NODE_ENV=production
WORKDIR /frontend
COPY . .
COPY --from=deps /frontend/node_modules ./node_modules
RUN yarn build

# Production image, copy all the files and run next
FROM node:lts-alpine AS runner

WORKDIR /frontend
ENV NODE_ENV=production
COPY --from=builder /frontend ./
CMD ["node_modules/.bin/next", "start"]