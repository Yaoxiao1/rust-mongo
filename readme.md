Run mongodb by docker
> docker run --name mongodb -d -p 27017:27017 -v $(pwd)/data:/data/db mongodb/mongodb-community-server:$MONGODB_VERSION

Run paperweb
> npm install
> npm start