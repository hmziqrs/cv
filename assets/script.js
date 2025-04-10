window.toggleJpegs = function () {
  const jpegs = Array.from(document.getElementsByClassName("jpeg"));
  jpegs.forEach((element) => {
    element.classList.toggle("hidden");
  });
};

window.toggleTheme = function () {
  const html_node = document.getElementsByTagName("html")[0];
  html_node.classList.toggle("dark");
};
