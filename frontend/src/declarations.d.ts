export {}

// Images
declare module "*.png"
declare module "*.jpg"
declare module "*.svg"

// Styles
declare module "*.css"
declare module "*.scss"

// Other
declare module "*.wasm"

// Environment variables
declare global {
    interface ImportMeta {
        [env: string]: any
    }
}