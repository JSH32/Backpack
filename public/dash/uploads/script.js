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
        <div id="${index}">
        <th><a href="/${file}">${file}</a></th>
        <th><a id="${index}" style="color: #ff5145;" class="dl">Delete</a></tf>
        </div>
        `)
    })
})

// Checking if the token is valid

$( document ).ready(function() {
    axios({
        method: 'post',
        url: '/user/info',
        data: {
            'token': localStorage.getItem("token")
        }
    
    }).then(function (response) {
        console.log(response.data.username)
        var usrname = response.data.username
    
        $("#subtitle").append(`
            <p>Uploads for user <b>${usrname}</b></p>
        `)
    })
});




$(document).on('click','.dl', function(){
    var id = $(this).attr('id');
    // make delete request with id


    document.getElementById(id).remove();
})

// function deleteFile() {
//     axios({
//         method: 'post',
//         url: '/files/delete',
//         data: {
//             'token': localStorage.getItem("token"),
//             'file': '${file}'
//         }
//     }).then(function (response) {
//         console.log(response.data)
//     })
// }