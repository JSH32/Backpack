// Declaring variables
let infoapi

// Checking if the token is valid
if (localStorage.getItem("admin_token") !== null) {
    axios({
        method: 'post',
        url: '/api/admin/token/valid',
        data: {
            'token': localStorage.getItem("admin_token")
        }
    
    }).catch(function () {
        localStorage.removeItem("admin_token")
        window.location.replace("/admin/login");
    })
} else {
    window.location.replace("/admin/login");
} 

// Logout
function logout() {
    localStorage.removeItem("admin_token")
    window.location.replace("/admin/login");
}

// Tabs
function openTab(evt, tabName) {
    // Declare all variables
    let i, tabcontent, tablinks;
  
    // Get all elements with class="tabcontent" and hide them
    tabcontent = document.getElementsByClassName("tabcontent");
    for (i = 0; i < tabcontent.length; i++) {
      tabcontent[i].style.display = "none";
    }
  
    // Get all elements with class="tablinks" and remove the class "active"
    tablinks = document.getElementsByClassName("tablinks");
    for (i = 0; i < tablinks.length; i++) {
      tablinks[i].className = tablinks[i].className.replace(" active", "");
    }
  
    // Show the current tab, and add an "active" class to the button that opened the tab
    document.getElementById(tabName).style.display = "block";
    evt.currentTarget.className += " active";
}


axios({
    method: 'get',
    url: '/api/info'
}).then(function (response) {
    infoapi = response.data
})


// Get upload list
function getListUpload() {
    let query = document.getElementById("filesearch").value
    if (query == "") {
        query = " "
    }

    // Clear results
    if ($('#efs').not(':empty')){
        $("#efs").empty();
    }

    axios({
        method: 'post',
        url: '/api/admin/list/uploads',
        data: {
            'token': localStorage.getItem("admin_token"),
            'query': query
        }
    }).then(function (response) {
        response.data.map( ({file, username}, index) => {
            // create an element
            $("#efs").append(`
            <div id="${index}">
            <th><p style="display: inline; color: #7a7a7a;">${username}</p></th>
            <th><a href="${infoapi.uploadURL}${file}">${file}</a></th>
            <th><a filename="${file}" id="${index}" style="color: #ff5145;" class="dlfl">Delete</a></th>
            </div>
            `)
        })

        // Delete previous error since it works
        if ($('#errortextupl').length > 0) {
            document.getElementById("errortextupl").remove();
        }

        // Send error if no results
        if (response.data.length == 0) {
            if ($('#uplnotfoundtxt').length > 0) {
                $("#uplnotfoundtxt").remove();
            }
            // Sending error text
            $("uploadnotfound").append(`<p id="uplnotfoundtxt"><div id="errorbox" style="margin-bottom: 20px; margin-top: -20px;"><p class="tag is-danger">No search results found!</p></div></p>`);
        }
    })
}

// Delete files
$(document).on('click','.dlfl', function(){
    let id = $(this).attr('id');
    let file = $(this).attr('filename');
    // make delete request with id
    axios({
        method: 'post',
        url: '/api/admin/delete/file',
        data: {
            'token': localStorage.getItem("admin_token"),
            'file': file
        }
    }).then(function () {
        removemsg = `<div class="listitem" style="color: #383838;"><th><p>This file has been deleted!</p></th></div>`
        document.getElementById(id).innerHTML = removemsg
    })
})

// Get userlist
function getListUsers() {
    let query = document.getElementById("usersearch").value
    if (query == "") {
        query = " "
    }

    // Clear results
    if ($('#efsusr').not(':empty')){
        $("#efsusr").empty();
    }

    axios({
        method: 'post',
        url: '/api/admin/list/users',
        data: {
            'token': localStorage.getItem("admin_token"),
            'query': query
        }
    }).then(function (response) {
        response.data.map( (username, index) => {
            // create an element
            $("#efsusr").append(`
            <div id="${index}">
            <th><p style="display: inline; color: #7a7a7a;">${username}</p></th>
            <th><a username="${username}" id="${index}" style="color: #ff5145;" class="dlusr">Delete</a></th>
            </div>
            `)
        })

        // Delete previous error since it works
        if ($('#errortextusr').length > 0) {
            document.getElementById("errortextusr").remove();
        }

        // Send error if no results
        if (response.data.length == 0) {
            if ($('#usrnotfoundtxt').length > 0) {
                document.getElementById("usrnotfoundtxt").remove();
            }
            // Sending error text
            $("#usernotfound").append(`<p id="usrnotfoundtxt"><div id="errorbox" style="margin-bottom: 20px; margin-top: -20px;"><p class="tag is-danger">No search results found!</p></div></p>`);
        }
    })
}

// Delete users
$(document).on('click','.dlusr', function(){
    let id = $(this).attr('id');
    let user = $(this).attr('username');
    // make delete request with id
    axios({
        method: 'post',
        url: '/api/admin/delete/user',
        data: {
            'token': localStorage.getItem("admin_token"),
            'username': user
        }
    }).then(function () {
        removemsg = `<div class="listitem" style="color: #383838;"><th><p>This user has been deleted!</p></th></div>`
        document.getElementById(id).innerHTML = removemsg
    })
})

function regKeyGen() {
    axios({
        method: 'post',
        url: '/api/admin/regkeygen',
        data: {
            'token': localStorage.getItem("admin_token")
        }
    }).then(function (response) {
        Swal.fire(
            'Regkey generated!',
            response.data.regkey
        )
    })
}
