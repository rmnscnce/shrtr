# shrtr
![](https://badgen.net/badge/%F0%9F%A6%80/rust/f74c00?labelColor=ffffff)

A simple link shortening web software for use on [Cloudflare Workers®](https://workers.cloudflare.com/)

## Dependencies
- [`npm`](https://docs.npmjs.com/cli/)
- [`cargo`](https://doc.rust-lang.org/stable/cargo/)
- A Cloudflare account ([create one](https://dash.cloudflare.com/signup))

## Usage
0. Copy/rename `wrangler_example.toml` to `wrangler.toml`
1. Install dependencies with `npm`:
```
npm install
npm install -g wrangler@latest
```
2. Set up the `SHRTR_MAP` Workers KV namespace with `wrangler`:
```
# You will be prompted to log in if you haven't set up `wrangler` before

wrangler kv:namespace create SHRTR_MAP # Use the `id` returned by this command in the `wrangler.toml` file
wrangler kv:namespace create SHRTR_MAP --preview # Use the `preview_id` returned by this command in the `wrangler.toml` file
```
3. Add some data to `SHRTR_MAP` (optional):
```
# Add `--preview` to add data to the `preview` KV namespace (the one used in `wrangler dev`)

wrangler kv:key put --binding SHRTR_MAP "/example" 'https://example.org'
```
4. Run the project locally (optional):
```
wrangler dev
```
5. Deploy the project on Cloudflare Workers:
```
wrangler publish
```
## Notices
Cloudflare, the Cloudflare logo, and Cloudflare Workers are trademarks and/or registered trademarks of Cloudflare, Inc. in the United States and other jurisdictions.