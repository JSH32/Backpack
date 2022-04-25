import { useTheme } from "@chakra-ui/react"
import { useAppInfo } from "helpers/info"
import Head from "next/head"
import * as React from "react"

export const Meta: React.FC<{
    title?: string,
    description?: string
}> = ({ title, description }) => {
    const appInfo = useAppInfo()
    const theme = useTheme()

    return <Head>
        <title>{title}</title>
        <meta property="og:title" content={title} key="title"/>
        <meta property="og:type" content="website"/>
        <meta property="og:description" content={description}/>
        <meta property="og:site_name" content={appInfo?.appName}/>
        <meta name="theme-color" content={theme.colors.primary[500]}/>
    </Head>
}