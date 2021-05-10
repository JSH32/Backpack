// Images
declare module "*.png"
declare module "*.jpg"

// SVG
declare module '*.svg' {
    const ref: React.FC
    export default ref
}

// Styles
declare module "*.css"
declare module "*.scss"

// Other
declare module "*.wasm"

// ImportMeta
interface ImportMeta {
    [env: string]: any;
}