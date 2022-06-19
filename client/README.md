## Backpack client
This is the backpack client library. This is powered by [openapi-typescript-codegen](https://www.npmjs.com/package/openapi-typescript-codegen) using the backpack OpenAPI schema.

### Usage
```ts
const client = new BackpackClient({
    BASE: 'http://localhost:3000',
    TOKEN: 'Your token here',
})
```