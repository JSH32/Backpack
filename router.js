require('dotenv').config()

module.exports = ({ db, app }) => {
    // User Frontend
    app.get('/signup', async(req, rep) => {
        rep.render('signup', { inviteonly : process.env.INVITEONLY })
    })

    app.get('/', async(req, rep) => {
        rep.render('index', { filecount : await db.collection('uploads').countDocuments() })
    })

    app.get('/login', async(req, rep) => {
        rep.render('login')
    })

    app.get('/dash', async(req, rep) => {
        rep.render('dash')
    })

    app.get('/about', async(req, rep) => {
        rep.render('about')
    })

    app.get('/upload', async(req, rep) => {
        rep.render('upload', { maxfilesize : process.env.MAXUPLOADSIZE })
    })

    // Admin frontend
    app.get('/admin', async(req, res) => {
        res.redirect('/admin/dash');
    })

    // Admin frontend
    app.get('/admin/signup', async(req, rep, res) => {
        if(JSON.parse(process.env.ADMINREGISTER) == true) {
            rep.render('admin/signup')
        } else {
            res.redirect('/');
        }
    })

    app.get('/admin/login', async(req, rep) => {
        rep.render('admin/login', { maxfilesize : process.env.MAXUPLOADSIZE })
    })

    app.get('/admin/dash', async(req, rep) => {
        rep.render('admin/dash', { inviteonly : process.env.INVITEONLY })
    })
}