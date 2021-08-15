/**
 * This script is designed to run in a docker container
 * It is supposed to rebuild the client when certain environment variables change
 */

const fs = require("fs")
const path = require("path")
const { spawn } = require("child_process")

const OLD_NAME = path.join(__dirname, "check.json")

const currentEnv = {
  SNOWPACK_PUBLIC_APP_NAME: process.env.APP_NAME == null ? "Backpack" : process.env.APP_NAME,
  SNOWPACK_PUBLIC_APP_DESCRIPTION: process.env.APP_DESCRIPTION == null ? "A file sharing service for all your needs" : process.env.APP_DESCRIPTION,
  SNOWPACK_PUBLIC_APP_SMTP_ENABLED: process.env.SMTP_ENABLED == null ? false : true
}

const build = () => {
  spawn(/^win/.test(process.platform) ? "npm.cmd" : "npm", ["run", "build"], { 
    stdio: "inherit",
    cwd: path.join(__dirname, "..", "client"),
    env: {
      ...process.env,
      ...currentEnv,
      NODE_ENV: "production"
    },
  })
}

if (!fs.existsSync(OLD_NAME)) {
  fs.writeFileSync(OLD_NAME, JSON.stringify(currentEnv))
  build()
  return
}

const oldEnv = JSON.parse(fs.readFileSync(OLD_NAME))
fs.writeFileSync(OLD_NAME, JSON.stringify(currentEnv))

for (const val in oldEnv) {
  if (oldEnv[val] != currentEnv[val]) {
    // Something changed so frontend will be rebuilt
    build()
    break
  }
}