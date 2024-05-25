window.addEventListener("keyup", function(e) {
    var key = document.getElementById("key");
    if (e.keyCode == 8 || e.keyCode == 13 || (e.keyCode >= 65 && e.keyCode <= 90)) {
        e.preventDefault();
        key.value = e.key.toLowerCase();
        htmx.trigger("form", "submit");
    }
});
