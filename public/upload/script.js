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
    $("#uploadbtn").dropzone({ 
        url: "/api/files/upload",
        paramName: "uploadFile",
        maxFilesize: infoapi.maxuploadsize,
        previewsContainer: '#uploadcontainer',
        previewTemplate: `
        <div id="tpl">
            <progress class="progress" value="15" max="100" data-dz-uploadprogress></progress>
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