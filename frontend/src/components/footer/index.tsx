import * as React from "react"
import { Link } from "react-router-dom"
import "./style.css"

export const Footer: React.FC = () => {
    return (
        <footer>
            <Link to="#">GitHub</Link>
            <Link to="#">Privacy Policy</Link>
            <Link to="#">Terms of Service</Link>
        </footer>
    )
}
