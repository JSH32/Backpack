import * as React from "react"
import "./style.scss"

import GoogleSVG from "assets/icons/google.svg"
import GithubSVG from "assets/icons/github.svg"

export const UserLogin: React.FC = () => {
    return (
        <form id="sign-in" onSubmit={e => e.preventDefault()}>
            <h2>Sign in</h2>

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

            <input type="text" placeholder="Username" />
            <input type="password" placeholder="Password" />
            
            <button>Submit</button>
        </form>
    )
}