import * as React from "react"

import { Box, Grid } from "@material-ui/core"

interface Props {
    children: React.ReactNode;
}

export const CenteredBox: React.FC = (props: Props) => {
    return (
        <Box width="100vw" height="100vh">
            <Grid
                container
                justify="center"
                alignItems="center"
                style={{
                    height: "100%",
                    width: "100%"
                }}
            >
                <Box p={10}>
                    {props.children}
                </Box>
            </Grid>
        </Box>
    )
}