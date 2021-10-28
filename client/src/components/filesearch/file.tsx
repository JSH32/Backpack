import { FileData } from "api"
import * as React from "react"
import "./style.scss"

// All image extensions that can be embedded
const imageExts = ["PNG", "JPG", "JPEG", "GIF", "WEBP", "JFIF", "PJPEG", "PJP"]

export const FileCard: React.FC<{ file: FileData }> = ({ file }) => {
    const ext = file.name.substr(file.name.lastIndexOf(".") + 1).toUpperCase()

    return <div className="file-card">
        { imageExts.includes(ext) ? <>
            <img src={file.url}/>
        </> : <h1 className="placeholder">{ext}</h1> }
        <div className="details">
           <p><b>{file.name}</b></p>
           <p className="date">{dateToString(file.uploaded)}</p>
        </div>
    </div>
}

// Convert a date to a nicely formatted string
const dateToString = (date: Date): string => {
    const ampm = date.getUTCHours() >= 12 ? "PM" : "AM"
    let hours: number = date.getUTCHours() % 12
    hours = hours ? hours : 12
    const minutes = 
        date.getUTCMinutes().toString().length === 1 
        ? "0"+date.getUTCMinutes().toString() 
        : date.getUTCMinutes().toString()

    return `${hours}:${minutes}${ampm} ${date.getDay()}/${date.getUTCMonth()}/${date.getUTCFullYear()}`
}