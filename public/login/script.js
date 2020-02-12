// Checking if the token is valid
if (localStorage.getItem("token") !== null) {
    axios({
        method: 'post',
        url: '/token/valid',
        data: {
            'token': localStorage.getItem("token")
        }
    
    }).then(function () {
        window.location.replace('/upload')
    }).catch(function (error) {
        localStorage.removeItem("token")
    })
}

function login() {
    username = document.getElementById("userfield").value
    password = document.getElementById("passfield").value
    axios({
        method: 'post',
        url: '/token/get',
        data: {
            'username': username,
            'password': password
        }
      }).then(function (response) {
        var token = response.data // Get user token
        localStorage.setItem('token', token); // Set user token in localstorage
        window.location.replace("/upload");
    }).catch(function (error) {
        if (!document.getElementById("errortext")) {
        // Sending error text
        var errortext = document.createElement("p"); 
        errortext.innerHTML = `<div style="margin-bottom: -20px; margin-top: 5px;"><p class="tag is-danger">${error.response.data}</p></div>`
        errortext.id = `errortext`
        errormessage.appendChild(errortext);
    
        // Sending breakline under text
        var breakline = document.createElement("br")
        errormessage.appendChild(breakline);
        }
    })
}