import * as React from "react"

import {
  BrowserRouter as Router,
  Route,
  RouteComponentProps,
  Switch,
  useHistory
} from "react-router-dom"

import { Home } from "routes/home"
import { UserCreate } from "routes/user/create"
import { UserLogin } from "routes/user/login"
import { UserVerify } from "routes/user/verify"
import { UploadFiles } from "routes/user/upload"
import { VerificationMessage } from "components/verificationmessage"
import { getUserData } from "api"
import store from "./store"
import { observe } from "mobx"
import { UserTokens } from "routes/user/tokens"
import { Box, useColorModeValue } from "@chakra-ui/react"
import { FileInfo } from "routes/user/upload/fileInfo"
import { UserSettings } from "routes/user/settings"
import { Page } from "components/page"

interface AuthenticatedRouteProps {
    path: string,
    allowUnverified?: boolean,
    component: React.ComponentType<RouteComponentProps<any>> | React.ComponentType<any>
}

const AuthenticatedRoute: React.FC<AuthenticatedRouteProps> = ({ path, component, allowUnverified }) => {
    const history = useHistory()
    const [userData, setUserData] = React.useState(null)
    
    React.useEffect(() => {
        // Since this might be loaded on initial page load MobX async constructor might not be done running
        // Make the HTTP request just in case this is the initial load
        getUserData()
            .then(setUserData)
            .catch(() => history.replace("/user/login"))

        // Watch changes so we can reload this componment and re-evaluate if we should lock the route
        return observe(store, "userData", setUserData)
    }, [])

    // While user data was not loaded just send back nothing
    if (!userData || userData.verified === undefined)
        return <Page></Page>

    // SMTP verification was enabled and the user was not verified
    console.log(userData.verified)
    if (import.meta.env.SNOWPACK_PUBLIC_APP_SMTP_ENABLED && !allowUnverified && !userData.verified)
        return <VerificationMessage email={userData.email}/>

    // User passed all checks, allow them to go to this route
    return <Route path={path} component={component}/>
}

export const App: React.FC = () => {
    return <>
        <Router>
            <Box>
                <Switch>
                    <Route path="/" component={Home} exact/>
                    <Route path="/user/create" component={UserCreate}/>
                    <Route path="/user/login" component={UserLogin}/>
                    {import.meta.env.SNOWPACK_PUBLIC_APP_SMTP_ENABLED ? <Route path="/user/verify" component={UserVerify}/> : null}
                    <AuthenticatedRoute path="/user/settings/:tab" component={UserSettings} allowUnverified/>
                    <AuthenticatedRoute path="/user/settings" component={UserSettings} allowUnverified/>
                    <AuthenticatedRoute path="/user/uploads/:id" component={FileInfo}/>
                    <AuthenticatedRoute path="/user/uploads" component={UploadFiles}/>
                    <AuthenticatedRoute path="/user/tokens" component={UserTokens}/>
                </Switch>
            </Box>
        </Router>
    </>
}