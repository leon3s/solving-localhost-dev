FROM node:20.13.1-slim AS builder

USER node

# Create app directory (with user `node`)
RUN mkdir -p /home/node/app
# Set is as cwd
WORKDIR /home/node/app

# Install app dependencies
# A wildcard is used to ensure both package.json AND package-lock.json are copied
# where available (npm@5+)
COPY --chown=node package*.json ./

# Install dependencies
RUN npm install

ENV NODE_ENV=production
ENV API_URL=https://api.next-hat.com

# Bundle app source code
COPY --chown=node . .

ENV NODE_ENV=production
ENV API_URL=https://api.dev.next-hat.com
ENV AUTH_URL=https://auth.dev.next-hat.com
ENV APP_URL=https://console.dev.next-hat.com

ENV PORT=3000

RUN npm run build

FROM node:20.13.1-slim

WORKDIR /home/node/app

COPY --from=0 /home/node/app/.next /home/node/app/.next
COPY --from=0 /home/node/app/public /home/node/app/public
# COPY --from=0 /home/node/app/server.js /home/node/app/server.js
COPY --from=0 /home/node/app/package.json /home/node/app/package.json

ENV NODE_ENV=production

RUN npm install

EXPOSE 3000

CMD ["npm", "start"]
