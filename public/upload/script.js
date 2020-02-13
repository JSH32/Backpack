// Checking if the token is valid
if (localStorage.getItem("token") !== null) {
    axios({
        method: 'post',
        url: '/api/token/valid',
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
    
    var loadingbars = document.getElementById("file-list");

    var loading = document.createElement("center"); 
    loading.innerHTML = `<progress class="progress is-small is-info" style="max-width: 250px; border-radius: 3px; margin-bottom: 10px;" max="100">60%</progress>` 
    loading.id = `loading_bar`

    loadingbars.appendChild(loading);

    axios.post('/api/files/upload', formData, {
        headers: {
            'token': localStorage.getItem("token"),
            'Content-Type': 'multipart/form-data'
        }
    }).then(function (response) {
        var filereturn = response.data.url // Link response data
        var filelist = document.getElementById("file-list"); // Div where links will go

        // Generated element
        var linkgen = document.createElement("a"); 
        linkgen.innerHTML = `<a href="${filereturn}">${filereturn}</a><br>` 

        // Delete previous error if it exists
        if ($('#errorup').length > 0) {
            document.getElementById("errorup").remove();
        }

        // Create element in list div
        filelist.appendChild(linkgen);
            
        document.getElementById("loading_bar").remove();
    }).catch(function (error) {
        // On windows it doesnt send 413 for some reason, so im also catching null responses
        if (error.response == null || error.response.status == 413) {
            document.getElementById("loading_bar").remove(); // Remove existing loading bar
            if ($('#errorup').length > 0) {
                document.getElementById("errorup").remove();
            }

            var errorup = document.createElement("div"); 
            errorup.id = `errorup`
            errorup.innerHTML = `<center><p>You have exceeded the file limit!</p></center>` 
        
            document.getElementById("file-list").appendChild(errorup);
        } else if (error.response.status == 400) {
                document.getElementById("loading_bar").remove(); // Remove existing loading bar
            if ($('#errorup').length > 0) {
                document.getElementById("errorup").remove();
            }

            var errorup = document.createElement("div"); 
            errorup.id = `errorup`
            errorup.innerHTML = `<center><p>No files have been uploaded!</p></center>` 
        
            document.getElementById("file-list").appendChild(errorup);
            }
        })
    
}

$(document).ready(function () {
    document.getElementById("uploadFile").onchange = function () {
        uploadfile()
    }
})