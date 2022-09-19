import Router from "next/router"
import * as React from "react"
import { Page } from "layouts/Page"
import { VerificationMessage } from "./VerificationMessage"
import { useAppInfo } from "helpers/info"
import { UserData } from "@/client"
import api from "helpers/api"
import { useStore } from "helpers/store"

export const Authenticated: React.FC<{
    allowUnverified?: boolean,
    children: React.ReactNode
}> = ({ allowUnverified, children }) => {
    const appInfo = useAppInfo()
    const store = useStore()
    const [userData, setUserData] = React.useState<UserData | null>(store?.userData || null)
    
    React.useEffect(() => {
        // Since this might be loaded on initial page load MobX async constructor might not be done running
        // Make the HTTP request just in case this is the initial load
        api.user.info()
            .then(data => {
                setUserData(data)
            })
            .catch(() => {
                Router.replace("/user/login")
            })
    }, [store])
    
    return <>
        {!userData || userData.verified === undefined ? 
            // User data wasn't loaded, render empty page.
            <Page></Page> 
        : appInfo?.smtp && !allowUnverified && !userData.verified ? 
            // SMTP verification was enabled and the user was not verified
            <VerificationMessage email={userData.email}/> 
        :
            // Everything is fine. Load page.
            <Page>{children}</Page>
        }
    </>
}