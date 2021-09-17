import * as React from "react"
import "./style.scss"
import UploadIcon from "../../assets/icons/upload.svg"

export const Home: React.FC = () => {
    // File counter
    const [count] = React.useState(420)

    return (
        <div className="centered" id="landing">
            <h1>{import.meta.env.SNOWPACK_PUBLIC_APP_NAME}</h1>
            <h3>{import.meta.env.SNOWPACK_PUBLIC_APP_DESCRIPTION}</h3>
            <h3>{count} <UploadIcon /></h3>
        </div>
    )
}