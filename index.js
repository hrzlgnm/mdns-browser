/**
 * Workaround for thaw inputs wrapped by components `AutoComplete` or `Input`
 * providing no possibility to set attributes.
 * We want no spell checking no autocorrection or capitalization in our inputs 
 * mostly dealing with lower case characters.
 */
window.addEventListener("TrunkApplicationStarted", function () {
    const inputs = document.querySelectorAll("input.thaw-input__input");
    for (const input of inputs) {
        input.setAttribute("autocapitalize", "none");
        input.setAttribute("autocorrect", "off");
        input.setAttribute("spellcheck", "false");
    }
});
