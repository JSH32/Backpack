require('dotenv').config()

const express = require('express')
const mongo = require('./api/mongo')
const app = express()
const bodyParser = require('body-parser');

app.use(express.json())
app.use(bodyParser.json());


mongo.init().then(db => {
    // Serving upload directory
    if (JSON.parse(process.env.UPLOADS_SERVE) == true) {app.use('/', express.static(process.env.UPLOAD_DIR))}
    

    // Serve interface
    app.use('/', express.static('./public'))

    // User endpoints
    require('./api/user/signup')({ db, app })
    require('./api/user/info')({ db, app })
    require('./api/user/passreset')({ db, app })
    require('./api/user/delete')({ db, app })

    // Token endpoints
    require('./api/token/get')({ db, app })
    require('./api/token/valid')({ db, app })
    require('./api/token/regen')({ db, app })

    // File endpoints
    require('./api/files/upload')({ db, app })
    require('./api/files/listfiles')({ db, app })
    require('./api/files/delete')({ db, app })

    // Other endpoints
    require('./api/info')({ db, app })

    // Admin endpoints
    require('./api/admin/token/get')({ db, app })
    require('./api/admin/token/valid')({ db, app })
    require('./api/admin/delete/file')({ db, app })
    require('./api/admin/delete/user')({ db, app })
    require('./api/admin/regkeygen')({ db, app })
    require('./api/admin/list/users')({ db, app })
    require('./api/admin/list/uploads')({ db, app })

    // Admin signup endpoint, usually disabled
    if (JSON.parse(process.env.ADMINREGISTER) == true) { require('./api/admin/signup')({ db, app }) }
})

app.listen(8080)