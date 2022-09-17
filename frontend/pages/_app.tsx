import "../styles/globals.scss"
import { mode } from "@chakra-ui/theme-tools"
import { ChakraProvider, extendTheme } from "@chakra-ui/react"
import React from "react"
import App, { AppContext } from "next/app"
import { AppInfoContext } from "helpers/info"
import axios from "axios"
import getConfig from "next/config"
import { AppInfo } from "@/client"
import api from "helpers/api"

const { serverRuntimeConfig } = getConfig()

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
  let appInfo: AppInfo

  // On server request might not have access to API through same URL (internal networking)
  if (appContext.ctx.req) {
    appInfo = (await axios.get<AppInfo>(`${serverRuntimeConfig.internalApiUrl}/info`)).data
  } else {
    appInfo = await api.server.info()
  }

  return { ...appProps, appInfo }
}

export default MyApp