# Fetching the latest node image on alpine linux
FROM node:latest

# Setting up the work directory
WORKDIR /usr/dir/frontend

# Installing dependencies
COPY ./package.json /usr/dir/frontend
RUN npm install

# Copying all the files in our project
COPY . .

# Starting our application
CMD npm start