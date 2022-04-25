import "../styles/globals.scss"
import { mode } from "@chakra-ui/theme-tools"
import { ChakraProvider, extendTheme } from "@chakra-ui/react"
import React from "react"
import App, { AppContext } from "next/app"
import { getAppInfo } from "helpers/api"
import { AppInfoContext } from "helpers/info"

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

const MyApp = ({ Component, pageProps, appInfo }: any) => {
  return (
    <AppInfoContext.Provider value={appInfo}>
      <ChakraProvider theme={{...theme, colors: { ...theme.colors, primary: theme.colors[appInfo.color] }}}>
        <Component {...pageProps} />
      </ChakraProvider>
    </AppInfoContext.Provider>
  )
}

MyApp.getInitialProps = async (appContext: AppContext) => {
  const appProps = await App.getInitialProps(appContext)
  const appInfo = await getAppInfo()
  return { ...appProps, appInfo }
}

export default MyApp