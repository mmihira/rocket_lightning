FROM node:8.11.3

COPY ./* /app/
WORKDIR /app

RUN npm install yarn
RUN yarn install
RUN yarn build



