// Right Click context menu, requires jQuery
$(document).bind("contextmenu", function(e) {
  e.preventDefault();
  $("#rc-menu").css("left", e.pageX);
  $("#rc-menu").css("top", e.pageY);
  $("#rc-menu").fadeIn(100, startFocusOut());
});

function startFocusOut() {
  $(document).on("click", function() {
    $("#rc-menu").hide();
    $(document).off("click");
  });
}

$("#context-menu > li").click(function() {
  if ($(this).text() == "Open App Drawer") {
    window.open("./app-drawer.html", "App List", "width=1280,height=720");
  }
  if ($(this).text() == "Open Desktop Settings") {
    window.open("./desktop-settings.html", "Desktop Settings", "width=1280,height=720");
  }
});
