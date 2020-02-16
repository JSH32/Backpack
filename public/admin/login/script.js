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

function login() {
    username = document.getElementById("userfield").value
    password = document.getElementById("passfield").value
    axios({
        method: 'post',
        url: '/api/admin/token/get',
        data: {
            'username': username,
            'password': password
        }
      }).then(function (response) {
        var token = response.data // Get user token
        localStorage.setItem('admin_token', token); // Set user token in localstorage
        window.location.replace("/admin/dash");
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