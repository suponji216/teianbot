FROM node:20
WORKDIR /app

COPY --chown=bot:bot prisma /app/prisma
COPY --chown=bot:bot src /app/src
COPY --chown=bot:bot .sapphirerc.yml /app/
COPY --chown=bot:bot package*.json /app/
COPY --chown=bot:bot tsconfig.json /app/

RUN npm install && npx prisma generate && npm run build

ENTRYPOINT [ "npm", "run", "start" ]