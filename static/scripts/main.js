const jumbotron = document.getElementById("jumbotron");
const burger = document.getElementById("burger");
const content = document.getElementById("jumbotron__content-inner");
const mobileNav = document.getElementById("mobile-nav");
const mobileNavItems = document.getElementsByClassName("mobile-nav__item");

document.getElementById("burger__checkbox").addEventListener("change", function() {
  if (this.checked) {
    jumbotron.classList.add("burger-is-checked");
    burger.classList.add("burger-is-checked");
    content.classList.add("burger-is-checked");
    mobileNav.classList.add("burger-is-checked");

    for (const item of mobileNavItems) {
      item.classList.remove("burger-was-checked");
      item.classList.add("burger-is-checked");
    }
  } else {
    jumbotron.classList.remove("burger-is-checked");
    burger.classList.remove("burger-is-checked");
    content.classList.remove("burger-is-checked");
    mobileNav.classList.remove("burger-is-checked");

    for (const item of mobileNavItems) {
      item.classList.remove("burger-is-checked");
      item.classList.add("burger-was-checked");
    }
  }
});