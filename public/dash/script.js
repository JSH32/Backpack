// Define variables
let userinfo
let infoapi

// Checking if the token is valid
if (localStorage.getItem('token') !== null) {
  axios({
    method: 'post',
    url: '/api/token/valid',
    data: {
      token: localStorage.getItem('token')
    }
  }).catch(function () {
    localStorage.removeItem('token')
    window.location.replace('/login')
  })
} else {
  window.location.replace('/login')
}

// Logout
function logout () {
  localStorage.removeItem('token')
  window.location.replace('/login')
}

// Tabs
function openTab (evt, tabName) {
  // Declare all variables
  let i, tabcontent, tablinks

  // Get all elements with class="tabcontent" and hide them
  tabcontent = document.getElementsByClassName('tabcontent')
  for (i = 0; i < tabcontent.length; i++) {
    tabcontent[i].style.display = 'none'
  }

  // Get all elements with class="tablinks" and remove the class "active"
  tablinks = document.getElementsByClassName('tablinks')
  for (i = 0; i < tablinks.length; i++) {
    tablinks[i].className = tablinks[i].className.replace(' active', '')
  }

  // Show the current tab, and add an "active" class to the button that opened the tab
  document.getElementById(tabName).style.display = 'block'
  evt.currentTarget.className += ' active'
}

// Create filelist and pagination array
axios({
  method: 'get',
  url: '/api/info'
}).then(function (response) {
  infoapi = response.data
  axios({
    method: 'post',
    url: '/api/files/list',
    data: {
      token: localStorage.getItem('token')
    }
  }).then(function (response) {
    response.data.map((file, index) => {
      // create an element
      $('#efs').append(`
            <div class="listitem" id="${index}">
            <th><a href="${infoapi.uploadURL}${file}">${file}</a></th>
            <th><a filename="${file}" id="${index}" style="color: #ff5145;" class="dl">Delete</a></tf>
            </div>
            `)
    })
  }).then(function () {
    checkifzero()
    $('#efs').nekoPaginate({
      paginateElement: 'div',
      elementsPerPage: 10,
      lastButton: false,
      firstButton: false
    })
  })
})

// Delete files
$(document).on('click', '.dl', function () {
  const id = $(this).attr('id')
  const file = $(this).attr('filename')
  // make delete request with id
  axios({
    method: 'post',
    url: '/api/files/delete',
    data: {
      token: localStorage.getItem('token'),
      file: file
    }
  }).then(function () {
    // Set to file deleted text
    document.getElementById(id).innerHTML = '<div class="listitem" style="color: #383838;"><th><p>This file has been deleted!</p></th></div>'
    checkifzero()
    getFilecount()
    // Set the value to removemsg in pagination array to prevent invalid value refreshing
    if (typeof pageobj !== 'undefined') {
      pageobj[id].innerHTML = removemsg
    }
  })
})

// Check if the filelist is zero
function checkifzero () {
  // Do nothing if more than zero, make element if over 0
  if ($('.listitem').length) {} else {
    $('#efs').append(`
        <div style="color: #616161;" class="noexistlist">
        <p>You have not uploaded any files :(</p>
        </div>
        `)
    document.getElementById('filecount').remove()
  }
}

// Setting username in upload tab
axios({
  method: 'post',
  url: '/api/user/info',
  data: {
    token: localStorage.getItem('token')
  }
}).then(function (response) {
  userinfo = response.data
  const totalfiles = response.data.filecount

  $('#file-subtitle').append(`
        <p>Uploads for user <b>${userinfo.username}</b></p>
    `)

  if ($('#filecount').length > 0) {
    document.getElementById('filecount').innerHTML = `<p>Total user uploads: ${totalfiles}</p>`
  }
})

// Checks filecount and resets value for each delete
function getFilecount () {
  axios({
    method: 'post',
    url: '/api/user/info',
    data: {
      token: localStorage.getItem('token')
    }

  }).then(function (response) {
    const totalfiles = response.data.filecount

    if (totalfiles > 0) {
      document.getElementById('filecount').innerHTML = `<p>Total user uploads: ${totalfiles}</p>`
    } else {
      document.getElementById('filecount').innerHTML = '<p>You have not uploaded any files :(</p>'
    }
  })
};

// I hate this more than words can express
function downloadSharex () {
  const sharex_dl =
`{
    "Name": "kawaii.sh",
    "DestinationType": "ImageUploader, TextUploader, FileUploader",
    "RequestMethod": "POST",
    "RequestURL": "${window.location.origin}/api/files/upload",
    "Headers": {
        "token": "${localStorage.getItem('token')}"
    },
    "Body": "MultipartFormData",
    "FileFormName": "uploadFile",
    "URL": "$json:url$",
    "ThumbnailURL": "$json:url$"
}`

  const fileType = 'text/plain'

  const blob = new Blob([sharex_dl], { type: fileType })

  const a = document.createElement('a')
  a.download = 'sharex.sxcu'
  a.href = URL.createObjectURL(blob)
  a.dataset.downloadurl = [fileType, a.download, a.href].join(':')
  a.style.display = 'none'
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  setTimeout(function () { URL.revokeObjectURL(a.href) }, 1500)
}

// Set token value from localstorage
document.getElementById('tokenval').innerHTML = `${localStorage.getItem('token')}`

// Reset token button
function resetToken () {
  axios({
    method: 'post',
    url: '/api/token/regen',
    data: {
      token: localStorage.getItem('token')
    }
  }).then(function () {
    localStorage.removeItem('token')
    window.location.replace('/login')
  })
}

// Reset password
function reset () {
  password = document.getElementById('passfield').value
  newpassword = document.getElementById('newpassfield').value

  axios({
    method: 'post',
    url: '/api/user/passreset',
    data: {
      username: userinfo.username,
      password: password,
      newpassword: newpassword
    }
  }).then(function () {
    if ($('#restext').length > 0) {
      $('#restext').remove()
    }

    $('#resbox').append('<div id="restext" style="margin-top: 5px;"><p class="tag is-link">Password has been reset!</p></div>')
  }).catch(function (error) {
    if ($('#restext').length > 0) {
      $('#restext').remove()
    }

    // Sending error text
    $('#resbox').append(`<div id="restext" style="margin-top: 5px;"><p class="tag is-danger">${error.response.data}</p></div>`)
  })
}

// Purge account and data
$(document).on('click', '#purgebutton', function () {
  Swal.fire({
    title: 'Are you sure?',
    text: "You won't be able to revert this!",
    html:
        '<h1>Username</h1>' +
        '<input style="text-align: center;" id="usernamepurge" class="swal2-input">' +
        '<h1>Password</h1>' +
        '<input style="text-align: center;" type="password" id="passwordpurge" class="swal2-input">',
    icon: 'warning',
    showCancelButton: true,
    confirmButtonColor: '#3085d6',
    cancelButtonColor: '#d33',
    confirmButtonText: 'Purge'
  }).then((result) => {
    if (result.value) {
      username = document.getElementById('usernamepurge').value
      password = document.getElementById('passwordpurge').value
      axios({
        method: 'post',
        url: '/api/user/delete',
        data: {
          username: username,
          password: password
        }
      }).then(function () {
        localStorage.removeItem('token')
        Swal.fire({
          title: 'Success',
          text: 'User has been purged from the system!',
          icon: 'success',
          showCancelButton: false,
          confirmButtonColor: '#3085d6',
          confirmButtonText: 'OK'
        }).then((result) => {
          if (result.value) {
            window.location.replace('/')
          }
        })
      }).catch(() => {
        Swal.fire({
          title: 'Incorrect password!',
          text: 'User was not purged from the system!',
          icon: 'error',
          showCancelButton: false,
          confirmButtonColor: '#3085d6',
          confirmButtonText: 'OK'
        })
      })
    }
  })
})
