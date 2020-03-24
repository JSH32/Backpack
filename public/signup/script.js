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
                $("#errortext").remove();
            }
            // Sending error text
            $("#errormessage").append(`<div id="errortext" style="margin-top: 5px;"><p class="tag is-danger">${error.response.data}</p></div>`);
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
                $("#errortext").remove();
            }
            // Sending error text
            $("#errormessage").append(`<div id="errortext" style="margin-top: 5px;"><p class="tag is-danger">${error.response.data}</p></div>`);
        })
    }
}