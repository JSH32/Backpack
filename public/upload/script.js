// Declare variables
let infoapi

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

// Request to get info about api
axios({
    method: 'get',
    url: '/api/info'
}).then(function (response) {
    infoapi = response.data
}).then(function () {
    // Uploading with dropzone
    $("#file-list").dropzone({ 
        url: "/api/files/upload",
        paramName: "uploadFile",
        maxFilesize: infoapi.maxuploadsize,
        previewsContainer: '#uploadcontainer',
        timeout: 0,
        previewTemplate: `
        <div id="tpl">
            <div id="loading_bar" style="max-width: 250px; margin: auto; margin-bottom: 10px; margin-top: 10px;"><progress class="ploader blue" max="100" data-dz-uploadprogress></progress></div>
            <div class="dz-error-message errorlist"><span data-dz-errormessage></span></div>
        </div>`,
        headers: {
            'token': localStorage.getItem("token")
        },
        init: function() {
            this.on("success", function(data) {
                var response = JSON.parse(data.xhr.response)
                $("#uploadcontainer").append(`<a href="${response.url}">${response.url}</a><br>` )
                document.getElementById("loading_bar").remove()
            }),
            this.on("error", function() {
                document.getElementById("loading_bar").remove()
            })
        }
    });
})