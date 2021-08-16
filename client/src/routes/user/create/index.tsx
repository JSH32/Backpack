import { userCreate } from "api"
import * as React from "react"
import { Link } from "react-router-dom"
import EmailIcon from "assets/icons/email.svg"
import Check from "assets/icons/check.svg"

export const UserCreate: React.FC = () => {
    const [errorMessage, setErrorMessage] = React.useState(null)
    const [emailPostSignup, setEmailPostSignup] = React.useState(null)

    const formSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault()
        const form = event.target as HTMLFormElement
        const username = (form.elements.namedItem("username") as HTMLInputElement).value
        const email = (form.elements.namedItem("email") as HTMLInputElement).value
        const password = (form.elements.namedItem("password") as HTMLInputElement).value

        userCreate(username, email, password)
            .then(() => setEmailPostSignup(email))
            .catch(error => setErrorMessage(error.response.data.message))
    }

    if (emailPostSignup != null) {
        return import.meta.env.SNOWPACK_PUBLIC_APP_SMTP_ENABLED === "true" ? <div className="fullpage-info">
            <EmailIcon />
            <h2>Verify your email</h2>
            <p>An email has been sent to <b>{emailPostSignup}</b>. Please click the link to verify and activate your account</p>
        </div> : <div className="fullpage-info">
            <Check />
            <h2>Account created</h2>
            <p>Your account has been created. Please login to your account</p>
        </div>
    }

    return (
        <form className="card" onSubmit={formSubmit}>
            <h2>Sign up</h2>
            <p>Create a new account</p>

            {errorMessage != null ? <p className="error">{errorMessage}</p> : null}

            <input type="text" name="username" placeholder="Username" minLength={4} maxLength={15} required/>
            <input type="email" name="email" placeholder="Email" required/>
            <input type="password" name="password" placeholder="Password" required/>

            <button>Submit</button>
            <p>Already have an account? <Link to="/user/login">Login</Link></p>
        </form>
    )
}