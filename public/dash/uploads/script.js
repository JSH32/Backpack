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
    
        $("#subtitle").append(`
            <p>Uploads for user <b>${usrname}</b></p>
        `)
    })
});



$(document).on('click','.dl', function(){
    var id = $(this).attr('id');
    var file = $(this).attr('filename');
    // make delete request with id

    axios({
        method: 'post',
        url: '/files/delete',
        data: {
            'token': localStorage.getItem("token"),
            'file': file
        }
    }).then(function () {
        document.getElementById(id).remove();
        checkifzero()
    })
})