import * as React from "react"

import {
    Grid, 
    Paper,
    Typography
} from "@material-ui/core"

// import GitHubIcon from "@material-ui/icons/GitHub"
import GoogleIcon from "assets/icons/google.svg"

import { CenteredBox } from "components/centeredbox"

import "./style.scss"

export const UserLogin: React.FC = () => {
    // console.log(GoogleIcon)

    return (
        <div>
            <CenteredBox>
                <Grid item>
                    <Paper id="login-container">
                        <Typography id="title" variant="h4">
                            SIGN IN
                        </Typography>
                        <div id="button-box">
                            <button>
                                <GoogleIcon />
                            </button>

                            <button>
                                <GoogleIcon />
                            </button>
                        </div>
                        <div id="seperator">
                            <hr/>
                            <p>OR</p>
                            <hr/>
                        </div>
                    </Paper>
                </Grid>
            </CenteredBox>
        </div>
    )
}