#!/usr/bin/env sh

# build the rust code
./build.sh

# build the react code into static html + js that can be served in gh pages
cd frontend

npm run build
npm run export

