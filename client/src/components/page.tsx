import * as React from "react"
import { Header } from "./header"

export const Page: React.FC<{
    children?: JSX.Element | JSX.Element[]
}> = ({ children }) => {
    return <>
        <Header/>
        {children}
    </>
}