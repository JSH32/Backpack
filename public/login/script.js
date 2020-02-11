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

    }).catch(function (error) {
        if (!document.getElementById("errortext")) {
        // Sending error text
        var errortext = document.createElement("p"); 
        errortext.innerHTML = `<span class="tag is-danger">Incorrect username/password!</span>`
        errortext.id = `errortext`
        errormessage.appendChild(errortext);
    
        // Sending breakline under text
        var breakline = document.createElement("br")
        errormessage.appendChild(breakline);
        }
    })
}