import { FileData } from "api"
import { dateToString, getExtension, isExtImage } from "bpkutil"
import * as React from "react"
import "./style.scss"

export const FileCard: React.FC<{ file: FileData, onClick: (event: any) => void }> = ({ file, onClick }) => {
    const ext = getExtension(file.name)

    return <div className="file-card" onClick={onClick}>
        { isExtImage(ext) ? <>
            <img src={file.url}/>
        </> : <div className="placeholder"><h1>{ext !== "" ? ext : "FILE"}</h1></div> }
        <div className="details">
           <p><b>{file.name}</b></p>
           <p className="date">{dateToString(file.uploaded)}</p>
        </div>
    </div>
}