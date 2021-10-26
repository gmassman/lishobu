FROM node:14-alpine as base

WORKDIR /app
COPY package.json package-lock.json /app/
COPY . /app
EXPOSE 3000 

FROM base AS development

# livereload port
ENV NODE_ENV=development
RUN npm install
CMD ["npm", "run", "dev"]

FROM base AS production

ENV NODE_ENV=production
RUN npm install --production
RUN npm run build

CMD ["npm", "start"]

