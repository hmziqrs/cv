window.toggleJpegs = function () {
  const jpegs = document.getElementsByClassName("jp");
};

var html_node = document.getElementsByTagName("html")[0];

window.toggleTheme = function () {
  if (html_node.classList.contains("dark")) {
    html_node.classList.remove("dark");
  } else {
    html_node.classList.add("dark");
  }
};
