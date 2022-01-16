// This is called bpkutil because of issues with node's util definition

/**
 * Convert bytes to string with size unit
 * 
 * @param usage amount of bytes
 * @param decimals decimal places
 * @returns formatted string with unit
 */
export const convertBytes = (usage: number, decimals = 0): string => {
    if (usage === 0) return "0 Bytes"
    const places = decimals < 0 ? 0 : decimals
    const sizes = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"]
    const i = Math.floor(Math.log(usage) / Math.log(1024))
    return parseFloat((usage / Math.pow(1024, i)).toFixed(places)) + " " + sizes[i]
}

/**
 * Convert a date to a formatted string
 * 
 * @param date 
 * @returns formatted string
 */
export const dateToString = (date: Date): string => {
    const ampm = date.getUTCHours() >= 12 ? "PM" : "AM"
    let hours: number = date.getUTCHours() % 12
    hours = hours ? hours : 12
    const minutes = 
        date.getUTCMinutes().toString().length === 1 
        ? "0"+date.getUTCMinutes().toString() 
        : date.getUTCMinutes().toString()

    return `${hours}:${minutes}${ampm} ${date.getUTCMonth() + 1}/${date.getUTCDate()}/${date.getUTCFullYear()}`
}

export const getExtension = (fileName: string): string =>
    fileName.substring(fileName.lastIndexOf(".") + 1).toUpperCase()

export const isExtImage = (ext: string): boolean =>
    ["PNG", "JPG", "JPEG", "GIF", "WEBP", "JFIF", "PJPEG", "PJP"].includes(ext)