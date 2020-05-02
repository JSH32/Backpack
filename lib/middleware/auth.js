const argon = require("argon2")

// Authentication middleware for kawaii.sh
module.exports = function (db, options) {

    // Middleware options
    options = Object.assign({
        database: "users", // Database to use
        authMethod: "token", // token, password
        useTokenHeaders: false // Use headers for token
    }, options)

    return async function(req, res, next) {
        const body = req.body
        const Users = db.collection(options.database)

        if (options.authMethod === "password") {
            // Returning error invalid request
            if (!body.password || !body.username) {
                return res.status(400).send('The username/password you entered is incorrect!')
            }

            const userExists = Boolean(await Users.findOne({ "username": body.username }))
    
            if (!userExists) {
                return res.status(400).send('The username/password you entered is incorrect!')
            }

            const user = await Users.findOne({ "username": body.username })

            // Checks if the user is in lockdown
            if (user.lockdown) {
                return res.status(400).send('The username/password you entered is incorrect!')
            }
    
            // If wrong password return error
            if (!await argon.verify(user.password_hash, body.password)) {
                return res.status(400).send('The username/password you entered is incorrect!')
            }
    
            next()

        // Authentication with token
        } else if (options.authMethod === "token") {
            let token = body.token

            if (options.useTokenHeaders) {
                token = req.headers.token         
            }
            
            if (!token) {
                return res.status(400).send('Invalid token!')
            }
            
            if (!Boolean(await Users.findOne({ "token": token }))) {
                return res.status(400).send('Invalid token!')
            }

            // Checks if the user is in lockdown
            const user = await Users.findOne({ "token": token })
            if (user.lockdown) {
                return res.status(400).send('Invalid token!')
            }
    
            // Go to next middleware/program
            next()
        }
    }

}