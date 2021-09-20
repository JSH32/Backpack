import * as React from "react"
import "./style.scss"

import GoogleSVG from "assets/icons/google.svg"
import GithubSVG from "assets/icons/github.svg"
import { Link, useHistory } from "react-router-dom"
import { passwordLogin } from "api"
import store from "../../../store"
import { VerificationMessage } from "components/verificationmessage"

export const UserLogin: React.FC = () => {
    const [errorMessage, setErrorMessage] = React.useState(null)
    const [postLoginUnverifiedEmail, setPostLoginUnverifiedEmail] = React.useState(null)
    const history = useHistory()

    const formSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault()
        const form = event.target as HTMLFormElement
        const auth = (form.elements.namedItem("auth") as HTMLInputElement).value
        const password = (form.elements.namedItem("password") as HTMLInputElement).value

        passwordLogin(auth, password)
            .then(userInfo => {
                store.setAppInfo(userInfo)
                if (!userInfo.verified)
                    setPostLoginUnverifiedEmail(userInfo.email)
                else
                    history.replace("/user/uploads")
            })
            .catch(error => setErrorMessage(error.response.data.message))
    }
    
    if (postLoginUnverifiedEmail != null)
        return <VerificationMessage email={postLoginUnverifiedEmail}/>

    return (
        <div className="centered">
            <form id="sign-in" className="card" onSubmit={formSubmit}>
                <h2>Sign in</h2>

                { errorMessage != null ? <p className="error">{errorMessage}</p> : null }

                <button className="github">
                    <GithubSVG />
                    Login with GitHub
                </button>

                <button className="google">
                    <GoogleSVG />
                    Login with Google
                </button>

                <div className="separator">
                    <hr />
                    <span>or</span>
                    <hr />
                </div>

                <input type="text" name="auth" placeholder="Username or Email" required/>
                <input type="password" name="password" placeholder="Password" required/>
                
                <button>Submit</button>

                <p>Don't have an account? <Link to="/user/create">Sign up</Link></p>
            </form>
        </div>
    )
}