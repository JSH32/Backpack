import { getUserData, UserData } from "helpers/api"
import store from "helpers/store"
import { observe } from "mobx"
import Router from "next/router"
import * as React from "react"
import { Page } from "layouts/Page"
import { VerificationMessage } from "./VerificationMessage"

export const Authenticated: React.FC<{
    allowUnverified?: boolean,
    children: React.ReactNode
}> = ({ allowUnverified, children }) => {
    const [userData, setUserData] = React.useState<UserData | null>(store.userData || null)
    
    React.useEffect(() => {
        // Since this might be loaded on initial page load MobX async constructor might not be done running
        // Make the HTTP request just in case this is the initial load
        getUserData()
            .then(setUserData)
            .catch(() => Router.replace("/user/login"))

        // Watch changes so we can reload this componment and re-evaluate if we should lock the route
        return observe(store, "userData", data => setUserData(data.newValue as UserData))
    }, [])
    
    // While user data was not loaded just send back nothing
    if (!userData || userData.verified === undefined)
        return <Page></Page>

    // SMTP verification was enabled and the user was not verified
    if (process.env.NEXT_PUBLIC_APP_SMTP_ENABLED && !allowUnverified && !userData.verified)
        return <VerificationMessage email={userData.email}/>

    return <>{children}</>
}