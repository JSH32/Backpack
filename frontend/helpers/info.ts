import { AppInfo } from "@/client"
import React from "react"

export const AppInfoContext = React.createContext<AppInfo | null>(null)
export const useAppInfo = (): AppInfo | null => React.useContext(AppInfoContext)
