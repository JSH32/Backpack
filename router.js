require('dotenv').config()

module.exports = ({ db, app }) => {

    // * API
    
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
    require('./api/admin/list/users')({ db, app })
    require('./api/admin/list/uploads')({ db, app })

    // Admin signup endpoint, usually disabled
    if (JSON.parse(process.env.ADMINREGISTER) == true) { require('./api/admin/signup')({ db, app }) }

    // Regkey generator, usually enabled
    if (JSON.parse(process.env.INVITEONLY) == true) { require('./api/admin/regkeygen')({ db, app }) }

    // * FRONTEND

    // User Frontend
    app.get('/signup', async(req, rep) => {
        rep.renderMin('signup', { inviteonly : process.env.INVITEONLY })
    })

    app.get('/', async(req, rep) => {
        rep.renderMin('index', { filecount : await db.collection('uploads').countDocuments() })
    })

    app.get('/login', async(req, rep) => {
        rep.renderMin('login')
    })

    app.get('/dash', async(req, rep) => {
        rep.renderMin('dash')
    })

    app.get('/about', async(req, rep) => {
        rep.renderMin('about')
    })

    app.get('/upload', async(req, rep) => {
        rep.renderMin('upload', { maxfilesize : process.env.MAXUPLOADSIZE })
    })

    // Admin frontend
    app.get('/admin', async(req, res) => {
        res.redirect('/admin/dash');
    })

    // Admin frontend
    app.get('/admin/signup', async(req, rep, res) => {
        if(JSON.parse(process.env.ADMINREGISTER) == true) {
            rep.renderMin('admin/signup')
        } else {
            res.redirect('/');
        }
    })

    app.get('/admin/login', async(req, rep) => {
        rep.renderMin('admin/login', { maxfilesize : process.env.MAXUPLOADSIZE })
    })

    app.get('/admin/dash', async(req, rep) => {
        rep.renderMin('admin/dash', { inviteonly : JSON.parse(process.env.INVITEONLY) })
    })

    // 404 Page
    app.get('*', function(req, rep){ 
        rep.renderMin('404'); 
    }) 
}