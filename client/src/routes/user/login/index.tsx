import * as React from "react"
import "./style.scss"

import GoogleSVG from "assets/icons/google.svg"
import GithubSVG from "assets/icons/github.svg"
import { Link } from "react-router-dom"
import { passwordLogin } from "api"
import store from "../../../store"
import EmailIcon from "../../../assets/icons/email.svg"

export const UserLogin: React.FC = () => {
    const [errorMessage, setErrorMessage] = React.useState(null)
    const [postLoginUnverifiedEmail, setPostLoginUnverifiedEmail] = React.useState(null)

    const formSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault()
        const form = event.target as HTMLFormElement
        const email = (form.elements.namedItem("email") as HTMLInputElement).value
        const password = (form.elements.namedItem("password") as HTMLInputElement).value

        passwordLogin(email, password)
            .then(userInfo => {
                if (!userInfo.verified) {
                    setPostLoginUnverifiedEmail(userInfo.email)
                } else {
                    store.setAppInfo(userInfo)
                }
            })
            .catch(error => setErrorMessage(error.response.data.message))
    }

    if (postLoginUnverifiedEmail != null) {
        return <div className="fullpage-info">
            <EmailIcon />
            <h2>Verify your email</h2>
            <p>An email was sent previously to <b>{postLoginUnverifiedEmail}</b>. Please click the link to verify and activate your account</p>
            <p>If you did not get a link please click <a href="about:page">here</a></p>
        </div>
    }

    return (
        <form id="sign-in" className="card" onSubmit={formSubmit}>
            <h2>Sign in</h2>

            {errorMessage != null ? <p className="error">{errorMessage}</p> : null}

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

            <input type="text" name="email" placeholder="Email" required/>
            <input type="password" name="password" placeholder="Password" required/>
            
            <button>Submit</button>

            <p>Don't have an account? <Link to="/user/create">Sign up</Link></p>
        </form>
    )
}