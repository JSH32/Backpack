import TimeAgo from "javascript-time-ago"
import en from "javascript-time-ago/locale/en.json"
TimeAgo.addLocale(en)

// Global timeAgo instance
// NOTE: When adding multiple language support this will need to become another context
export const timeAgo = new TimeAgo("en_US")

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
            ? "0" + date.getUTCMinutes().toString()
            : date.getUTCMinutes().toString()

    return `${hours}:${minutes}${ampm} ${date.getUTCMonth() + 1}/${date.getUTCDate()}/${date.getUTCFullYear()}`
}

export const getExtension = (fileName: string): string =>
    fileName.substring(fileName.lastIndexOf(".") + 1).toUpperCase()

export const isExtImage = (ext: string): boolean =>
    ["PNG", "JPG", "JPEG", "GIF", "WEBP", "JFIF", "PJPEG", "PJP"].includes(ext)

export const copyText = (text: string) => {
    const tempInput = document.createElement("input")
    tempInput.value = text
    document.body.appendChild(tempInput)
    tempInput.select()
    document.execCommand("copy")
    document.body.removeChild(tempInput)
}
