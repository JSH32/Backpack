import * as React from "react"
import EmailIcon from "../assets/icons/email.svg"

export const VerificationMessage: React.FC<{ email: string }> = ({ email }) => {
    return <div className="centered">
        <div className="fullpage-info">
            <EmailIcon />
            <h2>Verify your email</h2>
            <p>An email was sent previously to <b>{email}</b>. Please click the link to verify and activate your account</p>
            <p>If you did not get a link please click <a href="about:page">here</a></p>
        </div>
    </div>
}