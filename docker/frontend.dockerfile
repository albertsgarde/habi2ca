FROM node:18.19-alpine3.19 AS frontend-build
WORKDIR /habi2ca
COPY ./habi2ca-frontend/package.json ./habi2ca-frontend/package-lock.json ./
RUN npm install --frozen-lockfile

COPY ./habi2ca-frontend/ .
RUN npm run build

FROM node:18.19-alpine3.19 AS frontend-prod
WORKDIR /habi2ca
COPY --from=frontend-build /habi2ca/ ./

ENV BACKEND_ORIGIN=http://localhost:8080

CMD PUBLIC_BACKEND_ORIGIN=${BACKEND_ORIGIN} node build
