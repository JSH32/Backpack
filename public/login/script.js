// Checking if the token is valid
if (localStorage.getItem('token') !== null) {
  axios({
    method: 'post',
    url: '/api/token/valid',
    data: {
      token: localStorage.getItem('token')
    }

  }).then(function () {
    window.location.replace('/upload')
  }).catch(function () {
    localStorage.removeItem('token')
  })
}

function login () {
  username = document.getElementById('userfield').value
  password = document.getElementById('passfield').value
  axios({
    method: 'post',
    url: '/api/token/get',
    data: {
      username: username,
      password: password
    }
  }).then(function (response) {
    const token = response.data // Get user token
    localStorage.setItem('token', token) // Set user token in localstorage
    window.location.replace('/upload')
  }).catch(function (error) {
    if ($('#errortext').length > 0) {
      $('#errortext').remove()
    }
    // Sending error text
    $('#errormessage').append(`<div id="errortext" style="margin-top: 5px;"><p class="tag is-danger">${error.response.data}</p></div>`)
  })
}
