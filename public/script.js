axios({
    method: 'get',
    url: '/api/info'
}).then(function (response) {
    document.getElementById("count").innerHTML = response.data.totalfiles
})

// // Random neko
// $( document ).ready(function() {
//     $("#randomneko").append(`<img style="max-height: 400px;" src="/assets/nekos/${Math.floor(Math.random() * (11 - 1)) + 1}.png">`)
// });