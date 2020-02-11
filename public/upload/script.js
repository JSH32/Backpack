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


function uploadfile() {
    var formData = new FormData();
    var uploadFile = document.querySelector('#uploadFile');
    formData.append("uploadFile", uploadFile.files[0]);
    if (uploadFile.files[0] == undefined) {
        alert('no')
    } else {
        axios.post('/files/upload', formData, {
            headers: {
                'token': localStorage.getItem("token"),
                'Content-Type': 'multipart/form-data'
            }
        }).then(function (response) {
            var filereturn = response.data // Link response data
            var filelist = document.getElementById("file-list"); // Div where links will go

            // Generated element
            var linkgen = document.createElement("a"); 
            linkgen.innerHTML = `<a href="${filereturn}">${filereturn}</a><br>` 

            // Create element in list div
            filelist.appendChild(linkgen);
        })
    }
}

$(document).ready(function () {
    document.getElementById("uploadFile").onchange = function () {
        uploadfile()
    }
})