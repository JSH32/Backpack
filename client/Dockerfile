FROM node:lts-alpine AS deps

WORKDIR /client
COPY package.json yarn.lock ./
RUN yarn install --frozen-lockfile

# Build source code
FROM node:lts-alpine AS builder

ENV NODE_ENV=production
WORKDIR /client
COPY . .
COPY --from=deps /client/node_modules ./node_modules
RUN yarn build

# Production image, copy all the files and run next
FROM node:lts-alpine AS runner

WORKDIR /client
ENV NODE_ENV=production
COPY --from=builder /client ./
CMD ["node_modules/.bin/next", "start"]