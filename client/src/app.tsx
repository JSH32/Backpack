import * as React from "react"

import {
  BrowserRouter as Router,
  Route,
  Switch
} from "react-router-dom"

import { Header } from "components/header"

import { Home } from "routes/home"
import { UserCreate } from "routes/user/create"
import { UserLogin } from "routes/user/login"
import { Footer } from "components/footer"
import { UserVerify } from "routes/user/verify"

export const App: React.FC = () => (
    <>
        <Router>
            <Header/>
            <Switch>
                <Route path="/" component={Home} exact/>
                <Route path="/user/create" component={UserCreate}/>
                <Route path="/user/login" component={UserLogin}/>
                {import.meta.env.SNOWPACK_PUBLIC_APP_SMTP_ENABLED ? <Route path="/user/verify" component={UserVerify}/> : null}
            </Switch>
            <Footer />
        </Router>
    </>
)