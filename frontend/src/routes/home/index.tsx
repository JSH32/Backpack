import * as React from "react"

import { Button, Typography } from "@material-ui/core"
import InsertDriveFileIcon from "@material-ui/icons/InsertDriveFile"

import "./style.scss"

export const Home: React.FunctionComponent = () => {
    // File counter
    //const [count, setCount] = React.useState(0)

    return (
        <div>
            <div className="center">
                <div id="header">
                    <Typography variant="h1" color="primary" id="title" >ANOLIS</Typography>
                    <p id="subtitle">A filevault service for all your needs</p>
                </div>
                <div id="counter">
                    <h2>5</h2>
                    <InsertDriveFileIcon/>
                </div>
            </div>
            <div id="bottom-footer">
                <Button href="https://github.com/Riku32/Anolis">GITHUB</Button>
                <Button>PRIVACY POLICY</Button>
                <Button>TERMS OF SERVICE</Button>
            </div>
        </div>
    )
}