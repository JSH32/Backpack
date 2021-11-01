import { FileData } from "api"
import { dateToString } from "bpkutil"
import * as React from "react"
import "./style.scss"

// All image extensions that can be embedded
const imageExts = ["PNG", "JPG", "JPEG", "GIF", "WEBP", "JFIF", "PJPEG", "PJP"]

export const FileCard: React.FC<{ file: FileData }> = ({ file }) => {
    const ext = file.name.substr(file.name.lastIndexOf(".") + 1).toUpperCase()

    return <div className="file-card">
        { imageExts.includes(ext) ? <>
            <img src={file.url}/>
        </> : <div className="placeholder"><h1>{ext !== "" ? ext : "FILE"}</h1></div> }
        <div className="details">
           <p><b>{file.name}</b></p>
           <p className="date">{dateToString(file.uploaded)}</p>
        </div>
    </div>
}