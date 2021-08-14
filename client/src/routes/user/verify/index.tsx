import { verifyCode } from "api"
import * as React from "react"
import { useLocation } from "react-router-dom"

import Check from "assets/icons/check.svg"
import Error from "assets/icons/error.svg"

export const UserVerify: React.FC = () => {
    const verificationCode = new URLSearchParams(useLocation().search).get("code")

    const [verifySuccess, setVerifySuccess] = React.useState(null)

    React.useEffect(() => {
        verifyCode(verificationCode)
            .then(() => setVerifySuccess(true))
            .catch(() => setVerifySuccess(false))
    }, [])

    if (verifySuccess == null) 
        return <></>

    if (verifySuccess) {
        return <div className="fullpage-info">
            <Check />
            <h2>Account verified</h2>
            <p>Your account was verified. Please login to your account</p>
        </div>
    } else {
        return <div className="fullpage-info">
            <Error />
            <h2>Invalid verification code</h2>
            <p>Invalid or expired verification code was provided</p>
        </div>
    }
}