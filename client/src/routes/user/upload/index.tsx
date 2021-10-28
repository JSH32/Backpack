import * as React from "react"
import DatabaseSVG from "assets/icons/database.svg"
import { FileSearch } from "components/filesearch"
import "./style.scss"

import { searchFile } from "api"

export const UploadFiles: React.FC = () => {
    return <div id="upload" className="page">
        <h1 className="title">FILES</h1>
        <hr className="divider"/>
        <div className="data-label">
            <DatabaseSVG/>
            <span>USAGE</span>
            <hr/>
            <span>25GB</span>
        </div>

        <FileSearch onSearch={searchFile}/>
    </div>
}