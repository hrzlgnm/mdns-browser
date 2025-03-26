/**
 * Workarounds for thaw inputs wrapped by components `AutoComplete` or `Input`
 * providing no possibility to set attributes or to add onkeydown event listeners.
 * We want no spell checking no autocorrection or capitalization in our inputs 
 * mostly dealing with lower case characters.
 */
window.addEventListener("TrunkApplicationStarted", function () {
    const browseButton = document.querySelector("button.thaw-button--primary[contains(text(), 'Browse')]");
    const inputs = document.querySelectorAll("input.thaw-input__input");
    for (const input of inputs) {
        input.setAttribute("autocapitalize", "none");
        input.setAttribute("autocorrect", "off");
        input.setAttribute("spellcheck", "false");
        if (input.getAttribute("placeholder")?.includes("Quick")) {
            input.addEventListener("keydown", function (event) {
                if (event.key === "Enter") {
                    browseButton.click();
                }
            });
        }
    }
});
