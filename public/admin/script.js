axios({
    method: 'get',
    url: '/api/info'
}).then(function (response) {
    if (response.data.inviteonly == false) {
        window.location.replace('/')
    }
})

function regkeygen() {
    password = document.getElementById("passfield").value
    axios({
        method: 'post',
        url: '/api/admin/regkeygen',
        data: {
            'password': password
        }
      }).then(function (response) {
        if ($('#errortext').length > 0) {
            document.getElementById("errortext").remove();
        }
        var regkey = response.data.regkey // Get user token
        Swal.fire({
            icon: 'success',
            title: 'Regkey generated!',
            text: `${regkey}`
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