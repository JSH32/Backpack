import * as React from "react"
import * as ReactDOM from "react-dom"
import { App } from "./app"
import "./index.scss"
import { mode } from "@chakra-ui/theme-tools"


import { ChakraProvider, extendTheme } from "@chakra-ui/react"

const theme = extendTheme({
    fonts: {
        heading: "Greycliff CF, sans-serif",
        body: "Greycliff CF, sans-serif"
    },
    styles: {
        global: (props: any) => ({
            html: {
                bg: "gray.800"
            },
            body: {
                bg: mode("gray.50", "gray.800")(props),
                WebkitTapHighlightColor: "transparent"
            }
        })
    }
})

// Set the primary color to purple
theme.colors.primary = theme.colors.purple

ReactDOM.render(
    <ChakraProvider theme={theme}>
        <App />
    </ChakraProvider>,
    document.getElementById("root")
)
