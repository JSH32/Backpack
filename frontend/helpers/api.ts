import { BackpackClient } from "@/client"
import getConfig from "next/config"

const { publicRuntimeConfig } = getConfig()

export default new BackpackClient({
	BASE: publicRuntimeConfig.apiRoot,
	TOKEN: async () => localStorage.getItem("token") || undefined as unknown as string
})
