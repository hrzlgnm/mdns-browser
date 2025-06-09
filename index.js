/**
 * Workarounds for thaw inputs wrapped by components `AutoComplete` or `Input`
 * providing no possibility to set attributes or to add onkeydown event listeners.
 * We want no spell checking no autocorrection or capitalization in our inputs
 * mostly dealing with lower case characters.
 * Also turn off drop events globally by just ignoring swallowing those
 */
window.addEventListener("TrunkApplicationStarted", function () {
    document.addEventListener("dragover", (event) => {
        event.preventDefault();
        event.stopPropagation();
    });
    document.addEventListener("drop", (event) => {
        event.preventDefault();
        event.stopPropagation();
    });
    const browseButton = document.querySelector("button.thaw-button--primary");
    const inputs = document.querySelectorAll("input.thaw-input__input");
    const autocomplete = document.querySelector("div.thaw-auto-complete");
    function handleEnterKey(event, isValid = true) {
        if (event.key === "Enter" && isValid && browseButton) {
            event.preventDefault();
            browseButton.click();
        }
    }
    for (const input of inputs) {
        input.setAttribute("autocapitalize", "none");
        input.setAttribute("autocorrect", "off");
        input.setAttribute("spellcheck", "false");
        if (input.getAttribute("placeholder")?.includes("Quick")) {
            input.addEventListener("keydown", function (event) {
                handleEnterKey(event);
            });
        }
        if (input.getAttribute("placeholder")?.includes("Service")) {
            input.addEventListener("keydown", function (event) {
                const inputIsValid = autocomplete
                    ?.getAttribute("class")
                    ?.includes("service-type-valid");
                handleEnterKey(event, inputIsValid);
            });
        }
    }
});
