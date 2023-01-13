const commands = document.getElementsByClassName("command");
const noCommandsText = document.getElementById("no-commands-text");

function maybeShowNoCommandsText() {
  for (const command of commands) {
    if (getComputedStyle(command).display !== "none") {
      noCommandsText.classList.remove("show");
      return;
    }
  }
  noCommandsText.classList.add("show");
}

document.getElementById("search__text-input").addEventListener("input", function() {
  function includes(command, element, value) {
    return command.getElementsByClassName(`command__${element}`).item(0).innerHTML.toLowerCase().includes(value);
  }

  for (const command of commands) {
    const inputValue = this.value.trim().toLowerCase();

    if (includes(command, "name", inputValue) || includes(command, "description", inputValue)) {
      command.classList.remove("hidden");
    } else {
      command.classList.add("hidden");
    }
  }
  maybeShowNoCommandsText();
});

function initCheckbox(name) {
  document.getElementById(`${name}-checkbox`).addEventListener("change", function() {
    for (const command of commands) {
      if (command.getAttribute(`data-${name}`) === "false") {
        command.classList[this.checked ? "add" : "remove"](`hide-no-${name}`);
      }
    }
    maybeShowNoCommandsText();
  });
}

initCheckbox("speech-ready");
initCheckbox("dms");