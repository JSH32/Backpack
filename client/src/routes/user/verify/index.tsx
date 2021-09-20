import * as React from "react"
import { verify } from "api"
import { useLocation } from "react-router-dom"

import Check from "assets/icons/check.svg"
import Error from "assets/icons/error.svg"
import store from "../../../store"
import { toJS } from "mobx"

export const UserVerify: React.FC = () => {
    const verificationCode = new URLSearchParams(useLocation().search).get("code")

    const [verifySuccess, setVerifySuccess] = React.useState(null)

    React.useEffect(() => {
        verify(verificationCode)
            .then(() => {
                setVerifySuccess(true)

                const userData = toJS(store.userData)
                if (userData)
                    store.setAppInfo({ ...userData, verified: true })
            })
            .catch(() => setVerifySuccess(false))
    }, [])

    // While verification is pending
    if (verifySuccess == null) return <></>

    return <div className="centered">
        <div className="fullpage-info">
            { verifySuccess ? <>
                <Check />
                <h2>Account verified</h2>
                <p>Your account was verified. { toJS(store.userData) ? "You may now access your account" : "Please login to access your account" }</p>
            </> : <>
                <Error />
                <h2>Invalid verification code</h2>
                <p>Invalid or expired verification code was provided</p>
            </> }
        </div>
    </div>
}