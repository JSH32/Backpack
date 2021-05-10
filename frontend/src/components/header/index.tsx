import * as React from "react"
import AppBar from "@material-ui/core/AppBar"
import Toolbar from "@material-ui/core/Toolbar"
import Typography from "@material-ui/core/Typography"
import Button from "@material-ui/core/Button"

import "./style.scss"

export const Header: React.FC = () => {
    return <div>
        <AppBar position="fixed" color="transparent" elevation={0}>
            <Toolbar>
                <Typography id="bar-title">
                    ANOLIS
                </Typography>
                <Button>Login</Button>
            </Toolbar>
        </AppBar>
    </div>
}