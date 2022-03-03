import React from "react"
import { AppInfo } from "./api"

export const AppInfoContext = React.createContext<AppInfo | null>(null)
export const useAppInfo = (): AppInfo | null => React.useContext(AppInfoContext)