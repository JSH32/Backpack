axios({
    method: 'get',
    url: '/api/info'
}).then(function (response) {
    document.getElementById("count").innerHTML = response.data.totalfiles
})