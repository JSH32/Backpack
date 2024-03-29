import "../styles/globals.scss"
import { mode } from "@chakra-ui/theme-tools"
import { ChakraProvider, cookieStorageManagerSSR, extendTheme, localStorageManager } from "@chakra-ui/react"
import React from "react"
import App, { AppContext } from "next/app"
import { AppInfoContext } from "helpers/info"
import axios from "axios"
import getConfig from "next/config"
import { AppInfo } from "@/client"
import api from "helpers/api"
import { Store, StoreContext } from "helpers/store"

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

const MyApp = ({ Component, pageProps, appInfo, cookies }: any) => {
  const colorModeManager =
    typeof cookies === "string"
      ? cookieStorageManagerSSR(cookies)
      : localStorageManager

  const [dataStore, setDataStore] = React.useState<Store | null>(null)

  React.useEffect(() => {
    setDataStore(new Store())
  }, [])

  return (
    <AppInfoContext.Provider value={appInfo}>
      <ChakraProvider colorModeManager={colorModeManager} theme={{...theme, colors: { ...theme.colors, primary: theme.colors[appInfo.color] }}}>
        <StoreContext.Provider value={dataStore}>
          <Component {...pageProps} />
        </StoreContext.Provider>
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

  return { ...appProps, appInfo, cookies: appContext.ctx.req?.headers.cookie ?? "" }
}

export default MyApp
