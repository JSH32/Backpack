// Declare vars
let infoapi


// Checking if the token is valid
if (localStorage.getItem("token") !== null) {
    axios({
        method: 'post',
        url: '/api/token/valid',
        data: {
            'token': localStorage.getItem("token")
        }

    }).then(function () {
        window.location.replace('/upload')
    }).catch(function () {
        localStorage.removeItem("token")
    })
}

axios({
    method: 'get',
    url: '/api/info'
}).then(function (response) {
    infoapi = response.data
    if (response.data.inviteonly == true) {
        $("#regkeyarea").append(`
        <div class="field">
                        <label style="color: #4f4f4f;" class="label">Regkey</label>
                        <div class="control">
                            <center>
                                <input class="input" type="password" id="regkeyfield"
                                    style="max-width: 300px; margin-bottom: 15px; background-color: #141414; border-color: #3273dc; color: white; text-align: center;"
                                    type="text">
                            </center>
                        </div>
                    </div>
        `)
    }
})

// I tried to shorten this as much as possible but i really couldnt :/
function login() {
    // Sending request with regkey if server is private mode
    if (infoapi.inviteonly == true) {
        username = document.getElementById("userfield").value
        password = document.getElementById("passfield").value
        regkey = document.getElementById("regkeyfield").value

        axios({
            method: 'post',
            url: '/api/user/signup',
            data: {
                'username': username,
                'password': password,
                'regkey': regkey
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
                    window.location.replace('/login')
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
    } else {
        // Send request without regkey
        username = document.getElementById("userfield").value
        password = document.getElementById("passfield").value

        axios({
            method: 'post',
            url: '/api/user/signup',
            data: {
                'username': username,
                'password': password
            }
        }).then(function () {
            Swal.fire({
                title: 'Account created!',
                text: "Please log in now!",
                icon: 'success',
                showCancelButton: false,
                confirmButtonColor: '#3085d6',
                confirmButtonText: 'OK'
            }).then((result) => {
                if (result.value) {
                    window.location.replace('/login')
                }
            })
        }).catch(function (error) {
            if ($('#errortext').length > 0) {
                document.getElementById("errortext").remove();
            }
            // Sending error text
            $("#errormessage").append(`<div id="errortext" style="margin-top: 5px;"><p class="tag is-danger">${error.response.data}</p></div>`);
        })
    }
}