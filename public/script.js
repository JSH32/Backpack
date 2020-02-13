var xhr = new XMLHttpRequest();
xhr.open('GET', "/api/files/total", true);
xhr.send();

xhr.addEventListener("readystatechange", processRequest, false);
xhr.onreadystatechange = processRequest;

function processRequest(e) {
    if (xhr.readyState == 4 && xhr.status == 200) {
        document.getElementById("count").innerHTML = xhr.responseText;
    }
}