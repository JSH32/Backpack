// Checking if the token is valid
if (localStorage.getItem("admin_token") !== null) {
    axios({
        method: 'post',
        url: '/api/admin/token/valid',
        data: {
            'token': localStorage.getItem("admin_token")
        }

    }).then(function () {
        window.location.replace('/admin/dash')
    }).catch(function (error) {
        localStorage.removeItem("admin_token")
    })
}

axios({
    method: 'get',
    url: '/api/info'
}).then(function (response) {
    if (response.data.adminreg == false) {
        window.location.replace('/')
    }
})

// I tried to shorten this as much as possible but i really couldnt :/
function login() {
    // Sending request with regkey if server is private mode
    username = document.getElementById("userfield").value
    password = document.getElementById("passfield").value
    adminkey = document.getElementById("adminkey").value

    axios({
        method: 'post',
        url: '/api/admin/signup',
        data: {
            'username': username,
            'password': password,
            'adminkey': adminkey
        }
    }).then(function (response) {

        Swal.fire({
            title: 'Account created!',
            text: "Please log in now!",
            icon: 'success',
            showCancelButton: false,
            confirmButtonColor: '#3085d6',
            confirmButtonText: 'OK'
        }).then((result) => {
            if (result.value) {
                window.location.replace('/admin/login')
            }
        })

    }).catch(function (error) {
        if ($('#errortext').length > 0) {
            document.getElementById("errortext").remove();
        }
        // Sending error text
        var errortext = document.createElement("p");
        errortext.innerHTML = `<div style="margin-top: 5px;"><p class="tag is-danger">${error.response.data}</p></div>`
        errortext.id = `errortext`
        errormessage.appendChild(errortext);
    })
} 
