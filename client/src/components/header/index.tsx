import { toJS } from "mobx"
import { observer } from "mobx-react-lite"
import * as React from "react"
import { Link, useHistory } from "react-router-dom"
import store from "../../store"
import "./style.scss"

import UploadIcon from "../../assets/icons/upload.svg"
import SettingsIcon from "../../assets/icons/settings.svg"
import LogOutIcon from "../../assets/icons/log-out.svg"
import KeyIcon from "../../assets/icons/key.svg"
import { logout } from "api"

export const Header: React.FC = () => {
    const history = useHistory()

    const User = observer(() => {
        const onLogout = async() => {
            logout()
                .then(() => {
                    store.setAppInfo(null)
                    history.replace("/")
                })
        }

        const userData = toJS(store.userData)
        return !userData ? <Link to="/user/login">Login</Link> : <div className="dropdown">
            <span className="username">{userData.username}</span>
            <div className="dropdown-container">
                <span className="dropdown-triangle"/>
                <div className="dropdown-content">
                    <div>
                        <UploadIcon />
                        <Link to="/user/uploads">Uploads</Link>
                    </div>
                    <div>
                        <SettingsIcon />
                        <a>Settings</a>
                    </div>
                    <div>
                        <KeyIcon />
                        <a>Tokens</a>
                    </div>
                    <div className="dropdown-content-accent">
                        <LogOutIcon />
                        <a onClick={onLogout}>Log Out</a>
                    </div>
                </div>
            </div>
        </div>
    })

    return (
        <nav>
            <Link to="/">{import.meta.env.SNOWPACK_PUBLIC_APP_NAME}</Link>

            <User />
        </nav>
    )
}
