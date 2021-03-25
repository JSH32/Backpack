import * as React from "react"
import AppBar from "@material-ui/core/AppBar"
import Toolbar from "@material-ui/core/Toolbar"
import Typography from "@material-ui/core/Typography"
import Button from "@material-ui/core/Button"

import "./style.css"

export const Header: React.FunctionComponent = () => {
    return <div>
        <AppBar position="fixed" color="transparent" elevation={0}>
            <Toolbar>
                <Typography id="bar-title">
                    KAWAII.SH
                </Typography>
                <Button>Login</Button>
            </Toolbar>
        </AppBar>
    </div>
}