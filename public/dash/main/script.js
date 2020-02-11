// Checking if the token is valid
if (localStorage.getItem("token") !== null) {
    axios({
        method: 'post',
        url: '/token/valid',
        data: {
            'token': localStorage.getItem("token")
        }
    
    }).catch(function (error) {
        localStorage.removeItem("token")
        window.location.replace("/login");
    })
} else {
    window.location.replace("/login");
} 


function openTab(evt, tabName) {
    // Declare all variables
    var i, tabcontent, tablinks;
  
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
    method: 'post',
    url: '/files/list',
    data: {
        'token': localStorage.getItem("token")
    }
}).then(function (response) {
    response.data.map( (file, index) => {
        // create an element
        $("#efs").append(`
        <div class="listitem" id="${index}">
        <th><a href="/${file}">${file}</a></th>
        <th><a filename="${file}" id="${index}" style="color: #ff5145;" class="dl">Delete</a></tf>
        </div>
        `)
    })
}).then(function () {
    checkifzero()
})


// Check if the filelist is zero
function checkifzero () {
    // Do nothing if more than zero, make element if over 0
    if ($('.listitem').length){}else{
        $("#efs").append(`
        <div style="color: #616161;" class="noexistlist">
        <p>You have not uploaded any files :(</p>
        </div>
        `)
    }
}

$( document ).ready(function() {
    axios({
        method: 'post',
        url: '/user/info',
        data: {
            'token': localStorage.getItem("token")
        }
    
    }).then(function (response) {
        var usrname = response.data.username
    
        $("#file-subtitle").append(`
            <p>Uploads for user <b>${usrname}</b></p>
        `)
    })
});
