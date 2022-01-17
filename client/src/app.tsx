import * as React from "react"

import {
  BrowserRouter as Router,
  Route,
  RouteComponentProps,
  Switch,
  useHistory
} from "react-router-dom"

import { Header } from "components/header"

import { Home } from "routes/home"
import { UserCreate } from "routes/user/create"
import { UserLogin } from "routes/user/login"
import { Footer } from "components/footer"
import { UserVerify } from "routes/user/verify"
import { UploadFiles } from "routes/user/upload"
import { VerificationMessage } from "components/verificationmessage"
import { getUserData } from "api"
import store from "./store"
import { toJS } from "mobx"
import { UserTokens } from "routes/user/tokens"

interface AuthenticatedRouteProps {
    path: string,
    component: React.ComponentType<RouteComponentProps<any>> | React.ComponentType<any>
}

const AuthenticatedRoute: React.FC<AuthenticatedRouteProps> = ({ path, component }) => {
    const history = useHistory()
    const [userData, setUserData] = React.useState(null)
    
    React.useEffect(() => {
        const mobXUserData = toJS(store.userData)
        if (mobXUserData) {
            setUserData(mobXUserData)
            return
        }

        // Since this might be loaded on initial page load MobX async constructor might not be done running
        // Make the HTTP request just in case this is the initial load
        getUserData()
            .then(setUserData)
            .catch(() => history.replace("/user/login"))
    }, [])

    // While user data was not loaded just send back nothing
    if (!userData) 
        return <></>

    // SMTP verification was enabled and the user was not verified
    if (import.meta.env.SNOWPACK_PUBLIC_APP_SMTP_ENABLED && !userData.verified)
        return <VerificationMessage email={userData.email}/>

    // User passed all checks, allow them to go to this route
    return <Route path={path} component={component}/>
}

export const App: React.FC = () => {
    return <>
        <Router>
            <Header/>
            <Switch>
                <Route path="/" component={Home} exact/>
                <Route path="/user/create" component={UserCreate}/>
                <Route path="/user/login" component={UserLogin}/>
                {import.meta.env.SNOWPACK_PUBLIC_APP_SMTP_ENABLED ? <Route path="/user/verify" component={UserVerify}/> : null}
                <AuthenticatedRoute path="/user/uploads" component={UploadFiles}/>
                <AuthenticatedRoute path="/user/tokens" component={UserTokens}/>
            </Switch>
            <Footer/>
        </Router>
    </>
}