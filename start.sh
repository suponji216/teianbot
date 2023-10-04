#!/usr/bin/bash
git pull
npm install
npx prisma generate
npm run build
npm run start