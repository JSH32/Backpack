import "../styles/globals.scss"
import type { AppProps } from "next/app"
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

theme.colors.primary = theme.colors.purple

export default ({ Component, pageProps }: AppProps) => {
  return (
    <ChakraProvider theme={theme}>
      <Component {...pageProps} />
    </ChakraProvider>
  )
}
