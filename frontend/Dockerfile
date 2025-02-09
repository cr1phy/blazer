FROM oven/bun:latest AS builder
WORKDIR /app

COPY package.json bun.lockb tsconfig.json ./

RUN bun install

COPY . .

RUN bun run build


FROM oven/bun:alpine
WORKDIR /app

RUN apk add --no-cache openssl

COPY --from=builder /app ./

EXPOSE 3000

CMD ["bun", "run", "start"]