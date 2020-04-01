const config = require('./config')

module.exports = ({ db, app, s3 }) => {

    // * API
    
    // User endpoints
    require('./api/user/signup')({ db, app, config })
    require('./api/user/info')({ db, app, config })
    require('./api/user/passreset')({ db, app, config })
    require('./api/user/delete')({ db, app, config })

    // Token endpoints
    require('./api/token/get')({ db, app, config })
    require('./api/token/valid')({ db, app, config })
    require('./api/token/regen')({ db, app, config })

    // File endpoints
    require('./api/files/upload')({ db, app, config, s3 })
    require('./api/files/listfiles')({ db, app, config })
    require('./api/files/delete')({ db, app, config, s3 })

    // Other endpoints
    require('./api/info')({ db, app, config })

    // Admin endpoints
    require('./api/admin/token/get')({ db, app, config })
    require('./api/admin/token/valid')({ db, app, config })
    require('./api/admin/delete/file')({ db, app, config, s3 })
    require('./api/admin/delete/user')({ db, app, config, s3 })
    require('./api/admin/list/users')({ db, app, config })
    require('./api/admin/list/uploads')({ db, app, config })

    // Admin signup endpoint, usually disabled
    if (config.admin.register == true) { require('./api/admin/signup')({ db, app, config }) }

    // Regkey generator, usually enabled
    if (config.inviteOnly == true) { require('./api/admin/regkeygen')({ db, app, config }) }

    // * FRONTEND

    // User Frontend
    app.get('/signup', async(req, rep) => {
        rep.renderMin('signup', { inviteOnly : config.inviteOnly })
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
        rep.renderMin('upload', { maxUploadSize : config.maxUploadSize })
    })

    // Admin frontend
    app.get('/admin', async(req, res) => {
        res.redirect('/admin/dash');
    })

    // Admin frontend
    app.get('/admin/signup', async(req, rep) => {
        if(config.admin.register == true) {
            rep.renderMin('admin/signup')
        } else {
            rep.renderMin('404')
        }
    })

    app.get('/admin/login', async(req, rep) => {
        rep.renderMin('admin/login')
    })

    app.get('/admin/dash', async(req, rep) => {
        rep.renderMin('admin/dash', { inviteOnly : config.inviteOnly })
    })

    // 404 Page
    app.get('*', function(req, rep){ 
        rep.renderMin('404'); 
    }) 
}