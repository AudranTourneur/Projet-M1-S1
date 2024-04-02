FROM node:20

WORKDIR /opt/app

COPY package.json /opt/app
COPY package-lock.json /opt/app

RUN npm install

COPY . /opt/app

CMD ["npm", "run", "dev", "--", "--host", "--port", "7777"]
