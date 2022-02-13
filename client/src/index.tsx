import * as React from "react"
import * as ReactDOM from "react-dom"
import { App } from "./app"
import "./index.scss"

import { ChakraProvider, extendTheme } from "@chakra-ui/react"

const theme = extendTheme()

// Set the primary color to purple
theme.colors.primary = theme.colors.purple

ReactDOM.render(
    <ChakraProvider theme={theme}>
        <App />
    </ChakraProvider>,
    document.getElementById("root")
)
