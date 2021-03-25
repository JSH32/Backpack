import * as React from "react"

import { Button } from "@material-ui/core"
import InsertDriveFileIcon from "@material-ui/icons/InsertDriveFile"

import { getUserData } from "api"

import "./style.scss"

export const Home: React.FunctionComponent = () => {
    // File counter
    const [count, setCount] = React.useState(0)

    return (
        <div>
            <div className="center">
                <div id="header">
                    <h1 id="title">KAWAII.SH</h1>
                    <p id="subtitle">A filevault service for all your needs</p>
                </div>
                <div id="counter">
                    <h2>{count}</h2>
                    <InsertDriveFileIcon/>
                </div>
            </div>
            <div id="bottom-footer">
                <Button>GITHUB</Button>
                <Button>PRIVACY POLICY</Button>
                <Button>TERMS OF SERVICE</Button>
            </div>
        </div>
    )
}