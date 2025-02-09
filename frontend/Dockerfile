FROM oven/bun:latest AS builder
WORKDIR /app

COPY package.json bun.lockb tsconfig.json ./

RUN bun install

COPY . .

RUN bun run build


FROM oven/bun:runtime-alpine:latest
WORKDIR /app

COPY --from=builder /app ./

EXPOSE 3000

CMD ["bun", "run", "start"]