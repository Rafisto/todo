# pull official base image
FROM node:13.12.0-alpine

WORKDIR /app

COPY . .

ENV PATH /app/node_modules/.bin:$PATH

RUN npm install

CMD ["npm", "start"]