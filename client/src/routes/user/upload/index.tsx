import * as React from "react"
import DatabaseSVG from "assets/icons/database.svg"
import UploadSVG from "assets/icons/upload.svg"
import { FileSearch } from "components/filesearch"
import "./style.scss"

import { getUsage, searchFile, uploadFile } from "api"
import { convertBytes } from "bpkutil"

export const UploadFiles: React.FC = () => {
    const [usage, setUsage] = React.useState<string>()
    const [searchReload, setSearchReload] = React.useState(0)
    
    React.useEffect(() => {
        getUsage()
            .then(bytes => setUsage(convertBytes(bytes)))
    }, [searchReload])

    const shadowUploader = React.useRef(null)
    const uploadButtonCallback = React.useCallback(() => {
        shadowUploader.current?.click()
    }, [shadowUploader])

    const uploadCallback = React.useCallback(async (event: any) => {
        for (const file of event.target.files)
            await uploadFile(file)
            
        setSearchReload(searchReload + 1)
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
        <input type="file" ref={shadowUploader} onChange={uploadCallback} style={{display: "none"}} multiple/>
        <button onClick={uploadButtonCallback}>
            <UploadSVG/>
            <p>Select a file to upload</p>
        </button>
        <FileSearch key={searchReload} onSearch={searchFile}/>
    </div>
}