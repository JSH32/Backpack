// Checking if the token is valid
if (localStorage.getItem("token") !== null) {
    axios({
        method: 'post',
        url: '/api/token/valid',
        data: {
            'token': localStorage.getItem("token")
        }
    
    }).catch(function () {
        localStorage.removeItem("token")
        window.location.replace("/login");
    })
} else {
    window.location.replace("/login");
}

// Uploading with dropzone
$("#dropzone").dropzone({ 
    url: "/api/files/upload",
    paramName: "uploadFile",
    maxFilesize: window.uploadsize,
    previewsContainer: '#previewcontainer',
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
            let response = JSON.parse(data.xhr.response)
            return data.previewElement.innerHTML = `<a href="${response.url}">${response.url}</a><br>`
        }),
        this.on("error", function(data) {
            if (!data.xhr) {
                return data.previewElement.innerHTML = `<div class="errorlist">${data.previewElement.getElementsByClassName("dz-error-message")[0].innerHTML}</div>`
            } else {
                return data.previewElement.innerHTML = `<div class="errorlist">${data.xhr.response}</div>`
            }
        })
    }
});