import * as React from "react"
import DatabaseSVG from "assets/icons/database.svg"
import UploadSVG from "assets/icons/upload.svg"
import InfoSVG from "assets/icons/info.svg"
import { FileSearch } from "components/filesearch"
import "./style.scss"

import { getUsage, searchFile, uploadFile, deleteFile } from "api"
import { convertBytes } from "bpkutil"

export const UploadFiles: React.FC = () => {
    const [usage, setUsage] = React.useState<string>()
    const [searchReload, setSearchReload] = React.useState(0)
    const [currentUploading, setCurrentUploading] = React.useState(0)
    
    React.useEffect(() => {
        getUsage()
            .then(bytes => setUsage(convertBytes(bytes)))
    }, [searchReload])

    const shadowUploader = React.useRef(null)
    const uploadButtonCallback = React.useCallback(() => {
        shadowUploader.current?.click()
    }, [shadowUploader])

    const uploadCallback = React.useCallback((event: any) => {
        const uploadPromises = []

        for (const file of event.target.files) {
            setCurrentUploading(count => count + 1)
            uploadPromises.push(uploadFile(file)
                .finally(() => {
                    setCurrentUploading(count => count - 1)
                }))
        }

        Promise.allSettled(uploadPromises)
            .then(() => setSearchReload(searchReload + 1))
    }, [searchReload])

    return <div id="upload" className="page">
        <h1 className="title">FILES</h1>
        <hr className="divider"/>
        <div className="data-label">
            <DatabaseSVG/>
            <span>USAGE</span>
            <hr/>
            <span>{usage}</span>
        </div>
        {currentUploading > 0 ? <div className="notification-info">
            <InfoSVG/>
            <p>Currently uploading {currentUploading} files</p>
        </div> : <></>}
        <input type="file" ref={shadowUploader} onChange={uploadCallback} style={{display: "none"}} multiple/>
        <FileSearch key={searchReload} onSearch={searchFile} onDelete={deleteFile}/>
        <a className="floating-button" onClick={uploadButtonCallback}>
            <UploadSVG/>
        </a>
    </div>
}