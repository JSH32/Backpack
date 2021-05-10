import * as React from "react"
import "./style.scss"
import FileIcon from "../../assets/icons/file.svg"

export const Home: React.FC = () => {
    // File counter
    const [count] = React.useState(420)

    return (
        <div id="landing">
            <h1>ANOLIS</h1>
            <h3>A filevault service for all your needs</h3>
            <h3>{count} <FileIcon /></h3>
        </div>
    )
}
