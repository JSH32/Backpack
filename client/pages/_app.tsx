import "../styles/globals.scss"
import { mode } from "@chakra-ui/theme-tools"
import { ChakraProvider, extendTheme } from "@chakra-ui/react"
import React from "react"
import { AppInfo } from "helpers/api"
import App from "next/app"
import axios from "axios"
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

MyApp.getInitialProps = async (appContext: any): Promise<any> => {
  const appProps = await App.getInitialProps(appContext)
  
  const appInfo = (await axios.get<AppInfo>(`${process.env.NEXT_PUBLIC_API_URL}/info`)).data

  return { ...appProps, appInfo }
}

export default MyApp