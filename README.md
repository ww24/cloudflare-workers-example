# cloudflare-workers-example

Cloudflare Workers Example

## Requirements

- Rust
- [wrangler2](https://github.com/cloudflare/wrangler2)
  - `npm i wrangler --save-dev`

## Development

### Build

```sh
npx wrangler build
```

### Run

#### Set `.dev.vars`

<https://developers.cloudflare.com/workers/wrangler/configuration/#local-environments>

**Do it only the first time.**

```sh
# Generate key-pair
openssl ecparam -name prime256v1 -genkey | openssl pkcs8 -topk8 -nocrypt -out jwt.p8

echo JWT_PRIVATE_KEY="\"$(cat jwt.p8)\"" >> .dev.vars
echo JWT_PUBLIC_KEY="\"$(openssl ec -in jwt.p8 -pubout)\"" >> .dev.vars
```

#### Run on local machine

```sh
npx wrangler dev -l
```
